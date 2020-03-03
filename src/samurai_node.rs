use samurai::{server::passive::Server, bootstrap::file::load, client::active, debug_send};

use std::net::{TcpStream, SocketAddrV4, Ipv4Addr, SocketAddr, IpAddr};
use std::thread::sleep;
use std::time::Duration;
use std::process::exit;
use samurai::node::Node;
use samurai::id::Id;

const DEBUG_IP: &str = "192.168.35.1";
const DEBUG_PORT: u16 = 9393;
const PEER_LIST: &str = "peer_list.json";

fn main() {
    debug_send!(DEBUG_IP, DEBUG_PORT, "Node starting");

    let mut _server = Server::new();
    debug_send!(DEBUG_IP, DEBUG_PORT, "Server started");

    let peer_list = match load(PEER_LIST) {
        Ok(pl) => pl,
        Err(e) => {
            let msg = format!("Failed to load peer list: {}", e);
            debug_send!(DEBUG_IP, DEBUG_PORT, msg.as_str());
            return;
        }
    };

    debug_send!(DEBUG_IP, DEBUG_PORT, "Peer list loaded");
    sleep(Duration::from_secs(2));

    // Main loop
    loop {
        for node in peer_list.iter() {
            let mut s = match TcpStream::connect(node.addr()) {
                Ok(s) => s,
                Err(e) => {
                    let msg = format!("Error connecting [{:?}]", e);
                    debug_send!(DEBUG_IP, DEBUG_PORT, msg.as_str());
                    break;
                },
            };

            debug_send!(DEBUG_IP, DEBUG_PORT, "Sending message to {}", node.addr().ip());
            active::send_message(&mut s, &"testing".to_string());
        }

        sleep(Duration::from_secs(5));
    }
}
