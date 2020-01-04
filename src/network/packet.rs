use std::net::SocketAddr;

pub const TOTAL_SIZE: usize = 508;
const HEADER_SIZE: usize = 2;
const NUM_SIZE: usize = 2;
const COOKIE_SIZE: usize = 4;
const DATA_SIZE: usize = 500;

pub struct Packet {
    header: Vec<u8>, // Information about the contents of this message
    data: Vec<u8>, // Data inside the packet
    cookie: Vec<u8>, // To know what the other is responding to
    num: u32, // Packet number (if we are splitting it)
    src_ip: SocketAddr,
}

impl Packet {
    fn new(header: Vec<u8>,
        data: Vec<u8>,
        cookie: Vec<u8>,
        num: u32,
        src_ip: SocketAddr) -> Self {
        Packet {
            header: header,
            data: data[0..TOTAL_SIZE].to_vec(),
            cookie: cookie,
            num: num,
            src_ip: src_ip,
        }
    }
}