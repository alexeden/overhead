use log::LevelFilter;
use network_interface::{Addr, NetworkInterface, NetworkInterfaceConfig, V4IfAddr};
mod discovery;
use tplink::discover::DiscoverConfig;
mod tplink;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_config(state: tauri::State<'_, State>) -> DiscoverConfig {
    state.discover_config
}

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct State {
    discover_config: DiscoverConfig,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let netif = NetworkInterface::show()
        .ok()
        .and_then(|interfaces| interfaces.into_iter().find(|netif| netif.name == "en0"))
        .expect("No network interface named en0 found");

    let ip = netif
        .addr
        .into_iter()
        .find_map(|addr| match addr {
            Addr::V4(V4IfAddr {
                ip,
                broadcast: Some(_),
                netmask: _,
            }) => Some(ip),
            _ => None,
        })
        .expect("No IP address found");

    let discover_config = DiscoverConfig::from_ip(ip);
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_shell::init())
        .manage(State { discover_config })
        .invoke_handler(tauri::generate_handler![greet, get_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
