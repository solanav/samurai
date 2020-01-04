use crate::network::packet::{Packet, TOTAL_SIZE};
use std::error::Error;
use std::net::SocketAddr;
use std::net::UdpSocket;

struct Server {
    socket: UdpSocket, // Server's socket
    packets: Vec<Packet>, // List of packets not yet processed
}

impl Server {
    pub fn new() {        
        Server {
            socket: UdpSocket::bind("127.0.0.1:4321")
                .expect("Could not bind to that address");,
            packet: Vec::new(),
        }
    }

    pub fn start() {
        let buf = [0; TOTAL_SIZE];

        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf)
            .expect("Did not receive data");

        println!("{:?}", buf);
    }
}