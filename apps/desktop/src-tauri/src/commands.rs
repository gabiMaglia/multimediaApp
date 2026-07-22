use tauri::command;

#[command]
pub async fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[command]
pub fn list_devices() -> Vec<String> {
    vec!["Keyboard 1".into(), "Mouse 1".into()]
}

#[command]
pub fn start_routing() -> bool {
    true
}