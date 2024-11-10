use std::{net::SocketAddr, time::Duration};
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager,
};
mod discovery;
use tplink::{
    discover::{discover_devices, DiscoverConfig},
    models::DeviceData,
};
use utils::get_local_ip_addr;
mod tplink;
mod utils;

#[tauri::command]
fn get_config(state: tauri::State<'_, State>) -> DiscoverConfig {
    state.discover_config
}

#[tauri::command]
fn get_devices(state: tauri::State<'_, State>) -> Vec<(SocketAddr, DeviceData)> {
    let devices = discover_devices(state.discover_config);
    devices.unwrap()
}

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct State {
    discover_config: DiscoverConfig,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level_for("tplink", log::LevelFilter::Debug)
                .level(log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_config, get_devices])
        .setup(|app| {
            let ip = get_local_ip_addr("en0").expect("Failed to get local IP address from en0");

            app.manage(State {
                discover_config: DiscoverConfig::from_ip(ip)
                    .set_listen_timeout(Duration::from_secs(5)),
            });

            let quit_i = MenuItem::with_id(app, "quit", "Oh Fuck", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;
            let tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .menu_on_left_click(true)
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
