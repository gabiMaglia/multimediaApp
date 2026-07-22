use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowW, GetForegroundWindow, GetWindowTextW,
};
use anyhow::Result;
use log::debug;

/// Find a window by its title (exact match). Returns the HWND as u64.
pub fn find_window_by_title(title: &str) -> Result<u64> {
    let wide_title: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
    let hwnd = unsafe {
        FindWindowW(None, windows::core::PCWSTR(wide_title.as_ptr()))
    }?;
    if hwnd == HWND::default() {
        anyhow::bail!("Window not found: '{}'", title);
    }
    debug!("[windows_api] Found window '{}' -> HWND {:?}", title, hwnd);
    Ok(hwnd.0 as isize as u64)
}

/// Get the HWND of the current foreground window.
pub fn get_foreground_window() -> u64 {
    let hwnd = unsafe { GetForegroundWindow() };
    hwnd.0 as isize as u64
}

/// Get the title of a window given its HWND.
pub fn get_window_title(hwnd_raw: u64) -> Result<String> {
    let hwnd = HWND(hwnd_raw as isize as *mut _);
    let mut buf = [0u16; 512];
    let len = unsafe { GetWindowTextW(hwnd, &mut buf) };
    if len == 0 {
        anyhow::bail!("Could not get window title for HWND {}", hwnd_raw);
    }
    Ok(String::from_utf16_lossy(&buf[..len as usize]))
}
