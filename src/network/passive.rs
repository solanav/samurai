use crate::network::handler::Handler;
use crate::network::packet::{Packet, TOTAL_SIZE};
use std::net::{UdpSocket, SocketAddr};
use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

pub struct Server {
    socket: Arc<Mutex<UdpSocket>>, // Server's socket
    num_nodes: Arc<Mutex<usize>>,  // Number of nodes we send when find_node is received
    requests: Arc<Mutex<VecDeque<(u32, fn(&Packet, SocketAddr))>>> // List of requests
}

impl Server {
    pub fn new(num_nodes: usize) -> Self {
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
            requests: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn start(&self) {
        // Create copy for the server thread
        let socket = Arc::clone(&self.socket);
        let num_nodes = Arc::clone(&self.num_nodes);
        let requests = Arc::clone(&self.requests);

        thread::spawn(move || loop {
            let mut buf = [0; TOTAL_SIZE];
            let handler = Handler::new(*num_nodes.lock().unwrap());

            let (_number_of_bytes, src_addr) = (*socket.lock().unwrap())
                .recv_from(&mut buf)
                .expect("Did not receive data");

            let packet = Packet::from_bytes(&buf);

            let mut x = 0;
            for i in (*requests.lock().unwrap()).iter() {
                println!("{} ===== {:?}", x, i.0);
                x += 1;
            }

            handler.switch(&packet, src_addr);
        });
    }

    pub fn requests(&self) -> Arc<Mutex<VecDeque<(u32, fn(&Packet, SocketAddr))>>> {
        Arc::clone(&self.requests)
    }
}
