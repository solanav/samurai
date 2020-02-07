use std::net::{Ipv4Addr, IpAddr, SocketAddrV4};
use crate::error::ServerError;

/// Get ip in the current interface
pub fn local_ip() -> Ipv4Addr {
    // Get internal IP
    let ip_list = get_if_addrs::get_if_addrs().unwrap();
    for ip in ip_list.iter() {
        if let IpAddr::V4(ip) = ip.ip() {
            return ip;
        }
    }

    panic!("Failed to get local IPv4");
}

/// Uses UPnP to open a port on the router
pub fn open_port(local_ip: Ipv4Addr, local_port: u16) -> Result<u16, ServerError> {
    // Get a random external port with UPnP redirected to our internal port
    let port: u16;
    match igd::search_gateway(Default::default()) {
        Err(_) => return Err(ServerError::SearchRouter),
        Ok(gateway) => {
            match gateway.add_any_port(
                igd::PortMappingProtocol::TCP,
                SocketAddrV4::new(local_ip, local_port),
                10,
                "Samurai") {
                Err(_) => return Err(ServerError::AddPort),
                Ok(p) => port = p,
            }
        }
    }

    Ok(port)
}