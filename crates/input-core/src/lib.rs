use serde::{Serialize, Deserialize};

pub mod raw_input_listener;
pub mod device_manager;
pub mod event_router;
pub mod windows_api;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputDeviceType {
    Keyboard,
    Mouse,
    Gamepad,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputEventType {
    KeyDown,
    KeyUp,
    MouseMove,
    MouseButton,
    GamepadButton,
    GamepadAxis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputEvent {
    pub device_id: String,
    pub device_type: InputDeviceType,
    pub event_type: InputEventType,
    pub code: String,
    pub value: f32,
    pub timestamp: u128,
}

use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_time() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}