use crate::network::handler::Handler;
use crate::types::bucket_list::BucketList;
use crate::bootstrapping::file::{save, load};

use std::net::{Ipv4Addr, SocketAddrV4, IpAddr, TcpListener};
use igd;
use get_if_addrs;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;

static MAX_BUCKETS: usize = 10;
static BUCKET_SIZE: usize = 10;

pub struct Server {
    listener: TcpListener, // Server's socket
    bucket_list: Arc<Mutex<BucketList>>, // List of buckets
    port: u16, // External port
}

impl Server {
    pub fn new() -> Self {
        // Get internal IP
        let mut local_ip: Option<Ipv4Addr> = None;
        let ip_list = get_if_addrs::get_if_addrs().unwrap();
        for ip in ip_list.iter() {
            if let IpAddr::V4(ip) = ip.ip() {
                local_ip = Some(ip);
                break;
            };
        }
        if local_ip.is_none() {
            panic!("Failed to get local IPv4");
        }

        // Start listening on a random internal port
        let mut rng = rand::thread_rng();
        let listener;
        let internal_port;
        loop {
            let p = rng.gen_range(1024, 65535);

            if let Ok(l) = TcpListener::bind(format!("127.0.0.1:{}", p)) {
                listener = l;
                internal_port = p;
                break;
            }
        }

        // Get a random external port with UPnP redirected to our internal port
        let port: u16;
        match igd::search_gateway(Default::default()) {
            Err(ref err) => panic!("Error: {}", err),
            Ok(gateway) => {
                match gateway.add_any_port(
                    igd::PortMappingProtocol::UDP,
                    SocketAddrV4::new(local_ip.unwrap(), internal_port),
                    10,
                    "Samurai") {
                    Err(ref e) => panic!("Error getting port: {}", e),
                    Ok(p) => port = p,
                }
            }
        }

        println!("internal port > {}", internal_port);

        // Create the bucket list
        let bucket_list = BucketList::new(MAX_BUCKETS, BUCKET_SIZE);

        Server {
            listener,
            bucket_list: Arc::new(Mutex::new(bucket_list)),
            port,
        }
    }

    pub fn start(&mut self) {
        for s in self.listener.incoming() {
            if let Ok(stream) = s {
                let bucket_list = self.bucket_list.clone();

                thread::spawn(|| {
                    println!("Accepted connection with {}", stream.peer_addr().unwrap());
                    stream.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
                    let mut handler = Handler::new(stream, bucket_list);
                    handler.start();
                });
            }
        };
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn save(&self, path: &str) {
        let node_list = self.bucket_list.lock().unwrap().node_list();
        save(path, &node_list);
        println!("{:?}", self.bucket_list);
    }

    pub fn load(&self, path: &str) {
        let bucket_list = load(path);
        for node in bucket_list.iter() {
            self.bucket_list.lock().unwrap().add_node(node).unwrap();
        }
        println!("{:?}", bucket_list);
    }
}
