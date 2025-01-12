#![allow(unused)]
use serde_json::json;
use std::time::Duration;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum Request {
    GetSysinfo,
    SetAlias(String),
    SetBrightness(u8),
    SetRelayState(bool),
    Reboot,
    RebootWithDelay(Duration),
}

impl Request {
    pub fn to_string(&self) -> String {
        match self {
            Request::GetSysinfo => json!({
              "system": {"get_sysinfo": null}
            })
            .to_string(),
            Request::SetAlias(alias) => json!({
              "system": {"set_dev_alias": {"alias": alias}}
            })
            .to_string(),
            Request::SetBrightness(brightness) => json!({
                "smartlife.iot.dimmer": {
                    "set_brightness": {
                        "brightness": (*brightness as u8).clamp(0, 100)
                    }
                }
            })
            .to_string(),
            Request::SetRelayState(state) => json!({
              "system":{"set_relay_state":{"state": if *state { 1 } else { 0 }}}
            })
            .to_string(),
            Request::Reboot => json!({
                "system": {"reboot": {"delay": 0 }}
            })
            .to_string(),
            Request::RebootWithDelay(delay) => json!({
                "system": {"reboot": {"delay": delay.as_secs()}}
            })
            .to_string(),
        }
    }
}
