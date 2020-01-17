use std::net::UdpSocket;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use crate::network::active::Client;
use crate::network::passive::Server;
use std::time::Duration;

pub mod packet;
pub mod active;
pub mod passive;
pub mod handler;

const CLIENT_NUMNODES: usize = 10;

pub fn init_network() -> (Client, Server) {
    // Create socket for client and server
    let mut rng = rand::thread_rng();
    let socket;
    let internal_port;
    loop {
        let p = rng.gen_range(1024, 65535);
        match UdpSocket::bind(
            format!("127.0.0.1:{}", 1025)) {
            Ok(s) => {
                s.set_read_timeout(Some(Duration::new(15, 0)));
                socket = Arc::new(Mutex::new(s));
                internal_port = p;
                break;
            },
            Err(_) => {},
        }
    }

    println!("{:?}", internal_port);
    let requests = Arc::new(Mutex::new(VecDeque::new()));

    // Create client
    let client = Client::new(
        CLIENT_NUMNODES,
        Arc::clone(&requests),
        Arc::clone(&socket),
    );

    // Create server
    let server = Server::new(
        CLIENT_NUMNODES,
        Arc::clone(&requests),
        Arc::clone(&socket),
        internal_port,
    );

    (client, server)
}