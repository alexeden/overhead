use log::LevelFilter;
use network_interface::{NetworkInterface, NetworkInterfaceConfig};
mod discovery;
mod tplink;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_devices() -> String {
    format!("Hello! You've been greeted from Rust!")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let netif = NetworkInterface::show()
        .map(|interfaces| interfaces.into_iter().find(|netif| netif.name == "en0"))
        .expect("No network interface named en0 found");

    println!("{:#?}", netif);

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
