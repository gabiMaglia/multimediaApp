use tauri::AppHandle;
use tauri::Emitter;

use std::sync::mpsc::channel;
use std::thread;


use input_core::raw_input_listener::start_keyboard_mouse_listener;
use input_core::device_manager::start_gamepad_listener;

use input_core::InputEvent;

pub fn forward_to_frontend(app: AppHandle, event: InputEvent) {

    app.emit("input:event", event).ok();

}

pub fn start_device_system() {

       let (tx, rx) = channel();

    let tx_keyboard = tx.clone();
    let tx_gamepad = tx.clone();

    start_keyboard_mouse_listener(tx_keyboard);
    start_gamepad_listener(tx_gamepad);


    std::thread::spawn(move || {

        while let Ok(event) = rx.recv() {

            println!("DEVICE EVENT {:?}", event);

        }

    });

}