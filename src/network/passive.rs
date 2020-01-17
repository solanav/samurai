use crate::network::handler::Handler;
use crate::network::packet::{Packet, TOTAL_SIZE};
use crate::types::bucket_list::BucketList;
use std::net::{UdpSocket, Ipv4Addr, SocketAddrV4, IpAddr};
use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use igd;
use get_if_addrs;
use std::sync::mpsc;

const STOP_SERVER: u8 = 0;

pub struct Server {
    socket: Arc<Mutex<UdpSocket>>, // Server's socket (Same as client)
    num_nodes: Arc<Mutex<usize>>, // Number of nodes we send when find_node is received
    bucket_list: Arc<Mutex<BucketList>>, // List of buckets
    requests: Arc<Mutex<ReqList>>, // List of requests
    message_sender: Option<mpsc::Sender<u8>>,
    port: u16, // External port
}

pub type ReqList = VecDeque<Packet>;

impl Server {
    pub fn new(
        num_nodes: usize,
        requests: Arc<Mutex<ReqList>>,
        socket: Arc<Mutex<UdpSocket>>,
        internal_port: u16
    ) -> Self {
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
                    10,
                    "Samurai") {
                    Err(ref e) => panic!("Error getting port: {}", e),
                    Ok(port) => external_port = port,
                }
            }
        }

        // Create the bucket list
        let bucket_list = BucketList::new();

        Server {
            socket,
            num_nodes: Arc::new(Mutex::new(num_nodes)),
            bucket_list: Arc::new(Mutex::new(bucket_list)),
            requests,
            message_sender: None,
            port: external_port,
        }
    }

    pub fn start(&mut self) {
        // Create copy for the server thread
        let main_socket = Arc::clone(&self.socket);
        let handler_socket = Arc::clone(&self.socket);

        let num_nodes = *self.num_nodes.lock().unwrap();
        let requests = Arc::clone(&self.requests);
        let bucket_list = Arc::clone(&self.bucket_list);
        let message_receiver;

        let mp = mpsc::channel();
        self.message_sender = Some(mp.0);
        message_receiver = mp.1;

        // Launch thread with main loop
        thread::spawn(move || {
            let mut buf = [0; TOTAL_SIZE];
            let handler = Handler::new(
                num_nodes,
                requests,
                bucket_list,
                handler_socket,
            );

            // Msg handler loop
            loop {
                match message_receiver.try_recv() {
                    Ok(msg) => if msg == STOP_SERVER { break; },
                    Err(_) => {},
                }

                let (_number_of_bytes, src_addr) = match main_socket.lock().unwrap().recv_from(&mut buf) {
                    Ok((n, addr)) => (n, addr),
                    Err(e) => {
                        println!("ERROR {}", e);
                        continue;
                    },
                };

                let packet = Packet::from_bytes(&buf);
                handler.switch(&packet, src_addr);
            }
        });
    }

    pub fn stop(&self) {
        match &self.message_sender {
            Some(ms) => {
                match ms.send(STOP_SERVER) {
                    Ok(_) => {},
                    Err(e) => println!("Failed to send stop message to server {:?}", e),
                }
            },
            None => return,
        };
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}
