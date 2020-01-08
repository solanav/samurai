use crate::network::packet::{self, Packet};
use crate::network::active::Client;

pub fn switch(packet: &Packet) {
    match packet.header() {
        packet::PING_HEADER => ping(packet),
        packet::PONG_HEADER => pong(packet),
        _ => println!("Header not found")
    }
}

fn ping(packet: &Packet) {
    println!("PING {:?}", packet);
    let client = Client::new();
}

fn pong(packet: &Packet) {
    println!("PONG {:?}", packet);
    let client = Client::new();
}