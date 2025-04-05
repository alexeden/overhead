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
// "smartlife.iot.common.timesetting":{"get_time":{}},
// "smartlife.iot.dimmer": {"get_default_behavior": null},
// "smartlife.iot.dimmer": {"get_dimmer_parameters": null},
const DISCOVERY_QUERY: &str = r#"{ "system": {"get_sysinfo": null} }"#;
// "smartlife.iot.smartbulb.lightingservice": {"get_light_state": null}
// {"system":{"get_sysinfo":{"sw_ver":"1.0.15 Build 240429 Rel.154143","hw_ver":"1.0","model":"KL135(US)","deviceId":"8012F9190EB182F28C9C5B5B07E2C9A822788D29","oemId":"C1AC4843FD5824205B8C51C5131FB350","hwId":"CCCEA814DB7786E5A00F65412AAD11CE","rssi":-54,"latitude_i":-1879048193,"longitude_i":-1879048193,"alias":"Dining ","status":"new","obd_src":"tplink","description":"Smart Wi-Fi LED Bulb with Color Changing","mic_type":"IOT.SMARTBULB","mic_mac":"74FECEB8449B","dev_state":"normal","is_factory":false,"disco_ver":"1.0","ctrl_protocols":{"name":"Linkie","version":"1.0"},"active_mode":"none","is_dimmable":1,"is_color":1,"is_variable_color_temp":1,"light_state":{"on_off":1,"mode":"normal","hue":329,"saturation":99,"color_temp":0,"brightness":79},"preferred_state":[{"index":0,"hue":0,"saturation":0,"color_temp":2700,"brightness":50},{"index":1,"hue":0,"saturation":100,"color_temp":0,"brightness":100},{"index":2,"hue":120,"saturation":100,"color_temp":0,"brightness":100},{"index":3,"hue":240,"saturation":100,"color_temp":0,"brightness":100}],"err_code":0}},"smartlife.iot.smartbulb.lightingservice":{"get_light_state":{"on_off":1,"hue":329,"saturation":99,"color_temp":0,"brightness":79,"mode":"normal","err_code":0}}}
// "smartlife.iot.smartbulb.lightingservice": {"get_default_behavior":""},
// "smartlife.iot.smartbulb.lightingservice": {"get_light_details": null},

pub fn discover_devices() -> TpResult<Vec<(SocketAddr, DeviceResponse)>> {
    debug!("Begin discovery");
    let socket_addr = SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);
    let udp_socket = UdpSocket::bind(socket_addr)?;
    udp_socket.set_broadcast(true)?;
    udp_socket.set_read_timeout(Some(Duration::from_secs(1)))?;

    let request = encrypt(DISCOVERY_QUERY).unwrap();
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
        debug!("\n\nDecrypted buffer\n{}", data);

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
