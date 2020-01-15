use crate::network::handler::Handler;
use crate::network::packet::{Packet, TOTAL_SIZE};
use crate::types::bucket_list::BucketList;
use std::net::{UdpSocket, Ipv4Addr, SocketAddrV4, IpAddr};
use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use igd;
use get_if_addrs;

pub struct Server {
    socket: Arc<Mutex<UdpSocket>>, // Server's socket
    num_nodes: Arc<Mutex<usize>>, // Number of nodes we send when find_node is received
    bucket_list: Arc<Mutex<BucketList>>, // List of buckets
    requests: Arc<Mutex<ReqList>>, // List of requests
    port: u16, // External port
}

pub type ReqList = VecDeque<Packet>;

impl Server {
    pub fn new(num_nodes: usize, requests: Arc<Mutex<ReqList>>, bucket_list: BucketList) -> Self {
        // Look for a random local port that is not being used
        let socket: UdpSocket;
        let internal_port: u16;
        'outer: loop {
            for p in 1024..65535 {
                match UdpSocket::bind(format!("127.0.0.1:{}", p)) {
                    Ok(s) => {
                        socket = s;
                        internal_port = p;
                        break 'outer;
                    }
                    Err(_) => {}
                };
            }
        }

        // Get local IP
        let mut local_ip: Option<Ipv4Addr> = None;
        let ip_list = get_if_addrs::get_if_addrs().unwrap();
        for ip in ip_list.iter() {
            match ip.ip() {
                IpAddr::V4(ip) => {
                    local_ip = Some(ip);
                    break;
                },
                IpAddr::V6(_) => {},
            };
        }
        if local_ip.is_none() {
            panic!("Failed to get local IPv4");
        }

        // Get a random external port with UPnP
        let external_port: u16;
        match igd::search_gateway(Default::default()) {
            Err(ref err) => panic!("Error: {}", err),
            Ok(gateway) => {
                match gateway.add_any_port(
                    igd::PortMappingProtocol::UDP,
                    SocketAddrV4::new(local_ip.unwrap(), internal_port),
                    60,
                    "Samurai") {
                    Err(ref e) => panic!("Error getting port: {}", e),
                    Ok(port) => external_port = port,
                }
            }
        }

        Server {
            socket: Arc::new(Mutex::new(socket)),
            num_nodes: Arc::new(Mutex::new(num_nodes)),
            bucket_list: Arc::new(Mutex::new(bucket_list)),
            requests,
            port: external_port,
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
