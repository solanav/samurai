use std::net::{Ipv4Addr, IpAddr, SocketAddrV4};
use crate::error::ServerError;
use get_if_addrs::Interface;

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