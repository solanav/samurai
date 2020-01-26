use crate::server::handler::Handler;
use crate::types::bucket_list::BucketList;
use crate::bootstrap::file::{save, load};

use std::net::TcpListener;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;
use crate::server::router_utils::{open_port, local_ip};

const MAX_BUCKETS: usize = 10;
const BUCKET_SIZE: usize = 10;
const BTST_FILE: &str = "peers.json";

pub struct Server {
    bucket_list: Arc<Mutex<BucketList>>, // List of buckets
    port: u16, // External port
}

impl Server {
    pub fn new() -> Self {
        // Start listening on a random internal port
        let mut rng = rand::thread_rng();
        let listener;
        let local_port;
        loop {
            let p = rng.gen_range(1024, 65535);

            if let Ok(l) = TcpListener::bind(format!("127.0.0.1:{}", p)) {
                listener = l;
                local_port = p;
                break;
            }
        }

        let local_ip = local_ip();
        let port = open_port(local_ip, local_port);

        println!("internal port > {}", local_port);

        // Create the bucket list
        let bucket_list = Arc::new(Mutex::new(BucketList::new(MAX_BUCKETS, BUCKET_SIZE)));
        let bucket_list_thread = bucket_list.clone();

        thread::spawn(move || {
            for s in listener.incoming() {
                if let Ok(stream) = s {
                    let bl = bucket_list_thread.clone();

                    thread::spawn(|| {
                        println!("Accepted connection with {}", stream.peer_addr().unwrap());
                        stream.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
                        let mut handler = Handler::new(stream, bl);
                        handler.start();
                    });
                }
            };
        });

        Server {
            bucket_list,
            port,
        }
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn save(&self, path: &str) {
        let node_list = self.bucket_list.lock().unwrap().node_list();
        save(path, &node_list);
    }

    pub fn load(&self, path: &str) {
        let file_bucket_list = load(path);
        let mut self_bucket_list = self.bucket_list.lock().unwrap();
        for node in file_bucket_list.iter() {
            self_bucket_list.add_node(node).unwrap();
        }
    }
}
