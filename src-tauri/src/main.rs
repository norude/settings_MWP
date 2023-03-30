// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod set_settings;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn set_setting(path: String, value_name: String, value: String) {
    println!("set_setting: {}, {}:{}", path, value_name, value);
    set_settings::set_registry_value(&path, &value_name, &value).unwrap()
}
#[tauri::command]
fn get_setting(path: String, value_name: String) -> String {
    println!("get_setting: {}, {}", path, value_name);
    match set_settings::get_registry_value(&path, &value_name) {
        Ok(x) => x,
        _ => "".into(),
    }
}
#[tauri::command]
fn set_brightness(path: String, value_name: String, value: String) {
    println!("set_brightness: {}, {}:{}", path, value_name, value);
    set_settings::set_brightness(value.parse().unwrap());
}
#[tauri::command]
fn get_brightness(path: String, value_name: String) -> String {
    println!("get_brightness: {}, {}", path, value_name);
    match set_settings::get_brightness() {
        Ok(x) => x,
        _ => "".into(),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            set_setting,
            get_setting,
            set_brightness,
            get_brightness
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
