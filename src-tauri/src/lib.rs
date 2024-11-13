use specta_typescript::Typescript;
use std::{net::SocketAddr, time::Duration};
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager,
};
use tauri_plugin_store::StoreExt;
use tplink::{
    devices::Device,
    discover::{discover_devices, DiscoverConfig},
    error::{TpError, TpResult},
    models::DeviceResponse,
    prelude::*,
};
use utils::get_local_ip_addr;
mod discovery;
mod tplink;
mod utils;

#[tauri::command]
#[specta::specta]
fn get_config(state: tauri::State<'_, State>) -> DiscoverConfig {
    state.discover_config
}

#[tauri::command]
#[specta::specta]
fn get_devices(state: tauri::State<'_, State>) -> Vec<(SocketAddr, DeviceResponse)> {
    let devices = discover_devices(state.discover_config);
    devices.unwrap()
}

#[tauri::command]
#[specta::specta]
fn device_command(socket_addr: SocketAddr, device: DeviceResponse) -> TpResult<bool> {
    let mut dev = Device::from_response(socket_addr, &device).ok_or("Device not found")?;
    dev.toggle()
}

#[tauri::command]
#[specta::specta]
fn set_brightness(socket_addr: SocketAddr, device: DeviceResponse, brightness: u8) -> TpResult<()> {
    let mut dev = Device::from_response(socket_addr, &device).ok_or("Device not found")?;
    dev.as_dimmable()
        .ok_or(TpError::Other("Not dimmable".to_string()))
        .and_then(|d| d.set_brightness(brightness))
}

#[derive(Copy, Clone, Debug, serde::Serialize, specta::Type)]
struct State {
    discover_config: DiscoverConfig,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // // specta_typescript::export(conf)
    // #[cfg(debug_assertions)]
    // specta::export::ts_with_cfg(
    //     "../src/types.ts",
    //     Cow::Borrowed("/* eslint-disable */\n"),
    //     &ExportConfig::default().formatter(specta::ts::formatter::prettier),
    // )
    // .expect("Failed to write types.ts");
    // export::export::ts_with_cfg(
    //     "../src/types.ts",
    //     Cow::Borrowed("/* eslint-disable */\n"),
    //     &ExportConfig::default().formatter(specta::ts::formatter::prettier),
    // )
    // .expect("Failed to write types.ts");

    // let mut tauri_builder = tauri_specta::Builder::<tauri::Wry>::new()
    //     // Then register them (separated by a comma)
    //     .commands(tauri_specta::collect_commands![]);

    // #[cfg(debug_assertions)] // <- Only export on non-release builds
    // tauri_builder
    //     .export(Typescript::default(), "../src/bindings.ts")
    //     .expect("Failed to export typescript bindings");

    let specta_builder = tauri_specta::Builder::<tauri::Wry>::new()
        .commands(tauri_specta::collect_commands![
            device_command,
            get_config,
            get_devices,
            set_brightness
        ])
        .events(tauri_specta::collect_events![]);

    #[cfg(debug_assertions)] // <- Only export on non-release builds
    specta_builder
        .export(
            Typescript::new()
                .bigint(specta_typescript::BigIntExportBehavior::Number)
                .formatter(specta_typescript::formatter::prettier)
                .header("// @ts-nocheck"),
            "../src/bindings.ts",
        )
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level_for("tplink", log::LevelFilter::Debug)
                .level(log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(specta_builder.invoke_handler())
        .invoke_handler(tauri::generate_handler![
            device_command,
            get_config,
            get_devices,
            set_brightness
        ])
        .setup(move |app| {
            specta_builder.mount_events(app);
            let ip = get_local_ip_addr("en0").expect("Failed to get local IP address from en0");

            app.manage(State {
                discover_config: DiscoverConfig::from_ip(ip)
                    .set_listen_timeout(Duration::from_secs(2)),
            });

            // Store
            let _store = app.store("store.json")?;

            // System tray
            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&Menu::with_items(
                    app,
                    &[&MenuItem::with_id(app, "quit", "Test", true, None::<&str>)?],
                )?)
                .menu_on_left_click(true)
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
