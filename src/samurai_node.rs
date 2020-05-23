use samurai::{
    server::passive::Server,
    bootstrap::file::load,
    config::ConfigData,
    debug::DebugServer,
    node::Node,
    bucket::bucket_list::BucketList,
};

use std::{
    net::{TcpStream, IpAddr},
    thread::sleep,
    time::Duration,
    sync::{Mutex, Arc},
};

const MAX_BUCKETS: usize = 10;
const BUCKET_SIZE: usize = 10;
const PEER_LIST: &str = "peer_list.json";

fn main() {
    // Read the config
    let conf: ConfigData = samurai::config::read_config();

    // Connect to the debug server
    let mut debug: DebugServer = DebugServer::new(IpAddr::from(conf.debug_ip), conf.debug_port);
    debug.send_message("Connected to debug server");

    // Load the peer list file
    let mut node_list: Vec<Node> = match load(PEER_LIST) {
        Ok(pl) => pl,
        Err(e) => {
            debug.send_message(format!("Failed to load peer list: {}", e));
            return;
        }
    };
    debug.send_message("Peer list loaded");

    // Create the bucket list
    let bucket_list = Arc::new(Mutex::new(BucketList::new(MAX_BUCKETS, BUCKET_SIZE)));

    // Start the server
    let mut _server = Server::new(conf.bind_ip, Arc::clone(&bucket_list));
    debug.send_message("Server started");

    sleep(Duration::from_secs(2));

    println!("{:?}", bucket_list.lock().unwrap())
}
