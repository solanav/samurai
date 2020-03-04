use samurai::{server::passive::Server, bootstrap::file::load, client::active, debug_send};

use std::net::{TcpStream, SocketAddrV4, Ipv4Addr, SocketAddr, IpAddr};
use std::thread::sleep;
use std::time::Duration;
use std::process::exit;
use samurai::node::Node;
use samurai::id::Id;
use samurai::config::ConfigData;

const PEER_LIST: &str = "peer_list.json";

fn main() {
    let conf: ConfigData = samurai::config::read_config();
    debug_send!("Config read ok");

    let mut _server = Server::new(conf.ip);
    debug_send!("Server started");

    let peer_list = match load(PEER_LIST) {
        Ok(pl) => pl,
        Err(e) => {
            let msg = format!("Failed to load peer list: {}", e);
            debug_send!(msg.as_str());
            return;
        }
    };

    debug_send!("Peer list loaded");
    sleep(Duration::from_secs(2));

    // Main loop
    loop {
        for node in peer_list.iter() {
            if node.addr().ip() == conf.ip {
                continue;
            }

            debug_send!(format!("Connecting to {}", node.addr().ip()).as_str());

            let mut s = match TcpStream::connect_timeout(&node.addr(), Duration::from_secs(1)) {
                Ok(s) => s,
                Err(e) => {
                    let msg = format!("Error connecting [{:?}]", e);
                    debug_send!(msg.as_str());
                    continue;
                },
            };

            debug_send!(format!("Sending message to {}", node.addr().ip()).as_str());
            active::send_message(&mut s, &"testing".to_string());
            sleep(Duration::from_secs(5));
        }

        sleep(Duration::from_secs(5));
    }
}
