use std::sync::mpsc::{Receiver, Sender};
use log::{info, warn, debug};
use serde::{Serialize, Deserialize};

use crate::{InputEvent, InputDeviceType};

/// Where a routed event should be sent.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RouteTarget {
    /// Inject as virtual gamepad via ViGEm (for Steam, etc.)
    VirtualGamepad,
    /// Send to a specific window handle via PostMessage/SendMessage
    WindowHandle(u64),
    /// Drop the event
    Discard,
}

/// A single routing rule: matches a device_id (or device type) to a target.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingRule {
    /// If set, matches exact device_id
    pub device_id: Option<String>,
    /// If set, matches device type (fallback if device_id is None)
    pub device_type: Option<InputDeviceType>,
    /// Where to route matching events
    pub target: RouteTarget,
}

/// The routing profile containing all active rules, evaluated in order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingProfile {
    pub name: String,
    pub rules: Vec<RoutingRule>,
    /// Default target when no rule matches
    pub default_target: RouteTarget,
}

impl RoutingProfile {
    /// Evaluate routing rules for a given event. First match wins.
    pub fn resolve(&self, event: &InputEvent) -> &RouteTarget {
        for rule in &self.rules {
            // Match by device_id first (most specific)
            if let Some(ref rule_id) = rule.device_id {
                if rule_id == &event.device_id {
                    debug!(
                        "[event_router] Rule matched device_id='{}' -> {:?}",
                        event.device_id, rule.target
                    );
                    return &rule.target;
                }
            }
            // Match by device_type (less specific)
            if let Some(ref rule_type) = rule.device_type {
                if std::mem::discriminant(rule_type) == std::mem::discriminant(&event.device_type) {
                    debug!(
                        "[event_router] Rule matched device_type={:?} -> {:?}",
                        event.device_type, rule.target
                    );
                    return &rule.target;
                }
            }
        }

        debug!(
            "[event_router] No rule matched for device_id='{}', using default -> {:?}",
            event.device_id, self.default_target
        );
        &self.default_target
    }
}

/// Channels for sending routed events to the appropriate injectors.
pub struct RouterOutputs {
    pub gamepad_tx: Sender<InputEvent>,
    pub window_tx: Sender<(InputEvent, u64)>,
}

/// Starts the event router loop. Reads events from `input_rx`, evaluates routing
/// rules from `profile`, and forwards to the correct output channel.
pub fn start_event_router(
    input_rx: Receiver<InputEvent>,
    profile: RoutingProfile,
    outputs: RouterOutputs,
) {
    std::thread::spawn(move || {
        info!("[event_router] Started with profile '{}'", profile.name);

        while let Ok(event) = input_rx.recv() {
            let target = profile.resolve(&event);

            match target {
                RouteTarget::VirtualGamepad => {
                    if let Err(e) = outputs.gamepad_tx.send(event.clone()) {
                        warn!("[event_router] Failed to send to gamepad injector: {}", e);
                    }
                }
                RouteTarget::WindowHandle(hwnd) => {
                    if let Err(e) = outputs.window_tx.send((event.clone(), *hwnd)) {
                        warn!("[event_router] Failed to send to window injector: {}", e);
                    }
                }
                RouteTarget::Discard => {
                    debug!("[event_router] Discarded event from '{}'", event.device_id);
                }
            }
        }

        warn!("[event_router] Input channel closed, router shutting down");
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{InputEventType, current_time};

    fn make_event(device_id: &str, device_type: InputDeviceType) -> InputEvent {
        InputEvent {
            device_id: device_id.into(),
            device_type,
            event_type: InputEventType::GamepadButton,
            code: "South".into(),
            value: 1.0,
            timestamp: current_time(),
        }
    }

    fn make_profile() -> RoutingProfile {
        RoutingProfile {
            name: "test".into(),
            rules: vec![
                RoutingRule {
                    device_id: Some("gamepad-PS4-0".into()),
                    device_type: None,
                    target: RouteTarget::VirtualGamepad,
                },
                RoutingRule {
                    device_id: None,
                    device_type: Some(InputDeviceType::Keyboard),
                    target: RouteTarget::WindowHandle(12345),
                },
                RoutingRule {
                    device_id: None,
                    device_type: Some(InputDeviceType::Mouse),
                    target: RouteTarget::WindowHandle(12345),
                },
            ],
            default_target: RouteTarget::Discard,
        }
    }

    #[test]
    fn test_gamepad_routes_to_vigem() {
        let profile = make_profile();
        let event = make_event("gamepad-PS4-0", InputDeviceType::Gamepad);
        assert_eq!(profile.resolve(&event), &RouteTarget::VirtualGamepad);
    }

    #[test]
    fn test_keyboard_routes_to_window() {
        let profile = make_profile();
        let event = make_event("keyboard-0", InputDeviceType::Keyboard);
        assert_eq!(profile.resolve(&event), &RouteTarget::WindowHandle(12345));
    }

    #[test]
    fn test_mouse_routes_to_window() {
        let profile = make_profile();
        let event = make_event("mouse-0", InputDeviceType::Mouse);
        assert_eq!(profile.resolve(&event), &RouteTarget::WindowHandle(12345));
    }

    #[test]
    fn test_unknown_device_uses_default() {
        let profile = make_profile();
        let event = make_event("unknown-device-99", InputDeviceType::Gamepad);
        assert_eq!(profile.resolve(&event), &RouteTarget::Discard);
    }

    #[test]
    fn test_device_id_takes_priority_over_type() {
        let profile = RoutingProfile {
            name: "priority-test".into(),
            rules: vec![
                RoutingRule {
                    device_id: Some("keyboard-0".into()),
                    device_type: None,
                    target: RouteTarget::Discard,
                },
                RoutingRule {
                    device_id: None,
                    device_type: Some(InputDeviceType::Keyboard),
                    target: RouteTarget::WindowHandle(999),
                },
            ],
            default_target: RouteTarget::VirtualGamepad,
        };
        // keyboard-0 should match the first rule (by device_id), not the second (by type)
        let event = make_event("keyboard-0", InputDeviceType::Keyboard);
        assert_eq!(profile.resolve(&event), &RouteTarget::Discard);
    }
}