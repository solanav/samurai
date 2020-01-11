use crate::network::handler::Handler;
use crate::network::packet::{Packet, TOTAL_SIZE};
use std::net::{UdpSocket, SocketAddr};
use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

pub struct Server {
    socket: UdpSocket, // Server's socket
    port: u16,         // List of packets not yet processed
    num_nodes: usize,  // Number of nodes we send when find_node is received
    requests: ReqList // List of requests
}

type ReqList = Arc<Mutex<VecDeque<(u32, fn(&Packet, SocketAddr))>>>;

impl Server {
    pub fn new(num_nodes: usize) -> Self {
        let socket: UdpSocket;
        let port: u16;
        'outer: loop {
            for p in 1024..65535 {
                match UdpSocket::bind(format!("127.0.0.1:{}", p)) {
                    Ok(s) => {
                        socket = s;
                        port = p;
                        break 'outer;
                    },
                    Err(_) => {}
                };
            }
        }

        Server {
            socket,
            num_nodes,
            port,
            requests: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn start(self) {
        thread::spawn(move || loop {
            let mut buf = [0; TOTAL_SIZE];
            let handler = Handler::new(self.num_nodes);

            let (_number_of_bytes, src_addr) = self.socket
                .recv_from(&mut buf)
                .expect("Did not receive data");

            let packet = Packet::from_bytes(&buf);

            let mut requests = self.requests.lock().unwrap();
            let mut x = 0;
            for i in (*requests).iter() {
                println!("{} ===== {:?}", x, i.0);
                x += 1;
            }

            handler.switch(&packet, src_addr);
        });
    }

    pub fn requests(&self) -> &ReqList {
        &self.requests
    }
}
