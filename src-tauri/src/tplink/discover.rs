use super::{
    error::TpResult,
    models::DeviceData,
    protocol::{decrypt, encrypt},
};
use log::*;
use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    time::Duration,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct DiscoverConfig {
    pub listen_timeout: Duration,
    /// The IP address of the ESP32
    pub socket_ip: Ipv4Addr,
}

impl DiscoverConfig {
    pub fn from_ip(socket_ip: Ipv4Addr) -> Self {
        Self {
            listen_timeout: Default::default(),
            socket_ip,
        }
    }

    pub fn set_listen_timeout(self, listen_timeout: Duration) -> Self {
        Self {
            listen_timeout,
            ..self
        }
    }
}

/// The address that we send discovery packets to.
/// I have no reason to believe this'll ever be anything different.
const BROADCAST_ADDR: SocketAddr =
    SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(192, 168, 1, 255)), 9999);

// "schedule": {"get_next_action": {}},
// "cnCloud": {"get_info": {}},
// "emeter": {"get_realtime": null},
// "smartlife.iot.common.emeter": {"get_realtime": null},
const QUERY: &str = r#"{
    "system": {"get_sysinfo": null},
    "smartlife.iot.dimmer": {"get_dimmer_parameters": null},
    "smartlife.iot.smartbulb.lightingservice": {"get_light_state": null}
}"#;

pub fn discover_devices(config: DiscoverConfig) -> TpResult<Vec<(SocketAddr, DeviceData)>> {
    debug!("Begin discovery");
    let socket_addr = SocketAddr::new(std::net::IpAddr::V4(config.socket_ip), 0);
    let udp_socket = UdpSocket::bind(socket_addr)?;
    udp_socket.set_broadcast(true)?;
    udp_socket.set_read_timeout(Some(config.listen_timeout))?;

    let request = encrypt(QUERY).unwrap();
    let mut buf = [0_u8; 4096];
    let mut devices = HashMap::new();

    if let Err(err) = udp_socket.send_to(&request[4..request.len()], BROADCAST_ADDR) {
        error!("udp_socket.send_to error {:?}", err);
    }

    while let Ok((size, addr)) = udp_socket.recv_from(&mut buf) {
        debug!("Socket recvd {} bytes from {:?}", size, addr);

        if devices.contains_key(&addr) {
            continue;
        }

        let data = decrypt(&mut buf[0..size]);
        debug!("Decrypted buffer\n{}", data);

        match serde_json::from_str::<DeviceData>(&data) {
            Ok(device_data) => {
                devices.insert(addr, device_data);
                debug!("Device discovered at {:?}", addr);
            }
            Err(err) => {
                error!("Device data (address {:?}) parse error {:#?}", addr, err);
            }
        }
    }

    debug!("Discovery complete");
    Ok(devices.into_iter().collect())
}
