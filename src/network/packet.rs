use rand::random;
use std::fmt;

// Size in bytes
pub const TOTAL_SIZE: usize = 508;
pub const DATA_SIZE: usize = 502;

// Offsets in bytes
const HEADER_OFFSET: usize = 0;
const COOKIE_OFFSET: usize = 2;
const DATA_OFFSET: usize = 6;

// Headers
pub const PING_HEADER: u16 = 0;
pub const PONG_HEADER: u16 = 1;
pub const FINDNODE_HEADER: u16 = 2;
pub const SENDNODE_HEADER: u16 = 3;
pub const SENDMSG_HEADER: u16 = 4;
pub const SENDECHO_HEADER: u16 = 5;

pub struct Packet {
    header: u16,           // Information about the contents of this message
    cookie: u32,           // To know what the other is responding to
    data: [u8; DATA_SIZE], // Data inside the packet
}

impl Packet {
    pub fn new(header: u16, cookie: u32, data: &[u8; DATA_SIZE]) -> Self {
        Packet {
            header,
            cookie,
            data: *data,
        }
    }

    pub fn new_with_cookie(header: u16, data: &[u8; DATA_SIZE]) -> Self {
        Packet {
            header,
            cookie: random::<u32>(),
            data: *data,
        }
    }

    pub fn from_bytes(buf: &[u8; TOTAL_SIZE]) -> Self {
        // Copy raw data to packet
        let mut data: [u8; DATA_SIZE] = [0; DATA_SIZE];
        data.copy_from_slice(&buf[DATA_OFFSET..DATA_OFFSET + DATA_SIZE]);

        let header = ((buf[HEADER_OFFSET] as u16) << 8) + (buf[HEADER_OFFSET + 1] as u16);

        let cookie = ((buf[COOKIE_OFFSET] as u32) << 24)
            + ((buf[COOKIE_OFFSET + 1] as u32) << 16)
            + ((buf[COOKIE_OFFSET + 2] as u32) << 8)
            + (buf[COOKIE_OFFSET + 3] as u32);

        // Return the packet
        Packet {
            header,
            cookie,
            data,
        }
    }

    pub fn as_bytes(&self) -> [u8; TOTAL_SIZE] {
        let mut buf = [0u8; TOTAL_SIZE];

        // Copy header
        buf[HEADER_OFFSET] = (self.header >> 8) as u8;
        buf[HEADER_OFFSET + 1] = self.header as u8;
        // Copy cookie
        buf[COOKIE_OFFSET] = (self.cookie >> 24) as u8;
        buf[COOKIE_OFFSET + 1] = (self.cookie >> 16) as u8;
        buf[COOKIE_OFFSET + 2] = (self.cookie >> 8) as u8;
        buf[COOKIE_OFFSET + 3] = self.cookie as u8;
        // Copy data
        for i in 0..self.data.len() {
            buf[DATA_OFFSET + i] = self.data[i];
        }

        buf
    }

    pub fn header(&self) -> u16 {
        self.header
    }

    pub fn cookie(&self) -> u32 {
        self.cookie
    }

    pub fn data(&self) -> &[u8; DATA_SIZE] {
        &self.data
    }
}

impl fmt::Debug for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Header: [{}]; Cookie: [{}];\n{:?}",
            self.header,
            self.cookie,
            self.data.to_vec()
        )
    }
}
