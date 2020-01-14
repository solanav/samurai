use rand::Rng;
use std::net::SocketAddr;
use std::net::UdpSocket;
use crate::network::packet::{Packet, *};
use crate::types::id::{Id, ID_BYTES};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use crate::network::passive::ReqList;

pub struct Client {
    socket: UdpSocket, // Client's socket
    num_nodes: usize, // Number of nodes we send when someone asks
    requests: ReqList // List of requests
}

impl Client {
    pub fn new(num_nodes: usize) -> Self {
        let node_list_size = ID_BYTES * num_nodes as usize;
        if node_list_size > DATA_SIZE {
            panic!("Cannot save that many nodes on a packet");
        }

        let mut rng = rand::thread_rng();
        let socket = UdpSocket::bind(format!("127.0.0.1:{}", rng.gen_range(1024, 65536)))
            .expect("couldn't bind to address");

        Client {
            socket,
            num_nodes,
            requests: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn num_nodes(&self) -> usize { self.num_nodes }

    fn send_bytes(&self, dst: SocketAddr, buf: &[u8]) {
        self.socket
            .connect(format!("{}", dst))
            .expect("Connect function failed");

        self.socket.send(buf).expect("Couldn't send message");
    }

    fn send_packet(&self, dst: SocketAddr, packet: Packet) {
        self.send_bytes(dst, &packet.as_bytes());
        self.requests
            .lock()
            .unwrap()
            .push_back(packet);
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

    pub fn send_node(&self, dst: SocketAddr, cookie: u32, id_list: &Vec<Id>) {
        let mut buf = [0u8; DATA_SIZE];

        for i in 0..id_list.iter().len() {
            // Careful not to add too many ID
            if i >= DATA_SIZE/ID_BYTES {
                break;
            }

            for b in id.as_bytes().iter() {
                buf[i] = *b;
            }
        }

        let packet = Packet::new(SENDNODE_HEADER, cookie, &buf);
        self.send_packet(dst, packet);
    }

    pub fn requests(&self) -> ReqList {
        Arc::clone(&self.requests)
    }
}
