static TOTAL_SIZE: usize = 508;
static HEADER_SIZE: usize = 2;
static NUM_SIZE: usize = 2;
static COOKIE_SIZE: usize = 4;
static DATA_SIZE: usize = 500;

struct Packet {
    header: Vec<u8>, // Information about the contents of this message
    data: Vec<u8>, // Data inside the packet
    cookie: Vec<u8>, // To know what the other is responding to
    num: Vec<u8>, // Packet number (if we are splitting it)
    src_ip: SockAddr,
}

impl Packet {
    fn new(header: Vec<u8>,
        data: Vec<u8>,
        cookie: Vec<u8>,
        src_ip: SockAddr) -> Self {
        Packet {
            header: header,
            data: data[0..MAX_PACKET_SIZE],
            cookie: cookie,
            src_ip: src_ip,
        }
    }
}