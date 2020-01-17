use crate::network::active::Client;
use crate::network::packet::{self, Packet, DATA_SIZE};
use std::net::{SocketAddr, UdpSocket};
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
    pub fn new(num_nodes: usize,
        requests: Arc<Mutex<ReqList>>,
        bucket_list: Arc<Mutex<BucketList>>,
        socket: Arc<Mutex<UdpSocket>>,
    ) -> Self {
        Handler {
            client: Client::new(num_nodes, Arc::clone(&requests), socket),
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
            packet::SENDMSG_HEADER => self.send_message(packet, src),
            packet::SENDECHO_HEADER => self.send_echo(packet),
            _ => println!("Header not found, dropping packet"),
        }
    }

    fn ping(&self, packet: &Packet, src: SocketAddr) {
        self.client.pong(src, packet.cookie());
    }

    fn pong(&self, packet: &Packet) {
        let mut req_list = self.requests.lock().unwrap();
        let mut rm_list = Vec::new();
        for i in 0..req_list.len() {
            if req_list[i].cookie() == packet.cookie() {
                rm_list.push(i);
                println!("We found the original PING with cookie {}", req_list[i].cookie());
                break;
            }
        }

        for i in rm_list {
            req_list.remove(i);
        }
    }

    fn find_node(&self, packet: &Packet, src: SocketAddr) {
        // Extract the ID from the packet
        let mut id_bytes = [0u8; ID_BYTES];
        id_bytes.copy_from_slice(&packet.data()[..ID_BYTES]);

        // Get list of the closest nodes
        let id_list = self.bucket_list.lock().unwrap()
            .get_closest(&Id::from_bytes(&id_bytes));

        println!("{:?}", id_list);

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

    fn send_message(&self, packet: &Packet, src: SocketAddr) {
        self.client.send_echo(src,packet.cookie(), packet.data());
    }

    fn send_echo(&self, packet: &Packet) {
        println!("{:?}", packet.data().to_vec());
    }
}