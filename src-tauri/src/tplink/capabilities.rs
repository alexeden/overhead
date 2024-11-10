#![allow(unused)]
use super::{
    error::{TpError, TpResult},
    models::{DeviceData, SysInfo},
    protocol::validate_response_code,
};
use log::*;
use serde::de::DeserializeOwned;
use serde_json::json;
use std::time::Duration;

#[derive(Copy, Clone, Debug)]
pub struct ControlParams {
    pub brightness: u8,
    pub is_on: bool,
}

impl ControlParams {
    pub fn from_sysinfo(sysinfo: &SysInfo) -> Self {
        Self {
            brightness: sysinfo.brightness(),
            is_on: sysinfo.is_on(),
        }
    }
}

pub trait CachedControlParams {
    fn get_cached_params(&self) -> ControlParams;

    fn set_cached_params(&mut self, params: ControlParams) -> ();

    fn set_cached_brightness(&mut self, brightness: u8) -> () {
        self.set_cached_params(ControlParams {
            brightness,
            ..self.get_cached_params()
        })
    }

    fn set_cached_is_on(&mut self, is_on: bool) -> () {
        self.set_cached_params(ControlParams {
            is_on,
            ..self.get_cached_params()
        })
    }
}

/// The basic set of functions available to all TPLink smart devices
/// All devices support this trait.
pub trait CommonCapabilities: CachedControlParams {
    /// Send a message to a device and return its parsed response
    /// Will return `Err` if there is a `io::Error` communicating with the
    /// device or a problem decoding the response.
    fn send<T: DeserializeOwned>(&self, msg: &str) -> TpResult<T>;

    /// Get system information
    fn get_sysinfo(&mut self) -> TpResult<SysInfo> {
        Ok(self
            .send::<DeviceData>(r#"{"system":{"get_sysinfo":null}}"#)?
            .system
            .sysinfo)
        .map(|sysinfo| {
            self.set_cached_params(ControlParams::from_sysinfo(&sysinfo));
            info!("Cached control params set");
            sysinfo
        })
    }

    fn get_alias(&mut self) -> TpResult<String> {
        Ok(self.get_sysinfo()?.alias)
    }

    fn set_alias(&self, alias: &str) -> TpResult<()> {
        let command = json!({ "system": {"set_dev_alias": {"alias": alias}} }).to_string();

        validate_response_code(&self.send(&command)?, "/system/set_dev_alias/err_code")
    }

    /// Reboot the device in 1 second
    fn reboot(&self) -> TpResult<()> {
        self.reboot_with_delay(Duration::from_secs(1))
    }

    /// Reboot the device with a specified delay
    fn reboot_with_delay(&self, delay: Duration) -> TpResult<()> {
        let command = json!({
            "system": {"reboot": {"delay": delay.as_secs()}}
        })
        .to_string();

        validate_response_code(&self.send(&command)?, "/system/reboot/err_code")
    }

    /// Check whether the device is on
    fn get_is_on(&mut self) -> TpResult<bool> {
        self.get_sysinfo().map(|sysinfo| sysinfo.is_on())
    }

    /// Check whether the device is off
    fn get_is_off(&mut self) -> TpResult<bool> {
        Ok(!self.get_is_on()?)
    }

    /// Switch the device on
    fn switch_on(&mut self) -> TpResult<()> {
        validate_response_code(
            &self.send(&r#"{"system":{"set_relay_state":{"state":1}}}"#)?,
            "/system/set_relay_state/err_code",
        )
        .map(|_| self.set_cached_is_on(true))
    }

    /// Switch the device off
    fn switch_off(&mut self) -> TpResult<()> {
        validate_response_code(
            &self.send(&r#"{"system":{"set_relay_state":{"state":0}}}"#)?,
            "/system/set_relay_state/err_code",
        )
        .map(|_| self.set_cached_is_on(false))
    }

    /// Toggle the device's on state
    fn toggle(&mut self) -> TpResult<bool> {
        if self.get_is_on()? {
            self.switch_off()?;
            Ok(false)
        } else {
            self.switch_on()?;
            Ok(true)
        }
    }
}

pub trait Dimmable: CommonCapabilities {
    fn get_dimmer_parameters(&self) -> TpResult<()> {
        let command = json!({"smartlife.iot.dimmer":{"get_dimmer_parameters":{}}}).to_string();

        validate_response_code(
            &self.send(&command)?,
            "/smartlife.iot.dimmer/get_dimmer_parameters/err_code",
        )
    }

    fn get_default_behavior(&self) -> TpResult<()> {
        let command = json!({"smartlife.iot.dimmer":{"get_default_behavior":{}}}).to_string();

        validate_response_code(
            &self.send(&command)?,
            "/smartlife.iot.dimmer/get_default_behavior/err_code",
        )
    }

    /// @todo Replace with transition state
    fn set_transition(&mut self, brightness: u8) -> TpResult<()> {
        let brightness = brightness.min(100).max(1);
        let command = json!({
            "smartlife.iot.dimmer": {
                "set_dimmer_transition": {
                    "brightness": brightness,
                    "mode": "gentle_on_off",
                    "duration": 1
                }
            }
        })
        .to_string();

        validate_response_code(
            &self.send(&command)?,
            "/smartlife.iot.dimmer/set_dimmer_transition/err_code",
        )
        .map(|_| self.set_cached_brightness(brightness))
    }

    fn set_brightness(&mut self, brightness: u8) -> TpResult<()> {
        let brightness = brightness.min(100).max(1);

        let command = json!({
            "smartlife.iot.dimmer": {
                "set_brightness": {
                    "brightness": brightness
                }
            }
        })
        .to_string();

        validate_response_code(
            &self.send(&command)?,
            "/smartlife.iot.dimmer/set_brightness/err_code",
        )
        .map(|_| self.set_cached_brightness(brightness))
    }

    // fn set_switch_state(&self, switch_on: bool) -> TpResult<()> {
    //     let state = if switch_on { 1 } else { 0 };

    //     let command = json!({
    //         "smartlife.iot.dimmer": {
    //             "set_switch_state": {
    //                 "state": state
    //             }
    //         }
    //     })
    //     .to_string();

    //     validate_response_code(
    //         &self.send(&command)?,
    //         "/smartlife.iot.dimmer/set_switch_state/err_code",
    //     )
    // }
}
