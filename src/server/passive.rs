use crate::server::handler::Handler;
use crate::bucket::bucket_list::BucketList;
use crate::bootstrap::file::{save, load};

use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;
use crate::server::router_utils::{open_port, local_ip};
use crate::server::threadpool::ThreadPool;
use crate::error::FileError;

const MAX_BUCKETS: usize = 10;
const BUCKET_SIZE: usize = 10;

pub struct Server {
    thread_pool: Arc<Mutex<ThreadPool>>, // Worker pool for handlers
    bucket_list: Arc<Mutex<BucketList>>, // List of buckets
    port: Option<u16>, // External port
}

impl Server {
    pub fn new() -> Self {
        // Start listening on a random internal port
        let mut rng = rand::thread_rng();
        let listener;
        let local_port;
        loop {
            let p = 1024; // TODO restore random >> rng.gen_range(1024, 65535);

            if let Ok(l) = TcpListener::bind(format!("127.0.0.1:{}", p)) {
                listener = l;
                local_port = p;
                break;
            }
        }

        let local_ip = local_ip();
        let port = match open_port(local_ip, local_port) {
            Ok(p) => Some(p),
            Err(e) => {
                println!("Error opening port: {}", e);
                None
            }
        };

        println!("internal port > {}", local_port);

        // Create the bucket list
        let bucket_list = Arc::new(Mutex::new(BucketList::new(MAX_BUCKETS, BUCKET_SIZE)));
        let bucket_list_thread = Arc::clone(&bucket_list);

        // Create the threadpool for launching handlers
        let thread_pool = Arc::new(Mutex::new(ThreadPool::new(4)));
        let thread_pool_thread = Arc::clone(&thread_pool);

        thread::spawn(move || {
            for s in listener.incoming() {
                if let Ok(stream) = s {
                    // TODO: remove this and use queue_stream
                    let bl = bucket_list_thread.clone();

                    thread_pool_thread.lock().unwrap().execute(move || {
                        if let Err(e) = stream.set_read_timeout(Some(Duration::from_secs(10))) {
                            println!("Failed to set read timeout for tcpstream: {:?}", e);
                        }
                        let mut handler = Handler::new(stream, bl);
                        handler.start();
                    });
                }
            };
        });

        Server {
            thread_pool,
            bucket_list,
            port,
        }
    }

    pub fn queue_stream(&self, stream: TcpStream) {
        let bl = Arc::clone(&self.bucket_list);

        self.thread_pool.lock().unwrap().execute(move || {
            println!("Accepted connection with {}", stream.peer_addr().unwrap());
            if let Err(e) = stream.set_read_timeout(Some(Duration::from_secs(10))) {
                println!("Failed to set read timeout for tcpstream: {:?}", e);
            }
            let mut handler = Handler::new(stream, bl);
            handler.start();
        });
    }

    pub fn port(&self) -> Option<u16> {
        self.port
    }

    pub fn save(&self, path: &str) {
        let node_list = self.bucket_list.lock().unwrap().node_list();
        let _ = save(path, &node_list);
    }

    pub fn load(&self, path: &str) -> Result<(), FileError> {
        let file_bucket_list = load(path)?;

        let mut self_bucket_list = self.bucket_list.lock().unwrap();
        for node in file_bucket_list.iter() {
            self_bucket_list.add_node(node).unwrap();
        }

        Ok(())
    }
}
