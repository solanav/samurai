use rand::Rng;
use std::net::SocketAddr;
use std::net::UdpSocket;
use crate::network::packet::{Packet, *};

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
        let packet = Packet::new(PING_HEADER, cookie, &[0; DATA_SIZE]);
        self.send_packet(dst, packet);
    }
}
