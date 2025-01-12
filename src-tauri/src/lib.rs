use serde::Serialize;
use specta_typescript::Typescript;
use std::net::SocketAddr;
use tauri::ipc::Channel;
use tauri_plugin_store::StoreExt;
use tplink::{
    devices::Device,
    discover::discover_devices,
    error::{TpError, TpResult},
    models::{DeviceResponse, SysInfo},
    prelude::*,
};
mod tplink;

#[derive(Debug, Clone, PartialEq, Serialize, Eq, specta::Type)]
pub enum DiscoverEvent {
    Start,
    Progress(u64),
    End,
    Error(String),
}

#[tauri::command]
#[specta::specta]
fn discover(on_event: Channel<DiscoverEvent>) -> TpResult<()> {
    // let devices = discover_devices();
    // devices.unwrap()
    on_event.send(DiscoverEvent::Start).unwrap();
    // Wait for 5 seconds
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(5));
        on_event.send(DiscoverEvent::End).unwrap();
    });
    Ok(())
}

#[tauri::command]
#[specta::specta]
fn get_devices() -> Vec<(SocketAddr, DeviceResponse)> {
    let devices = discover_devices();
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
fn set_brightness(
    socket_addr: SocketAddr,
    device: DeviceResponse,
    brightness: u8,
) -> TpResult<SysInfo> {
    let mut dev = Device::from_response(socket_addr, &device).ok_or("Device not found")?;
    dev.as_dimmable()
        .ok_or(TpError::Other("Not dimmable".to_string()))
        .and_then(|d| d.set_brightness(brightness).map(|_| d))
        .and_then(|d| d.get_sysinfo())
    // .and_then(|d| d.g)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let specta_builder = tauri_specta::Builder::<tauri::Wry>::new()
        .commands(tauri_specta::collect_commands![
            discover,
            device_command,
            get_devices,
            set_brightness
        ])
        .events(tauri_specta::collect_events![]);

    #[cfg(debug_assertions)]
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
                .level(log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(specta_builder.invoke_handler())
        .invoke_handler(tauri::generate_handler![
            discover,
            device_command,
            get_devices,
            set_brightness
        ])
        .setup(move |app| {
            specta_builder.mount_events(app);

            let _store = app.store("store.json")?;

            #[cfg(desktop)]
            {
                use tauri::menu::{Menu, MenuItem};

                // System tray
                tauri::tray::TrayIconBuilder::new()
                    .icon(app.default_window_icon().unwrap().clone())
                    .menu(&Menu::with_items(
                        app,
                        &[&MenuItem::with_id(app, "quit", "Test", true, None::<&str>)?],
                    )?)
                    .menu_on_left_click(true)
                    .build(app)?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
