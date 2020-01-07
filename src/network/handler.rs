use crate::network::packet::{self, Packet};

pub fn switch(packet: &Packet) {
    match *packet.header() {
        packet::PING_HEADER => ping(packet),
        packet::PONG_HEADER => pong(packet),
        _ => {}
    }
}

fn ping(packet: &Packet) {
    println!("PING {:?}", packet);
}

fn pong(packet: &Packet) {
    println!("PONG {:?}", packet);
}