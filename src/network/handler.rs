use crate::network::active::Client;
use crate::network::packet::{self, Packet};
use std::net::SocketAddr;

pub fn switch(packet: &Packet, src: SocketAddr) {
    match packet.header() {
        packet::PING_HEADER => ping(packet, src),
        packet::PONG_HEADER => pong(packet),
        _ => println!("Header not found"),
    }
}

fn ping(packet: &Packet, mut src: SocketAddr) {
    println!("RECV PING FROM {}", src);
    let client = Client::new();
    src.set_port(4321);
    client.pong(src, packet.cookie());
}

fn pong(packet: &Packet) {
    println!("RECV PONG\n");
}

fn find_node(packet: &Packet) {
    println!("Someone is asking about nodes")
}