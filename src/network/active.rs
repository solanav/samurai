use rand::Rng;
use std::net::SocketAddr;
use std::net::UdpSocket;
use crate::network::packet::{Packet, *};
use crate::kbucket::id::Id;

pub struct Client {
    socket: UdpSocket, // Client's socket
}

impl Client {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let socket = UdpSocket::bind(format!("127.0.0.1:{}", rng.gen_range(1024, 65536)))
            .expect("couldn't bind to address");

        Client { socket: socket }
    }

    fn send_bytes(&self, dst: SocketAddr, buf: &[u8]) {
        self.socket
            .connect(format!("{}", dst))
            .expect("connect function failed");

        self.socket.send(buf).expect("couldn't send message");
    }

    fn send_packet(&self, dst: SocketAddr, packet: Packet) {
        self.send_bytes(dst, &packet.as_bytes());
    }

    pub fn ping(&self, dst: SocketAddr) {
        let packet = Packet::new_with_cookie(PING_HEADER, &[0; DATA_SIZE]);
        self.send_packet(dst, packet);
    }

    pub fn pong(&self, dst: SocketAddr, cookie: u32) {
        let packet = Packet::new(PONG_HEADER, cookie, &[0; DATA_SIZE]);
        self.send_packet(dst, packet);
    }

    pub fn find_node(&self, dst: SocketAddr, id: &Id) {
        let mut buf = [0u8; DATA_SIZE];
        let id_bytes = id.as_bytes();

        for i in 0..id_bytes.len() {
            buf[i] = id_bytes[i];
        }

        let packet = Packet::new_with_cookie(FINDNODE_HEADER, &buf);
        self.send_packet(dst, packet);
    }

    pub fn send_node(&self, dst: SocketAddr, cookie: u32, id: &[Id; 10]) {
        let packet = Packet::new(PONG_HEADER, cookie, &[0; DATA_SIZE]);
        self.send_packet(dst, packet);
    }
}
