use crate::{
    packet::{self, Packet, DATA_SIZE, TOTAL_SIZE},
    id::{Id, ID_BYTES},
    bucket::bucket_list::BucketList,
    node::Node,
};
use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
};
use std::net::SocketAddr;
use std::io::Read;

pub struct Handler {
    addr: SocketAddr, // Address of the node
    bucket_list: Arc<Mutex<BucketList>>, // To keep peers and add new ones
}

impl Handler {
    pub fn new(stream: TcpStream, bucket_list: Arc<Mutex<BucketList>>) -> Self {
        // Get the addr from the stream
        let addr = match stream.peer_addr() {
            Ok(a) => a,
            Err(e) => {
                panic!("Failed to extract addr from stream: {}", e);
            },
        };

        // Add the new peer to bucket_list TODO: this is not good tho its ok for now
        {
            let mut bl = bucket_list.lock().unwrap();
            let node = bl.get_node(addr);

            match node {
                Some(n) => {
                    n.set_con(stream);
                },
                None => {
                    // Create the new node
                    let mut n = Node::new(Id::zero(), false, addr);

                    n.set_con(stream);

                    // Add new node to bucket list
                    if let Err(e) = bl.add_node(n) {
                        println!("Failed to add new node to bucket_list: {}", e);
                    }
                }
            };
        }

        Handler {
            addr,
            bucket_list,
        }
    }

    pub fn start(&mut self) {
        // Keep going until we get a timeout
        loop {
            let packet = {
                let mut bl = self.bucket_list.lock().unwrap();
                let node = bl.get_node(self.addr).unwrap();

                let mut buf= [0u8; TOTAL_SIZE];

                let con = match node.con() {
                    Some(c) => c,
                    None => {
                        println!("No connection, stopping handler");
                        break;
                    },
                };

                if let Err(_) = con.read(&mut buf) {
                    println!("Timeout on handler, closing connection");
                    break;
                };

                Packet::from_bytes(&buf)
            };

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
        let mut bl = self.bucket_list.lock().unwrap();
        let node = bl.get_node(self.addr).unwrap();

        node.pong(packet.cookie());
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

        let mut bl = self.bucket_list.lock().unwrap();
        let node = bl.get_node(self.addr).unwrap();

        node.send_node(packet.cookie(), &id_list);
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
        let mut bl = self.bucket_list.lock().unwrap();
        let node = bl.get_node(self.addr).unwrap();

        node.send_echo(packet.cookie(), packet.data());

        println!("Echo from {} [", self.addr);
        for i in packet.data().iter() {
            print!("{}", *i as char)
        }
        println!("]");
    }

    fn send_echo(&self, packet: &Packet) {
        for i in packet.data().iter() {
            print!("{}", *i as char)
        }
        println!("");
    }
}