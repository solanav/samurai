use std::net::{SocketAddr, SocketAddrV6};
use rand::Rng;

pub fn random_ipv4() -> SocketAddr {
    let mut rng = rand::thread_rng();

    let mut ip = [0u8; 4];
    for i in 0..4 {
        ip[i] = rng.gen();
    }

    let port: u16 = rng.gen();

    SocketAddr::from((ip, port))
}

pub fn random_ipv6() -> SocketAddr {
    let mut rng = rand::thread_rng();

    let mut ip = [0u8; 16];
    for i in 0..16 {
        ip[i] = rng.gen();
    }

    let port: u16 = rng.gen();

    SocketAddr::from((ip, port))
}