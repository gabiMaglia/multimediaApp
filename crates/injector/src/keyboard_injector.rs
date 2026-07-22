use anyhow::Result;
use log::{info, warn, debug};
use windows::Win32::Foundation::{HWND, WPARAM, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    PostMessageW, WM_KEYDOWN, WM_KEYUP,
};

use input_core::{InputEvent, InputEventType};
use crate::event_translator;

/// Injects keyboard events to a target window via PostMessage (no focus needed).
pub struct KeyboardInjector {
    target_hwnd: HWND,
}

impl KeyboardInjector {
    pub fn new(hwnd_raw: u64) -> Self {
        Self {
            target_hwnd: HWND(hwnd_raw as isize as *mut _),
        }
    }

    /// Send a key event to the target window.
    pub fn inject(&self, event: &InputEvent) -> Result<()> {
        let vk_code = event_translator::key_name_to_vk(&event.code);
        if vk_code == 0 {
            debug!("[keyboard_injector] Unknown key code: '{}'", event.code);
            return Ok(());
        }

        let msg = match event.event_type {
            InputEventType::KeyDown => WM_KEYDOWN,
            InputEventType::KeyUp => WM_KEYUP,
            _ => return Ok(()), // Not a keyboard event
        };

        // lParam: repeat count=1, scan code, extended key flag, etc.
        // Simplified: just the virtual key code in WPARAM
        let wparam = WPARAM(vk_code as usize);
        let lparam = LPARAM(0);

        let result = unsafe {
            PostMessageW(Some(self.target_hwnd), msg, wparam, lparam)
        };

        if let Err(e) = result {
            warn!("[keyboard_injector] PostMessage failed: {:?}", e);
        } else {
            debug!(
                "[keyboard_injector] Sent {:?} vk={:#04x} to HWND {:?}",
                event.event_type, vk_code, self.target_hwnd
            );
        }

        Ok(())
    }
}

/// Starts a background thread that reads keyboard InputEvents and injects them
/// to the target window.
pub fn start_keyboard_injector(
    rx: std::sync::mpsc::Receiver<(InputEvent, u64)>,
) {
    std::thread::spawn(move || {
        info!("[keyboard_injector] Injector thread started");

        while let Ok((event, hwnd)) = rx.recv() {
            match event.event_type {
                InputEventType::KeyDown | InputEventType::KeyUp => {
                    let injector = KeyboardInjector::new(hwnd);
                    if let Err(e) = injector.inject(&event) {
                        warn!("[keyboard_injector] Error: {}", e);
                    }
                }
                _ => {} // Not keyboard, skip
            }
        }

        warn!("[keyboard_injector] Channel closed, shutting down");
    });
}
