use crate::network::handler::Handler;
use crate::network::packet::{Packet, TOTAL_SIZE};
use std::net::UdpSocket;
use std::thread;

pub struct Server {
    socket: UdpSocket, // Server's socket
    port: u16,         // List of packets not yet processed
    num_nodes: usize,  // Number of nodes we send when find_node is received
}

impl Server {
    pub fn new(port: u16, num_nodes: usize) -> Self {
        Server {
            // TODO: Try ports until you get one that is free
            socket: UdpSocket::bind(format!("127.0.0.1:{}", port))
                .expect("Could not bind to that address"),
            num_nodes: num_nodes,
            port: port,
        }
    }

    pub fn start(self) {
        thread::spawn(move || loop {
            let mut buf = [0; TOTAL_SIZE];
            let handler = Handler::new(self.num_nodes);
            let (number_of_bytes, src_addr) = self
                .socket
                .recv_from(&mut buf)
                .expect("Did not receive data");
            let packet = Packet::from_bytes(&buf);
            handler.switch(&packet, src_addr);
        });
    }
}
