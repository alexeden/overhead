use serde::{Deserialize, Serialize};

type ErrCode = i16;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeviceResponse {
    pub system: System,
}

impl DeviceResponse {
    pub fn sysinfo(&self) -> &SysInfo {
        &self.system.sysinfo
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct System {
    #[serde(rename = "get_sysinfo")]
    pub sysinfo: SysInfo,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SysInfo {
    pub alias: String,
    pub brightness: Option<u8>,
    #[serde(rename = "deviceId")]
    pub device_id: String,
    pub err_code: ErrCode,
    #[serde(rename = "hwId")]
    pub hw_id: String,
    #[serde(alias = "type")]
    #[serde(alias = "mic_type")]
    pub hw_type: String,
    pub hw_ver: String,
    pub latitude_i: Option<i32>,
    pub longitude_i: Option<i32>,
    #[serde(alias = "mic_mac")]
    pub mac: String,
    pub model: String,
    pub on_time: Option<i64>,
    pub relay_state: Option<u8>,
    pub rssi: i32,
    pub sw_ver: String,
    pub updating: Option<u8>,
}

impl SysInfo {
    /// If the device isn't dimmable, returns 0 or 100 based solely on relay
    /// state
    pub fn brightness(&self) -> u8 {
        self.brightness
            .unwrap_or(if self.is_on() { 100 } else { 0 })
    }

    pub fn brightness_f32(&self) -> f32 {
        (self.brightness() as f32) / 100.
    }

    pub fn is_on(&self) -> bool {
        self.relay_state
            .map(|relay_state| relay_state > 0)
            .unwrap_or(false)
    }
}
