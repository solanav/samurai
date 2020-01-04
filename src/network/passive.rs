use crate::network::packet::{Packet, TOTAL_SIZE};
use std::net::UdpSocket;

pub struct Server {
    socket: UdpSocket, // Server's socket
    packets: Vec<Packet>, // List of packets not yet processed
}

impl Server {
    pub fn new(port: u16) -> Self {        
        Server {
            socket: UdpSocket::bind(format!("127.0.0.1:{}", port))
                .expect("Could not bind to that address"),
            packets: Vec::new(),
        }
    }

    pub fn start(&self) {
        let mut buf = [0; TOTAL_SIZE];

        let (number_of_bytes, src_addr) = self.socket.recv_from(&mut buf)
            .expect("Did not receive data");

        println!("<<< RECV {} [{}]", src_addr, number_of_bytes);
    }
}