use crate::network::handler::Handler;
use crate::network::packet::{Packet, TOTAL_SIZE};
use std::net::UdpSocket;
use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use crate::types::bucket_list::BucketList;

pub struct Server {
    socket: Arc<Mutex<UdpSocket>>, // Server's socket
    num_nodes: Arc<Mutex<usize>>,  // Number of nodes we send when find_node is received
    bucket_list: Arc<Mutex<BucketList>>, // List of peers
    requests: Arc<Mutex<ReqList>>, // List of requests
}

pub type ReqList = VecDeque<Packet>;

impl Server {
    pub fn new(num_nodes: usize, requests: Arc<Mutex<ReqList>>, bucket_list: BucketList) -> Self {
        let socket: UdpSocket;
        'outer: loop {
            for p in 1024..65535 {
                match UdpSocket::bind(format!("127.0.0.1:{}", p)) {
                    Ok(s) => {
                        socket = s;
                        break 'outer;
                    },
                    Err(_) => {}
                };
            }
        }

        Server {
            socket: Arc::new(Mutex::new(socket)),
            num_nodes: Arc::new(Mutex::new(num_nodes)),
            bucket_list: Arc::new(Mutex::new(bucket_list)),
            requests,
        }
    }

    pub fn start(&self) {
        // Create copy for the server thread
        let socket = Arc::clone(&self.socket);
        let num_nodes = Arc::clone(&self.num_nodes);
        let requests = Arc::clone(&self.requests);
        let bucket_list = Arc::clone(&self.bucket_list);

        // Launch thread with main loop
        thread::spawn(move || loop {
            let mut buf = [0; TOTAL_SIZE];
            let handler = Handler::new(
                *num_nodes.lock().unwrap(),
                Arc::clone(&requests),
                Arc::clone(&bucket_list),
            );

            let (_number_of_bytes, src_addr) = socket.lock().unwrap()
                .recv_from(&mut buf)
                .expect("Did not receive data");

            let packet = Packet::from_bytes(&buf);
            handler.switch(&packet, src_addr);
        });
    }
}
