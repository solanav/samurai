use crate::network::active::Client;
use crate::network::packet::{self, Packet};
use std::net::SocketAddr;

pub fn switch(packet: &Packet, src: SocketAddr) {
    match packet.header() {
        packet::PING_HEADER => ping(packet, src),
        packet::PONG_HEADER => pong(packet, src),
        _ => println!("Header not found"),
    }
}

fn ping(packet: &Packet, src: SocketAddr) {
    println!("PING {:?}", packet);
    let client = Client::new();
    client.pong(src, packet.cookie());
}

fn pong(packet: &Packet, src: SocketAddr) {
    println!("PONG {:?}", packet);
    let client = Client::new();
    client.ping(src);
}
