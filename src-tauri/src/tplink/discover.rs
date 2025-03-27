use super::{
    error::TpResult,
    models::DeviceResponse,
    protocol::{decrypt, encrypt},
};
use log::*;
use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    time::Duration,
};

/// The address that we send discovery packets to.
/// I have no reason to believe this'll ever be anything different.
const BROADCAST_ADDR: SocketAddr =
    SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(192, 168, 1, 255)), 9999);

// "schedule": {"get_next_action": {}},
// "cnCloud": {"get_info": {}},
// "emeter": {"get_realtime": null},
// "smartlife.iot.common.emeter": {"get_realtime": null},
// "smartlife.iot.smartbulb.lightingservice": {"get_light_state": null},
// "smartlife.iot.smartbulb.lightingservice": {"get_default_behavior":""}
// "smartlife.iot.common.timesetting":{"get_time":{}},
// "smartlife.iot.dimmer": {"get_default_behavior": null},
// "smartlife.iot.dimmer": {"get_dimmer_parameters": null},
// "smartlife.iot.smartbulb.lightingservice": {"get_light_details": null},
const QUERY: &str = r#"{ "system": {"get_sysinfo": null} }"#;

pub fn discover_devices() -> TpResult<Vec<(SocketAddr, DeviceResponse)>> {
    debug!("Begin discovery");
    // let socket_addr = SocketAddr::new(std::net::IpAddr::V4(config.socket_ip), 0);
    let socket_addr = SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);
    let udp_socket = UdpSocket::bind(socket_addr)?;
    udp_socket.set_broadcast(true)?;
    udp_socket.set_read_timeout(Some(Duration::from_secs(1)))?;

    let request = encrypt(QUERY).unwrap();
    let mut buf = [0_u8; 4096];
    let mut devices = HashMap::new();

    if let Err(err) = udp_socket.send_to(&request[4..request.len()], BROADCAST_ADDR) {
        error!("udp_socket.send_to error {:?}", err);
    }

    while let Ok((size, addr)) = udp_socket.recv_from(&mut buf) {
        debug!("Socket recvd {} bytes from {:?}", size, addr);

        if devices.contains_key(&addr) {
            info!("Already have device at {:?}", addr);
            continue;
        }

        let data = decrypt(&mut buf[0..size]);
        // info!("\n\nDecrypted buffer\n{}", data);

        match serde_json::from_str::<DeviceResponse>(&data) {
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
