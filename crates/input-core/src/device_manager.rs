use gilrs::{Gilrs, Event as GilrsEvent, EventType};
use crate::{InputEvent, InputDeviceType, InputEventType};
use crate::current_time;
use log::{info, warn};

/// Starts a background thread that listens for gamepad events from all connected controllers.
/// Supports PS4, Xbox, generic HID gamepads — anything gilrs recognizes.
pub fn start_gamepad_listener(sender: std::sync::mpsc::Sender<InputEvent>) {
    std::thread::spawn(move || {
        let mut gilrs = match Gilrs::new() {
            Ok(g) => g,
            Err(e) => {
                log::error!("[device_manager] Failed to initialize gilrs: {}", e);
                return;
            }
        };

        // Log all connected gamepads at startup
        for (_id, gamepad) in gilrs.gamepads() {
            info!(
                "[device_manager] Found gamepad: '{}' (id: {:?}, power: {:?})",
                gamepad.name(),
                _id,
                gamepad.power_info()
            );
        }

        loop {
            while let Some(GilrsEvent { id, event, .. }) = gilrs.next_event() {
                let gamepad = gilrs.gamepad(id);
                let device_id = format!("gamepad-{}-{}", gamepad.name(), id);

                let (event_type, code, value) = match event {
                    EventType::ButtonPressed(btn, _) => (
                        InputEventType::GamepadButton,
                        format!("{:?}", btn),
                        1.0,
                    ),
                    EventType::ButtonReleased(btn, _) => (
                        InputEventType::GamepadButton,
                        format!("{:?}", btn),
                        0.0,
                    ),
                    EventType::AxisChanged(axis, val, _) => (
                        InputEventType::GamepadAxis,
                        format!("{:?}", axis),
                        val,
                    ),
                    EventType::Connected => {
                        info!("[device_manager] Gamepad connected: {} (id: {:?})", gamepad.name(), id);
                        continue;
                    }
                    EventType::Disconnected => {
                        warn!("[device_manager] Gamepad disconnected: {} (id: {:?})", gamepad.name(), id);
                        continue;
                    }
                    _ => continue,
                };

                let input_event = InputEvent {
                    device_id,
                    device_type: InputDeviceType::Gamepad,
                    event_type,
                    code,
                    value,
                    timestamp: current_time(),
                };

                sender.send(input_event).ok();
            }
        }
    });
}