use crate::{
    packet::{self, Packet, DATA_SIZE, TOTAL_SIZE},
    id::{Id, ID_BYTES},
    bucket::bucket_list::BucketList,
    node::Node,
};
use std::{
    net::{TcpStream, SocketAddr},
    sync::{Arc, Mutex},
    io::Read,
};

pub struct Handler {
    stream: TcpStream, // To send messages
    addr: SocketAddr, // Address of the sender
    bucket_list: Arc<Mutex<BucketList>>, // To keep peers and add new ones
}

impl Handler {
    pub fn new(
        stream: TcpStream,
        bucket_list: Arc<Mutex<BucketList>>,
    ) -> Self {
        let addr = match stream.peer_addr() {
            Ok(addr) => addr,
            Err(e) => panic!("Failed to get addr out of TcpStream: {}", e),
        };

        Handler {
            stream,
            addr,
            bucket_list,
        }
    }

    pub fn start(&mut self) {
        // Keep going until we get a timeout
        loop {
            let mut buf= [0u8; TOTAL_SIZE];

            if let Err(_) = self.stream.read(&mut buf) {
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
        // Check if we know this node already
        let mut bl = self.bucket_list.lock().unwrap();
        let node = bl.get_node(self.addr);

        match node {
            Some(n) => {
                n.pong(packet.cookie())
            },
            None => {
                // Create the new node
                let mut n = Node::new(Id::zero(), false, self.addr);

                n.pong(packet.cookie());

                // Send the new node the pong
                if let Err(e) = self.bucket_list.lock().unwrap().add_node(n) {
                    println!("Failed to add new node to bucket_list: {}", e);
                }
            }
        };
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

        // Check if we know this node already
        let mut bl = self.bucket_list.lock().unwrap();
        let node = bl.get_node(self.addr);

        match node {
            Some(n) => n.send_node(packet.cookie(), &id_list),
            None => {
                let mut n = Node::new(Id::zero(), false, self.addr);
                n.send_node(packet.cookie(), &id_list);
            },
        };
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
        // Check if we know this node already
        let mut bl = self.bucket_list.lock().unwrap();
        let node = bl.get_node(self.addr);

        match node {
            Some(n) => n.send_echo(packet.cookie(), packet.data()),
            None => {
                // Create the new node
                let mut n = Node::new(Id::zero(), false, self.addr);
                n.send_echo(packet.cookie(), packet.data());
            }
        };

        println!("Echo from {} [", self.stream.peer_addr().unwrap().ip());
        for i in packet.data().iter() {
            print!("{}", *i as char)
        }
        println!("]");
    }

    fn send_echo(&mut self, packet: &Packet) {
        for i in packet.data().iter() {
            print!("{}", *i as char)
        }
        println!("");
    }
}