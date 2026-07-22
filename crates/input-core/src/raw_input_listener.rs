use rdev::{listen, Event, EventType};
use std::sync::mpsc::Sender;
use log::{info, error};

use crate::{InputEvent, InputDeviceType, InputEventType};

/// Starts a background thread capturing keyboard and mouse events via OS-level hooks.
pub fn start_keyboard_mouse_listener(sender: Sender<InputEvent>) {
    std::thread::spawn(move || {
        info!("[raw_input_listener] Starting keyboard/mouse listener");

        let callback = move |event: Event| {
            if let Some(e) = convert_event(event) {
                sender.send(e).ok();
            }
        };

        if let Err(e) = listen(callback) {
            error!("[raw_input_listener] Listener error: {:?}", e);
        }
    });
}

fn convert_event(event: Event) -> Option<InputEvent> {
    let timestamp = event.time
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?
        .as_millis();

    match event.event_type {
        EventType::KeyPress(key) => Some(InputEvent {
            device_id: "keyboard-0".into(),
            device_type: InputDeviceType::Keyboard,
            event_type: InputEventType::KeyDown,
            code: format!("{:?}", key),
            value: 1.0,
            timestamp,
        }),

        EventType::KeyRelease(key) => Some(InputEvent {
            device_id: "keyboard-0".into(),
            device_type: InputDeviceType::Keyboard,
            event_type: InputEventType::KeyUp,
            code: format!("{:?}", key),
            value: 0.0,
            timestamp,
        }),

        EventType::ButtonPress(button) => Some(InputEvent {
            device_id: "mouse-0".into(),
            device_type: InputDeviceType::Mouse,
            event_type: InputEventType::MouseButton,
            code: format!("{:?}", button),
            value: 1.0,
            timestamp,
        }),

        EventType::ButtonRelease(button) => Some(InputEvent {
            device_id: "mouse-0".into(),
            device_type: InputDeviceType::Mouse,
            event_type: InputEventType::MouseButton,
            code: format!("{:?}", button),
            value: 0.0,
            timestamp,
        }),

        EventType::MouseMove { x, y } => Some(InputEvent {
            device_id: "mouse-0".into(),
            device_type: InputDeviceType::Mouse,
            event_type: InputEventType::MouseMove,
            code: format!("{}:{}", x, y),
            value: 0.0,
            timestamp,
        }),

        _ => None,
    }
}