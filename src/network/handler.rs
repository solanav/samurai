use crate::network::active::Client;
use crate::network::packet::{self, Packet, DATA_SIZE};
use std::net::SocketAddr;
use crate::types::id::{Id, ID_BYTES};
use crate::network::passive::ReqList;
use crate::types::bucket_list::BucketList;
use std::sync::{Arc, Mutex};

pub struct Handler {
    client: Client, // Used to respond to messages
    bucket_list: Arc<Mutex<BucketList>>,
    requests: Arc<Mutex<ReqList>>,
}

impl Handler {
    pub fn new(num_nodes: usize, requests: Arc<Mutex<ReqList>>, bucket_list: Arc<Mutex<BucketList>>) -> Self {
        Handler {
            client: Client::new(num_nodes),
            bucket_list,
            requests,
        }
    }

    pub fn switch(&self, packet: &Packet, src: SocketAddr) {
        match packet.header() {
            packet::PING_HEADER => self.ping(packet, src),
            packet::PONG_HEADER => self.pong(packet),
            packet::FINDNODE_HEADER => self.find_node(packet, src),
            packet::SENDNODE_HEADER => self.send_node(packet),
            _ => println!("Header not found, dropping packet"),
        }
    }

    fn ping(&self, packet: &Packet, mut src: SocketAddr) {
        src.set_port(1024);
        self.client.pong(src, packet.cookie());
    }

    fn pong(&self, packet: &Packet) {
        let req_list = self.requests.lock().unwrap();
        for i in 0..req_list.len() {
            if req_list[i].cookie() == packet.cookie() {
                // We found the original ping
                println!("We found the original PING with cookie {}", req_list[i].cookie());
            }
        }
    }

    fn find_node(&self, packet: &Packet, mut src: SocketAddr) {
        // Extract the ID from the packet
        let mut id_bytes = [0u8; ID_BYTES];
        id_bytes.copy_from_slice(&packet.data()[..ID_BYTES]);

        // Get list of the closest nodes
        let id_list = self.bucket_list.lock().unwrap()
            .get_closest(&Id::from_bytes(&id_bytes));

        println!("{:?}", id_list);

        src.set_port(1024);
        self.client.send_node(src, packet.cookie(), &id_list);
    }

    fn send_node(&self, packet: &Packet) {
        let mut id_list: Vec<Id> = Vec::new();

        // Extract the ID from send_node
        for i in 0..DATA_SIZE/ID_BYTES {
            let mut id_bytes = [0u8; ID_BYTES];
            id_bytes.copy_from_slice(&packet.data()[i*ID_BYTES..(i+1)*ID_BYTES]);
            id_list.push(Id::from_bytes(&id_bytes))
        }

        println!("{:?}", id_list);
    }
}