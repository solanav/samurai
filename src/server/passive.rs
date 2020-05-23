use crate::{
    server::handler::Handler,
    bucket::bucket_list::BucketList,
    bootstrap::file::{save, load},
    server::{
        router_utils::open_port,
        threadpool::ThreadPool
    },
    error::FileError,
    node::Node,
};

use std::{
    net::{TcpListener, TcpStream, Ipv4Addr},
    sync::{Arc, Mutex},
    time::Duration,
    thread,
};

pub struct Server {
    thread_pool: Arc<Mutex<ThreadPool>>, // Worker pool for handlers
    bucket_list: Arc<Mutex<BucketList>>, // List of buckets
    port: Option<u16>, // External port
}

impl Server {
    pub fn new(local_ip: Ipv4Addr, bucket_list: Arc<Mutex<BucketList>>) -> Self {
        // Start listening on a random internal port
        //let mut rng = rand::thread_rng();
        let listener;
        let local_port;
        loop {
            let p = 1024; // TODO restore random >> rng.gen_range(1024, 65535);

            if let Ok(l) = TcpListener::bind(format!("{}:{}", local_ip, p)) {
                listener = l;
                local_port = p;
                break;
            }
        }

        let port = match open_port(local_ip, local_port) {
            Ok(p) => Some(p),
            Err(e) => {
                println!("Error opening port: {}", e);
                None
            }
        };

        println!("Bound to {}:{}", local_ip, local_port);

        // Clone bucket list for thread
        let bucket_list_thread = Arc::clone(&bucket_list);

        // Create the threadpool for launching handlers
        let thread_pool = Arc::new(Mutex::new(ThreadPool::new(4)));
        let thread_pool_thread = Arc::clone(&thread_pool);

        println!("Created bucket list and threadpool");

        thread::spawn(move || {
            for s in listener.incoming() {
                if let Ok(stream) = s {
                    // Ignore messages to self TODO: change this to check if local is true in the node
                    if stream.peer_addr().unwrap().ip() == local_ip {
                        break;
                    }

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

        println!("Server launched ok");

        Server {
            thread_pool,
            bucket_list,
            port,
        }
    }

    pub fn add_node(&mut self, node: Node) {
        let mut self_bucket_list = self.bucket_list.lock().unwrap();
        self_bucket_list.add_node(node).unwrap();
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
        let mut bl = self.bucket_list.lock().unwrap();
        let temp_node_list = bl.node_list();

        let mut node_list = Vec::new();
        for node in temp_node_list {
            node_list.push(&*node)
        }
        let _ = save(path, node_list);
    }

    pub fn load(&self, path: &str) -> Result<(), FileError> {
        let node_list = load(path)?;

        let mut self_bucket_list = self.bucket_list.lock().unwrap();

        for node in node_list {
            self_bucket_list.add_node(node).unwrap();
        }

        Ok(())
    }
}
