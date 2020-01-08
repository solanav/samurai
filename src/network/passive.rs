use crate::network::handler;
use crate::network::packet::{Packet, TOTAL_SIZE};
use std::net::UdpSocket;
use std::thread;

pub struct Server {
    socket: UdpSocket, // Server's socket
    port: u16,         // List of packets not yet processed
}

impl Server {
    pub fn new(port: u16) -> Self {
        Server {
            // TODO: Try ports until you get one that is free
            socket: UdpSocket::bind(format!("127.0.0.1:{}", port))
                .expect("Could not bind to that address"),
            port: port,
        }
    }

    pub fn start(self) {
        thread::spawn(move || loop {
            let mut buf = [0; TOTAL_SIZE];
            let (number_of_bytes, src_addr) = self
                .socket
                .recv_from(&mut buf)
                .expect("Did not receive data");
            let packet = Packet::from_bytes(&buf);
            handler::switch(&packet, src_addr);
        });
    }
}
