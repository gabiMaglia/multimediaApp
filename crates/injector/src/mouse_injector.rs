use anyhow::Result;
use log::{info, warn, debug};
use windows::Win32::Foundation::{HWND, WPARAM, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    PostMessageW, WM_MOUSEMOVE, WM_LBUTTONDOWN, WM_LBUTTONUP,
    WM_RBUTTONDOWN, WM_RBUTTONUP,
};

use input_core::{InputEvent, InputEventType};

/// Injects mouse events to a target window via PostMessage (no focus needed).
pub struct MouseInjector {
    target_hwnd: HWND,
}

impl MouseInjector {
    pub fn new(hwnd_raw: u64) -> Self {
        Self {
            target_hwnd: HWND(hwnd_raw as isize as *mut _),
        }
    }

    /// Send a mouse event to the target window.
    pub fn inject(&self, event: &InputEvent) -> Result<()> {
        match event.event_type {
            InputEventType::MouseMove => self.inject_mouse_move(event),
            InputEventType::MouseButton => self.inject_mouse_button(event),
            _ => Ok(()), // Not a mouse event
        }
    }

    fn inject_mouse_move(&self, event: &InputEvent) -> Result<()> {
        // code format is "x:y"
        let parts: Vec<&str> = event.code.split(':').collect();
        if parts.len() != 2 {
            return Ok(());
        }

        let x: i32 = parts[0].parse().unwrap_or(0);
        let y: i32 = parts[1].parse().unwrap_or(0);

        // LPARAM encodes position: low word = x, high word = y
        let lparam = LPARAM(((y as i32 & 0xFFFF) << 16 | (x as i32 & 0xFFFF)) as isize);

        let result = unsafe {
            PostMessageW(Some(self.target_hwnd), WM_MOUSEMOVE, WPARAM(0), lparam)
        };

        if let Err(e) = result {
            warn!("[mouse_injector] PostMessage MouseMove failed: {:?}", e);
        } else {
            debug!("[mouse_injector] Sent MouseMove ({},{}) to HWND {:?}", x, y, self.target_hwnd);
        }

        Ok(())
    }

    fn inject_mouse_button(&self, event: &InputEvent) -> Result<()> {
        let (msg_down, msg_up) = if event.code.contains("Left") {
            (WM_LBUTTONDOWN, WM_LBUTTONUP)
        } else if event.code.contains("Right") {
            (WM_RBUTTONDOWN, WM_RBUTTONUP)
        } else {
            debug!("[mouse_injector] Unsupported mouse button: '{}'", event.code);
            return Ok(());
        };

        let msg = if event.value > 0.5 { msg_down } else { msg_up };

        let result = unsafe {
            PostMessageW(Some(self.target_hwnd), msg, WPARAM(0), LPARAM(0))
        };

        if let Err(e) = result {
            warn!("[mouse_injector] PostMessage MouseButton failed: {:?}", e);
        } else {
            debug!(
                "[mouse_injector] Sent {:?} button={} to HWND {:?}",
                if event.value > 0.5 { "Down" } else { "Up" },
                event.code,
                self.target_hwnd
            );
        }

        Ok(())
    }
}

/// Starts a background thread that reads mouse InputEvents and injects them
/// to the target window.
pub fn start_mouse_injector(
    rx: std::sync::mpsc::Receiver<(InputEvent, u64)>,
) {
    std::thread::spawn(move || {
        info!("[mouse_injector] Injector thread started");

        while let Ok((event, hwnd)) = rx.recv() {
            match event.event_type {
                InputEventType::MouseMove | InputEventType::MouseButton => {
                    let injector = MouseInjector::new(hwnd);
                    if let Err(e) = injector.inject(&event) {
                        warn!("[mouse_injector] Error: {}", e);
                    }
                }
                _ => {} // Not mouse, skip
            }
        }

        warn!("[mouse_injector] Channel closed, shutting down");
    });
}
