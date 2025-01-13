use crate::tplink::models::DeviceResponse;
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Debug, Clone, PartialEq, Serialize, Eq, specta::Type)]
pub enum DiscoverEvent {
    Start,
    // Progress(u64),
    End,
    // Error(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Eq, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub addr: SocketAddr,
    pub id: String,
    pub model: String,
    pub name: String,
    pub brightness: Option<u8>,
    pub is_on: bool,
}

impl From<(SocketAddr, DeviceResponse)> for Device {
    fn from((addr, resp): (SocketAddr, DeviceResponse)) -> Self {
        Self {
            addr,
            brightness: resp.sysinfo().brightness,
            id: resp.sysinfo().device_id.clone(),
            is_on: resp.sysinfo().is_on(),
            model: resp.sysinfo().model.clone(),
            name: resp.sysinfo().alias.clone(),
        }
    }
}
