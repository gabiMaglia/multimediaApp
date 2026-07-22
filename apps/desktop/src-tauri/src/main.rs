#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod services;

fn main() {

    services::device_service::start_device_system();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::list_devices,
            commands::start_routing,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}