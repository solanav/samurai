use std::fmt;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

// Size in bytes
pub const TOTAL_SIZE: usize = 508;
const HEADER_SIZE: usize = 2;
const COOKIE_SIZE: usize = 4;
const SRCIP_SIZE: usize = 4;
const NUM_SIZE: usize = 2;
const DATA_SIZE: usize = 496;

// Offsets in bytes
const HEADER_OFFSET: usize = 0;
const COOKIE_OFFSET: usize = 2;
const SRCIP_OFFSET: usize = 6;
const NUM_OFFSET: usize = 10;
const DATA_OFFSET: usize = 12;

#[derive(fmt::Debug)]
pub struct Packet {
    header: Vec<u8>,    // Information about the contents of this message
    cookie: Vec<u8>,    // To know what the other is responding to
    src_ip: SocketAddr, // Sender address (either v4 or v6)
    num: u32,           // Packet number (if we are splitting it)
    data: Vec<u8>,      // Data inside the packet
}

impl Packet {
    fn new(header: Vec<u8>, data: Vec<u8>, cookie: Vec<u8>, num: u32, src_ip: SocketAddr) -> Self {
        Packet {
            header: header,
            cookie: cookie,
            src_ip: src_ip,
            num: num,
            data: data[0..TOTAL_SIZE].to_vec(),
        }
    }

    pub fn from_bytes(buf: &[u8]) -> Self {
        if buf.len() != TOTAL_SIZE {
            panic!("Packet has to be of size TOTAL_SIZE")
        }

        let src_ip = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(
                buf[SRCIP_OFFSET],
                buf[SRCIP_OFFSET + 1],
                buf[SRCIP_OFFSET + 2],
                buf[SRCIP_OFFSET + 3],
            )),
            4321,
        );

        Packet {
            header: buf[HEADER_OFFSET..COOKIE_OFFSET].to_vec(),
            cookie: buf[COOKIE_OFFSET..SRCIP_OFFSET].to_vec(),
            src_ip: src_ip,
            num: ((buf[NUM_OFFSET] as u32) << 8) + (buf[NUM_OFFSET + 1] as u32),
            data: buf[DATA_OFFSET..TOTAL_SIZE].to_vec(),
        }
    }
}
