use network_interface::{Addr, NetworkInterface, NetworkInterfaceConfig, V4IfAddr};
use std::net::Ipv4Addr;

pub(crate) fn get_local_ip_addr(netif_name: &str) -> Option<Ipv4Addr> {
    NetworkInterface::show()
        .ok()
        .and_then(|interfaces| {
            interfaces
                .into_iter()
                .find(|netif| netif.name == netif_name)
        })
        .and_then(|netif| {
            netif.addr.into_iter().find_map(|addr| match addr {
                Addr::V4(V4IfAddr {
                    ip,
                    broadcast: Some(_),
                    netmask: _,
                }) => Some(ip),
                _ => None,
            })
        })
}
