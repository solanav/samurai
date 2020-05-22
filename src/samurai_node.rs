use samurai::{
    server::passive::Server,
    bootstrap::file::load,
    client::active,
    config::ConfigData,
    debug::DebugServer,
};

use std::{
    net::{TcpStream, IpAddr},
    thread::sleep,
    time::Duration,
};
use samurai::client::active::Active;
use std::sync::{Mutex, Arc};
use samurai::node::Node;
use samurai::bucket::bucket_list::BucketList;

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
