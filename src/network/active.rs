use std::net::UdpSocket;
use rand::Rng;
use std::net::SocketAddr;

pub struct Client {
    socket: UdpSocket, // Client's socket
}

impl Client {
    pub fn new() -> Self {    
        let mut rng = rand::thread_rng();
        let socket = UdpSocket::bind(format!("127.0.0.1:{}", rng.gen_range(1024, 65536)))
            .expect("couldn't bind to address");    
        
        Client {
            socket: socket,
        }
    }

    pub fn send(&self, ip: SocketAddr, buf: &[u8]) {
        self.socket.connect(format!("{}", ip))
            .expect("connect function failed");

        self.socket.send(buf)
            .expect("couldn't send message");
    }
}