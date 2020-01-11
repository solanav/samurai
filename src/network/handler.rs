use crate::network::active::Client;
use crate::network::packet::{self, Packet, DATA_SIZE};
use std::net::SocketAddr;
use crate::kbucket::id::{Id, ID_BYTES};
use std::process::id;

pub struct Handler {
    client: Client, // Used to respond to messages
}

impl Handler {
    pub fn new(num_nodes: usize) -> Self {
        Handler {
            client: Client::new(num_nodes),
        }
    }

    pub fn switch(&self, packet: &Packet, src: SocketAddr) {
        match packet.header() {
            packet::PING_HEADER => self.ping(packet, src),
            packet::PONG_HEADER => self.pong(packet),
            packet::FINDNODE_HEADER => self.find_node(packet, src),
            packet::SENDNODE_HEADER => self.send_node(packet, src),
            _ => println!("Header not found, dropping packet"),
        }
    }

    fn ping(&self, packet: &Packet, mut src: SocketAddr) {
        println!("RECV PING FROM {}", src);
        src.set_port(4321);
        self.client.pong(src, packet.cookie());
    }

    fn pong(&self, packet: &Packet) {
        println!("RECV PONG\n");
    }

    fn find_node(&self, packet: &Packet, mut src: SocketAddr) {
        let mut id_bytes = [0u8; ID_BYTES];
        id_bytes.copy_from_slice(&packet.data()[..ID_BYTES]);
        println!("RECV FINDNODE {:?}", Id::from_bytes(&id_bytes));

        let mut id_list= Vec::new();
        src.set_port(4321);
        for i in 0..self.client.num_nodes() {
            let id = Id::rand();
            println!("{:?}", id);
            id_list.push(id);
        }
        self.client.send_node(src, packet.cookie(), &id_list);
    }

    fn send_node(&self, packet: &Packet, mut src: SocketAddr) {
        let mut id_list: Vec<Id> = Vec::new();

        for i in 0..DATA_SIZE/ID_BYTES {
            let mut id_bytes = [0u8; ID_BYTES];
            id_bytes.copy_from_slice(&packet.data()[i*ID_BYTES..(i+1)*ID_BYTES]);
            id_list.push(Id::from_bytes(&id_bytes))
        }

        println!("RECV SENDNODE {:?}", id_list);
    }
}