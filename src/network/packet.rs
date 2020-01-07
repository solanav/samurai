use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::fmt;

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

// Headers
pub const PING_HEADER: [u8; HEADER_SIZE] = [0, 0];
pub const PONG_HEADER: [u8; HEADER_SIZE] = [0, 1];

pub struct Packet {
    header: [u8; HEADER_SIZE], // Information about the contents of this message
    cookie: [u8; COOKIE_SIZE], // To know what the other is responding to
    src_ip: SocketAddr,        // Sender address (either v4 or v6)
    num: u32,                  // Packet number (if we are splitting it)
    data: [u8; DATA_SIZE],     // Data inside the packet
}

impl Packet {
    fn new(
        header: &[u8; HEADER_SIZE],
        data: &[u8; DATA_SIZE],
        cookie: &[u8; COOKIE_SIZE],
        num: u32,
        src_ip: SocketAddr,
    ) -> Self {
        Packet {
            header: *header,
            cookie: *cookie,
            src_ip: src_ip,
            num: num,
            data: *data,
        }
    }

    pub fn from_bytes(buf: &[u8; TOTAL_SIZE]) -> Self {
        if buf.len() != TOTAL_SIZE {
            panic!("Packet has to be of size TOTAL_SIZE")
        }

        // Get the source ip
        let src_ip = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(
                buf[SRCIP_OFFSET],
                buf[SRCIP_OFFSET + 1],
                buf[SRCIP_OFFSET + 2],
                buf[SRCIP_OFFSET + 3],
            )),
            4321,
        );

        // Do the memcpy from the buffer
        let mut header: [u8; HEADER_SIZE] = [0, HEADER_SIZE as u8];
        let mut cookie: [u8; COOKIE_SIZE] = [0; COOKIE_SIZE];
        let mut data: [u8; DATA_SIZE] = [0; DATA_SIZE];

        header.copy_from_slice(&buf[HEADER_OFFSET..HEADER_OFFSET + HEADER_SIZE]);
        cookie.copy_from_slice(&buf[COOKIE_OFFSET..COOKIE_OFFSET + COOKIE_SIZE]);
        data.copy_from_slice(&buf[DATA_OFFSET..DATA_OFFSET + DATA_SIZE]);

        // Return the packet
        Packet {
            header: header,
            cookie: cookie,
            src_ip: src_ip,
            num: ((buf[NUM_OFFSET] as u32) << 8) + (buf[NUM_OFFSET + 1] as u32),
            data: data,
        }
    }

    pub fn header(&self) -> &[u8; HEADER_SIZE] {
        return &self.header;
    }
}

impl fmt::Debug for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}][{:?}][{:?}][{}]\n{:?}",
            self.src_ip,
            self.header,
            self.cookie,
            self.num,
            self.data.to_vec()
        )
    }
}