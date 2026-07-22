use vigem_client::{XGamepad, XButtons};
use input_core::{InputEvent, InputEventType};
use log::debug;

/// Apply a gamepad InputEvent to the ViGEm XGamepad state struct.
/// Maps button names from gilrs format to Xbox 360 button flags.
pub fn apply_gamepad_event(state: &mut XGamepad, event: &InputEvent) {
    match event.event_type {
        InputEventType::GamepadButton => {
            let pressed = event.value > 0.5;
            if let Some(button) = map_button_name(&event.code) {
                if pressed {
                    state.buttons.raw |= button;
                } else {
                    state.buttons.raw &= !button;
                }
                debug!(
                    "[event_translator] Button '{}' {} -> buttons={:#06x}",
                    event.code,
                    if pressed { "pressed" } else { "released" },
                    state.buttons.raw
                );
            }
        }
        InputEventType::GamepadAxis => {
            apply_axis_event(state, &event.code, event.value);
        }
        _ => {}
    }
}

/// Map gilrs button names to Xbox 360 button bit flags.
/// Supports PS4, Xbox, and generic controller naming.
fn map_button_name(name: &str) -> Option<u16> {
    match name {
        // Face buttons (PS4: Cross/Circle/Square/Triangle, Xbox: A/B/X/Y)
        "South" => Some(XButtons::A),
        "East" => Some(XButtons::B),
        "West" => Some(XButtons::X),
        "North" => Some(XButtons::Y),

        // Shoulder buttons
        "LeftTrigger" | "L1" => Some(XButtons::LB),
        "RightTrigger" | "R1" => Some(XButtons::RB),

        // Stick clicks
        "LeftThumb" | "L3" => Some(XButtons::LTHUMB),
        "RightThumb" | "R3" => Some(XButtons::RTHUMB),

        // D-pad
        "DPadUp" => Some(XButtons::UP),
        "DPadDown" => Some(XButtons::DOWN),
        "DPadLeft" => Some(XButtons::LEFT),
        "DPadRight" => Some(XButtons::RIGHT),

        // Start / Select (PS4: Options/Share)
        "Start" | "Options" => Some(XButtons::START),
        "Select" | "Share" | "Back" => Some(XButtons::BACK),

        // Guide/Home button
        "Mode" | "Guide" => Some(XButtons::GUIDE),

        _ => {
            debug!("[event_translator] Unmapped button: '{}'", name);
            None
        }
    }
}

/// Map axis events to the XGamepad struct fields.
/// gilrs provides axis values in -1.0..1.0 range, Xbox 360 uses i16 for sticks
/// and u8 for triggers.
fn apply_axis_event(state: &mut XGamepad, axis_name: &str, value: f32) {
    match axis_name {
        "LeftStickX" => {
            state.thumb_lx = (value * i16::MAX as f32) as i16;
        }
        "LeftStickY" => {
            state.thumb_ly = (value * i16::MAX as f32) as i16;
        }
        "RightStickX" => {
            state.thumb_rx = (value * i16::MAX as f32) as i16;
        }
        "RightStickY" => {
            state.thumb_ry = (value * i16::MAX as f32) as i16;
        }
        // Triggers: gilrs gives 0.0..1.0, Xbox 360 uses 0..255
        "LeftTrigger2" | "L2" => {
            state.left_trigger = (value.clamp(0.0, 1.0) * 255.0) as u8;
        }
        "RightTrigger2" | "R2" => {
            state.right_trigger = (value.clamp(0.0, 1.0) * 255.0) as u8;
        }
        _ => {
            debug!("[event_translator] Unmapped axis: '{}'", axis_name);
        }
    }
}

/// Map rdev key name strings to Windows Virtual Key codes.
/// Returns 0 if unknown.
pub fn key_name_to_vk(key_name: &str) -> u16 {
    match key_name {
        // Letters
        "KeyA" => 0x41, "KeyB" => 0x42, "KeyC" => 0x43, "KeyD" => 0x44,
        "KeyE" => 0x45, "KeyF" => 0x46, "KeyG" => 0x47, "KeyH" => 0x48,
        "KeyI" => 0x49, "KeyJ" => 0x4A, "KeyK" => 0x4B, "KeyL" => 0x4C,
        "KeyM" => 0x4D, "KeyN" => 0x4E, "KeyO" => 0x4F, "KeyP" => 0x50,
        "KeyQ" => 0x51, "KeyR" => 0x52, "KeyS" => 0x53, "KeyT" => 0x54,
        "KeyU" => 0x55, "KeyV" => 0x56, "KeyW" => 0x57, "KeyX" => 0x58,
        "KeyY" => 0x59, "KeyZ" => 0x5A,

        // Numbers
        "Num0" | "Key0" => 0x30, "Num1" | "Key1" => 0x31,
        "Num2" | "Key2" => 0x32, "Num3" | "Key3" => 0x33,
        "Num4" | "Key4" => 0x34, "Num5" | "Key5" => 0x35,
        "Num6" | "Key6" => 0x36, "Num7" | "Key7" => 0x37,
        "Num8" | "Key8" => 0x38, "Num9" | "Key9" => 0x39,

        // Function keys
        "F1" => 0x70, "F2" => 0x71, "F3" => 0x72, "F4" => 0x73,
        "F5" => 0x74, "F6" => 0x75, "F7" => 0x76, "F8" => 0x77,
        "F9" => 0x78, "F10" => 0x79, "F11" => 0x7A, "F12" => 0x7B,

        // Modifiers
        "ShiftLeft" | "LShift" => 0xA0,
        "ShiftRight" | "RShift" => 0xA1,
        "ControlLeft" | "LControl" => 0xA2,
        "ControlRight" | "RControl" => 0xA3,
        "Alt" | "AltLeft" | "LAlt" => 0xA4,
        "AltGr" | "AltRight" | "RAlt" => 0xA5,

        // Special
        "Space" => 0x20,
        "Return" | "Enter" => 0x0D,
        "Escape" => 0x1B,
        "Backspace" | "BackSpace" => 0x08,
        "Tab" => 0x09,
        "CapsLock" => 0x14,
        "Delete" => 0x2E,
        "Insert" => 0x2D,
        "Home" => 0x24,
        "End" => 0x23,
        "PageUp" => 0x21,
        "PageDown" => 0x22,

        // Arrows
        "UpArrow" | "Up" => 0x26,
        "DownArrow" | "Down" => 0x28,
        "LeftArrow" | "Left" => 0x25,
        "RightArrow" | "Right" => 0x27,

        _ => {
            debug!("[event_translator] Unmapped key: '{}'", key_name);
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use input_core::{InputDeviceType, current_time};

    fn make_gamepad_button(code: &str, value: f32) -> InputEvent {
        InputEvent {
            device_id: "gamepad-PS4-0".into(),
            device_type: InputDeviceType::Gamepad,
            event_type: InputEventType::GamepadButton,
            code: code.into(),
            value,
            timestamp: current_time(),
        }
    }

    fn make_gamepad_axis(code: &str, value: f32) -> InputEvent {
        InputEvent {
            device_id: "gamepad-PS4-0".into(),
            device_type: InputDeviceType::Gamepad,
            event_type: InputEventType::GamepadAxis,
            code: code.into(),
            value,
            timestamp: current_time(),
        }
    }

    #[test]
    fn test_button_south_maps_to_a() {
        let mut state = XGamepad::default();
        let event = make_gamepad_button("South", 1.0);
        apply_gamepad_event(&mut state, &event);
        assert_ne!(state.buttons.raw & XButtons::A, 0);
    }

    #[test]
    fn test_button_release_clears_flag() {
        let mut state = XGamepad::default();
        // Press
        apply_gamepad_event(&mut state, &make_gamepad_button("South", 1.0));
        assert_ne!(state.buttons.raw & XButtons::A, 0);
        // Release
        apply_gamepad_event(&mut state, &make_gamepad_button("South", 0.0));
        assert_eq!(state.buttons.raw & XButtons::A, 0);
    }

    #[test]
    fn test_left_stick_x_maps_correctly() {
        let mut state = XGamepad::default();
        let event = make_gamepad_axis("LeftStickX", 0.5);
        apply_gamepad_event(&mut state, &event);
        assert!(state.thumb_lx > 0);
    }

    #[test]
    fn test_trigger_maps_to_u8() {
        let mut state = XGamepad::default();
        let event = make_gamepad_axis("LeftTrigger2", 1.0);
        apply_gamepad_event(&mut state, &event);
        assert_eq!(state.left_trigger, 255);
    }

    #[test]
    fn test_key_name_to_vk_common_keys() {
        assert_eq!(key_name_to_vk("Space"), 0x20);
        assert_eq!(key_name_to_vk("KeyA"), 0x41);
        assert_eq!(key_name_to_vk("Escape"), 0x1B);
        assert_eq!(key_name_to_vk("F1"), 0x70);
        assert_eq!(key_name_to_vk("unknown_key"), 0);
    }
}
