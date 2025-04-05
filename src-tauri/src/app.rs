use crate::tplink::{
    error::TpError,
    models::{DeviceResponse, LightState},
};
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

#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub addr: SocketAddr,
    pub brightness: u8,
    pub hw_type: String,
    pub id: String,
    pub is_on: bool,
    pub light_state: Option<LightState>,
    pub model: String,
    pub name: String,
}

impl From<(SocketAddr, DeviceResponse)> for Device {
    fn from((addr, resp): (SocketAddr, DeviceResponse)) -> Self {
        Self {
            addr,
            brightness: resp.sysinfo().brightness(),
            hw_type: resp.sysinfo().hw_type.clone(),
            id: resp.sysinfo().device_id.clone(),
            is_on: resp.sysinfo().is_on(),
            light_state: resp.sysinfo().light_state.clone(),
            model: resp.sysinfo().model.clone(),
            name: resp.sysinfo().alias.clone(),
        }
    }
}
