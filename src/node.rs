use std::{
    fmt,
    net::{SocketAddr, TcpStream}
};
use crate::{
    error::ActiveError,
    packet::*,
    id::{Id, ID_BYTES},
};
use std::io::Write;

#[derive(fmt::Debug)]
pub struct Node {
    id: Id, // Id of the node
    local: bool, // Is this node us?
    addr: SocketAddr, // Address
    con: Option<TcpStream>, // Socket to talk to this node
}

impl Node {
    pub fn new(id: Id, local: bool, addr: SocketAddr) -> Self {
        Node {
            id,
            local,
            addr,
            con: None,
        }
    }

    pub fn is_local(&self) -> bool {
        self.local
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn connect(&mut self) -> Result<(), ActiveError>{
        self.con = match TcpStream::connect(self.addr) {
            Ok(c) => Some(c),
            Err(e) => {
                println!("Error connecting [{:?}]", e);
                return Err(ActiveError::ConnectTimeout);
            },
        };

        Ok(())
    }

    fn send_packet(&mut self, packet: Packet) {
        match self.con {
            Some(mut c) => {
                if let Err(e) = c.write(&packet.as_bytes()) {
                    println!("Failed to send bytes [{}]", e);
                    return;
                }

                if let Err(e) = c.flush() {
                    println!("Failed to flush stream [{}]", e);
                    return;
                }
            },
            None => println!("Tried to send packet to node that is not connected"),
        }
    }

    pub fn ping(&mut self) {
        let packet = Packet::new_with_cookie(PING_HEADER, &[0; DATA_SIZE]);
        self.send_packet(packet);
    }

    pub fn pong(&mut self, cookie: u32) {
        let packet = Packet::new(PONG_HEADER, cookie, &[0; DATA_SIZE]);
        self.send_packet(packet);
    }

    pub fn find_node(&mut self, id: &Id) {
        let mut buf = [0u8; DATA_SIZE];
        let id_bytes = id.as_bytes();

        for i in 0..id_bytes.len() {
            buf[i] = id_bytes[i];
        }

        let packet = Packet::new_with_cookie(FINDNODE_HEADER, &buf);
        self.send_packet(packet);
    }

    pub fn send_node(&mut self, cookie: u32, id_list: &Vec<Id>) {
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
        self.send_packet(packet);
    }

    pub fn send_message(&mut self, msg: &String) {
        let mut buf = [0u8; DATA_SIZE];

        let mut i = 0;
        for b in msg.as_bytes().iter() {
            if i >= DATA_SIZE {
                break;
            }

            buf[i] = *b;
            i += 1;
        }

        let packet = Packet::new_with_cookie(SENDMSG_HEADER, &buf);
        self.send_packet(packet);
    }

    pub fn send_echo(&mut self, cookie: u32, buf: &[u8; DATA_SIZE]) {
        let packet = Packet::new(SENDECHO_HEADER, cookie, buf);
        self.send_packet(packet);
    }
}