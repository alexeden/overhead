use crate::tplink::{error::TpError, models::DeviceResponse};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr};

#[derive(Default)]
pub struct AppState {
    /// Maintain a map of device models by their socket address
    pub models: HashMap<SocketAddr, String>,
}

impl AppState {
    pub fn get_model(&self, socket_addr: SocketAddr) -> AppResult<String> {
        self.models
            .get(&socket_addr)
            .ok_or(AppError::NotFound(socket_addr.to_string()))
            .cloned()
    }
}

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Debug, Deserialize, Serialize, specta::Type)]
pub enum AppError {
    /// Tried to send a command to a device that was never discovered
    NotFound(String),
    /// TPLinker error
    Tp(TpError),
}

impl From<TpError> for AppError {
    fn from(err: TpError) -> Self {
        AppError::Tp(err)
    }
}

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
