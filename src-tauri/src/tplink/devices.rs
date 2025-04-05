use super::{
    error::{TpError, TpResult},
    prelude::*,
    protocol::send,
};
use log::*;
use serde::de::DeserializeOwned;
use std::net::SocketAddr;

macro_rules! new_device {
    ( $model:ident ) => {
        #[derive(Clone, Debug)]
        pub struct $model {
            pub addr: SocketAddr,
        }

        impl $model {
            pub fn new(addr: SocketAddr) -> Self {
                Self { addr: addr.clone() }
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
new_device!(KL135);
impl Dimmable for HS220 {}
impl Dimmable for KL135 {}

#[derive(Clone, Debug)]
pub enum Device {
    /// Device variant for an EP10 smart plug
    EP10(EP10),
    /// Dimmable switch
    HS220(HS220),
    /// Smart bulb
    KL135(KL135),
}

impl Device {
    pub fn try_new(addr: SocketAddr, model: &str) -> TpResult<Device> {
        // let model = &device_data.sysinfo().model;
        if model.contains("EP10") {
            Ok(Device::EP10(EP10::new(addr)))
        } else if model.contains("HS220") || model.contains("KP405") || model.contains("ES20M") {
            Ok(Device::HS220(HS220::new(addr)))
        } else if model.contains("KL135") {
            Ok(Device::KL135(KL135::new(addr)))
        } else {
            Err(TpError::UnknownModel(model.to_string()))
        }
    }

    pub fn try_into_dimmable(&mut self) -> TpResult<&mut impl Dimmable> {
        match self {
            Device::HS220(d) => Ok(d),
            Device::KL135(d) => Ok(d),
            _ => Err(TpError::Unsupported("dimmable".to_string())),
        }
    }
}

impl CommonCapabilities for Device {
    fn send<D: DeserializeOwned>(&self, msg: &str) -> TpResult<D> {
        match self {
            Device::EP10(d) => d.send(msg),
            Device::HS220(d) => d.send(msg),
            Device::KL135(d) => d.send(msg),
        }
    }
}
