use std::net::SocketAddr;
use std::net::UdpSocket;
use crate::network::packet::{Packet, *};
use crate::types::id::{Id, ID_BYTES};
use std::sync::{Arc, Mutex};
use crate::network::passive::ReqList;

pub struct Client {
    socket: UdpSocket, // Client's socket (Same as server)
    num_nodes: usize, // Number of nodes we send when someone asks
    requests: Arc<Mutex<ReqList>> // List of requests
}

impl Client {
    pub fn new(num_nodes: usize, requests: Arc<Mutex<ReqList>>, socket: UdpSocket) -> Self {
        // Check if num_nodes is ok
        let node_list_size = ID_BYTES * num_nodes as usize;
        if node_list_size > DATA_SIZE {
            panic!("Cannot save that many nodes on a packet");
        }

        // Create client
        Client {
            socket,
            num_nodes,
            requests,
        }
    }

    pub fn num_nodes(&self) -> usize { self.num_nodes }

    fn send_bytes(&self, dst: SocketAddr, buf: &[u8]) {
        match self.socket.send_to(buf, dst) {
            Ok(_) => {},
            Err(e) => println!("Failed to send bytes \"{}\"", e),
        }
    }

    fn send_request(&self, dst: SocketAddr, packet: Packet) {
        self.send_bytes(dst, &packet.as_bytes());
    }

    fn send_response(&self, dst: SocketAddr, packet: Packet) {
        self.send_bytes(dst, &packet.as_bytes());
        self.requests
            .lock()
            .unwrap()
            .push_back(packet);
    }

    pub fn ping(&self, dst: SocketAddr) {
        let packet = Packet::new_with_cookie(PING_HEADER, &[0; DATA_SIZE]);
        self.send_request(dst, packet);
    }

    pub fn pong(&self, dst: SocketAddr, cookie: u32) {
        let packet = Packet::new(PONG_HEADER, cookie, &[0; DATA_SIZE]);
        self.send_response(dst, packet);
    }

    pub fn find_node(&self, dst: SocketAddr, id: &Id) {
        let mut buf = [0u8; DATA_SIZE];
        let id_bytes = id.as_bytes();

        for i in 0..id_bytes.len() {
            buf[i] = id_bytes[i];
        }

        let packet = Packet::new_with_cookie(FINDNODE_HEADER, &buf);
        self.send_request(dst, packet);
    }

    pub fn send_node(&self, dst: SocketAddr, cookie: u32, id_list: &Vec<Id>) {
        let mut buf = [0u8; DATA_SIZE];

        let mut j = 0;
        for i in 0..id_list.iter().len() {
            // Careful not to add too many ID
            if i >= DATA_SIZE/ID_BYTES {
                break;
            }

            // Copy ID to the buffer
            for b in id_list[i].as_bytes().iter() {
                buf[j] = *b;
                j += 1;
            }
        }

        let packet = Packet::new(SENDNODE_HEADER, cookie, &buf);
        self.send_response(dst, packet);
    }

    pub fn send_message(&self, dst: SocketAddr, msg: &String) {
        let mut buf = [0u8; DATA_SIZE];

        let mut i = 0;
        for b in msg.as_bytes().iter() {
            if i >= 502 {
                break;
            }

            buf[i] = *b;
            i += 1;
        }

        let packet = Packet::new_with_cookie(SENDNODE_HEADER, &buf);
        self.send_request(dst, packet);
    }

    pub fn send_echo(&self, dst: SocketAddr, cookie: u32, buf: &[u8; DATA_SIZE]) {
        let packet = Packet::new(SENDNODE_HEADER, cookie, buf);
        self.send_response(dst, packet);
    }
}
