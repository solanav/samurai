use crate::network::packet::{self, Packet, DATA_SIZE, TOTAL_SIZE};
use std::net::TcpStream;
use crate::types::id::{Id, ID_BYTES};
use crate::types::bucket_list::BucketList;
use std::sync::{Arc, Mutex};
use std::io::Read;
use crate::network::active;

pub struct Handler {
    stream: TcpStream, // To send messages
    bucket_list: Arc<Mutex<BucketList>>, // To keep peers and add new ones
}

impl Handler {
    pub fn new(stream: TcpStream, bucket_list: Arc<Mutex<BucketList>>) -> Self {
        Handler {
            stream,
            bucket_list,
        }
    }

    pub fn start(&mut self) {
        // Keep going until we get a timeout
        loop {
            let mut buf= [0u8; TOTAL_SIZE];

            if let Err(e) = self.stream.read(&mut buf) {
                println!("Timeout on handler, closing connection");
                break;
            };

            let packet = Packet::from_bytes(&buf);

            match packet.header() {
                packet::PING_HEADER => self.ping(&packet),
                packet::PONG_HEADER => self.pong(&packet),
                packet::FINDNODE_HEADER => self.find_node(&packet),
                packet::SENDNODE_HEADER => self.send_node(&packet),
                packet::SENDMSG_HEADER => self.send_message(&packet),
                packet::SENDECHO_HEADER => self.send_echo(&packet),
                _ => println!("Header not found, dropping packet"),
            }
        }
    }

    fn ping(&mut self, packet: &Packet) {
        active::pong(&mut self.stream, packet.cookie());
    }

    fn pong(&self, packet: &Packet) {
        println!("received pong {:?}", packet);
    }

    fn find_node(&mut self, packet: &Packet) {
        // Extract the ID from the packet
        let mut id_bytes = [0u8; ID_BYTES];
        id_bytes.copy_from_slice(&packet.data()[..ID_BYTES]);

        // Get list of the closest nodes
        let id_list = self.bucket_list.lock().unwrap()
            .get_closest(&Id::from_bytes(&id_bytes));

        println!("{:?}", id_list);

        active::send_node(&mut self.stream, packet.cookie(), &id_list);
    }

    fn send_node(&mut self, packet: &Packet) {
        let mut id_list: Vec<Id> = Vec::new();

        // Extract the ID from send_node
        for i in 0..DATA_SIZE/ID_BYTES {
            let mut id_bytes = [0u8; ID_BYTES];
            id_bytes.copy_from_slice(&packet.data()[i*ID_BYTES..(i+1)*ID_BYTES]);
            id_list.push(Id::from_bytes(&id_bytes))
        }

        println!("{:?}", id_list);
    }

    fn send_message(&mut self, packet: &Packet) {
        active::send_echo(&mut self.stream, packet.cookie(), packet.data());
    }

    fn send_echo(&mut self, packet: &Packet) {
        println!("{:?}", packet.data().to_vec());
    }
}