use super::{
    capabilities::{CachedControlParams, ControlParams},
    error::TpResult,
    models::DeviceData,
    prelude::*,
    protocol::send,
};
use log::*;
use serde::de::DeserializeOwned;
use std::net::SocketAddr;

// #[derive(Clone, Debug)]
// pub struct DeviceInfo {
//     pub addr: SocketAddr,
//     pub alias: String,
//     pub mac: String,
//     pub model: String,
// }

// impl DeviceInfo {
//     pub fn new(addr: SocketAddr, data: DeviceData) -> Self {
//         let sysinfo = data.system.sysinfo;

//         Self {
//             addr,
//             alias: sysinfo.alias,
//             mac: sysinfo.mac,
//             model: sysinfo.model,
//         }
//     }
// }

macro_rules! new_device {
    ( $model:ident ) => {
        #[derive(Clone, Debug)]
        pub struct $model {
            pub addr: SocketAddr,
            control_params: ControlParams,
            // info: DeviceInfo,
        }

        impl $model {
            pub fn new(addr: SocketAddr, data: &DeviceData) -> Self {
                Self {
                    addr: addr.clone(),
                    control_params: ControlParams::from_sysinfo(&data.system.sysinfo),
                    // info: DeviceInfo::new(addr, data.clone()),
                }
            }
        }

        impl CachedControlParams for $model {
            fn get_cached_params(&self) -> ControlParams {
                self.control_params
            }

            fn set_cached_params(&mut self, params: ControlParams) -> () {
                self.control_params = params;
                info!(
                    "{} - Params set {:?}",
                    std::thread::current().name().unwrap_or("NO THREAD NAME"),
                    params
                )
            }
        }

        impl CommonCapabilities for $model {
            fn send<D: DeserializeOwned>(&self, msg: &str) -> TpResult<D> {
                debug!("Sending {}", msg);
                Ok(serde_json::from_str::<D>(&send(self.addr, &msg)?)?)
            }
        }
    };
}

new_device!(EP10);
new_device!(HS220);
impl Dimmable for HS220 {}

#[derive(Clone, Debug)]
pub enum Device {
    /// Device variant for an EP10 smart plug
    EP10(EP10),
    /// Dimmable switch
    HS220(HS220),
}

impl Device {
    pub fn from_data(addr: SocketAddr, device_data: &DeviceData) -> Option<Device> {
        let model = &device_data.sysinfo().model;
        if model.contains("EP10") {
            Some(Device::EP10(EP10::new(addr, device_data)))
        } else if model.contains("HS220") || model.contains("KP405") || model.contains("ES20M") {
            Some(Device::HS220(HS220::new(addr, device_data)))
        } else {
            warn!("Unknown device {:?}", device_data);
            None
        }
    }

    // pub(crate) fn info(&self) -> &DeviceInfo {
    //     match self {
    //         Device::EP10(d) => &d.info,
    //         Device::HS220(d) => &d.info,
    //     }
    // }

    pub fn as_dimmable(&mut self) -> Option<&mut impl Dimmable> {
        match self {
            Device::HS220(d) => Some(d),
            _ => None,
        }
    }

    pub fn is_dimmable(&self) -> bool {
        if let Device::HS220(_) = self {
            true
        } else {
            false
        }
    }
}

impl CachedControlParams for Device {
    fn get_cached_params(&self) -> ControlParams {
        match self {
            Device::EP10(d) => d.control_params,
            Device::HS220(d) => d.control_params,
        }
    }

    fn set_cached_params(&mut self, params: ControlParams) {
        match self {
            Device::EP10(d) => d.set_cached_params(params),
            Device::HS220(d) => d.set_cached_params(params),
        }
    }
}

impl CommonCapabilities for Device {
    fn send<D: DeserializeOwned>(&self, msg: &str) -> TpResult<D> {
        match self {
            Device::EP10(d) => d.send(msg),
            Device::HS220(d) => d.send(msg),
        }
    }
}
