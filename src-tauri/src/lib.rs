use crate::tplink::{devices::Device as TpLinkDevice, discover::discover_devices, prelude::*};
use app::{AppResult, AppState, Device};
use std::{net::SocketAddr, sync::Mutex};
use tauri::{Manager, State};
use tauri_plugin_store::StoreExt;
mod app;
mod tplink;

#[tauri::command]
#[specta::specta]
async fn discover(state: State<'_, Mutex<AppState>>) -> AppResult<Vec<Device>> {
    let mut state = state.lock().unwrap();

    discover_devices()
        .map(|resps| {
            resps
                .into_iter()
                .map(|(addr, resp)| {
                    let model = resp.sysinfo().model.clone();
                    state.models.insert(addr, model);
                    (addr, resp).into()
                })
                .collect()
        })
        .map_err(|err| err.into())
}

#[tauri::command]
#[specta::specta]
fn set_brightness(
    socket_addr: SocketAddr,
    brightness: u8,
    state: State<'_, Mutex<AppState>>,
) -> AppResult<()> {
    let state = state.lock().unwrap();
    let model = state.get_model(socket_addr)?;

    // let device = TpLinkDevice::try_new(socket_addr, &model)?;
    // match device {
    //     TpLinkDevice::KL135(d) | TpLinkDevice::HS220(d) => d.set_brightness(brightness),
    //     TpLinkDevice::HS220(d) => d.set_brightness(brightness),
    // }
    TpLinkDevice::try_new(socket_addr, &model)?
        .try_into_dimmable()
        .and_then(|d| d.set_brightness(brightness))
        .map_err(|err| err.into())
}

#[tauri::command]
#[specta::specta]
fn toggle(socket_addr: SocketAddr, state: State<'_, Mutex<AppState>>) -> AppResult<bool> {
    let state = state.lock().unwrap();
    let model = state.get_model(socket_addr)?;
    TpLinkDevice::try_new(socket_addr, &model)?
        .toggle()
        .map_err(|err| err.into())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let specta_builder = tauri_specta::Builder::<tauri::Wry>::new()
        .commands(tauri_specta::collect_commands![
            discover,
            set_brightness,
            toggle
        ])
        .events(tauri_specta::collect_events![]);

    #[cfg(debug_assertions)]
    specta_builder
        .export(
            specta_typescript::Typescript::new()
                .bigint(specta_typescript::BigIntExportBehavior::Number)
                .formatter(specta_typescript::formatter::prettier)
                .header("// @ts-nocheck"),
            "../src/bindings.ts",
        )
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Debug)
                .build(),
        )
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(specta_builder.invoke_handler())
        .invoke_handler(tauri::generate_handler![discover, set_brightness, toggle])
        .setup(move |app| {
            specta_builder.mount_events(app);

            app.manage(Mutex::new(AppState::default()));

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
                    .build(app)?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
