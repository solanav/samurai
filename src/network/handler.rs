use crate::network::active::Client;
use crate::network::packet::{self, Packet};
use std::net::SocketAddr;
use crate::kbucket::id::{Id, ID_BYTES};

pub fn switch(packet: &Packet, src: SocketAddr) {
    match packet.header() {
        packet::PING_HEADER => ping(packet, src),
        packet::PONG_HEADER => pong(packet),
        packet::FINDNODE_HEADER => find_node(packet, src),
        packet::SENDNODE_HEADER => send_node(packet),
        _ => println!("Header not found, dropping packet"),
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

fn find_node(packet: &Packet, mut src: SocketAddr) {
    let mut id_bytes = [0u8; ID_BYTES];
    id_bytes.copy_from_slice(&packet.data()[..ID_BYTES]);
    println!("RECV FINDNODE {:?}", Id::from_bytes(&id_bytes));

    // TODO: find the closest nodes and send them
}

fn send_node(packet: &Packet) {
    println!("RECV SENDNODE");
}