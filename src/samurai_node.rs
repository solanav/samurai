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

const PEER_LIST: &str = "peer_list.json";

fn main() {
    // Read the config
    let conf: ConfigData = samurai::config::read_config();

    // Connect to the debug server
    let mut debug: DebugServer = DebugServer::new(IpAddr::from(conf.debug_ip), conf.debug_port);
    debug.send_message("Connected to debug server");

    // Start the server
    let mut _server = Server::new(conf.bind_ip);
    debug.send_message("Server started");


    // Load the peer list file
    let peer_list = match load(PEER_LIST) {
        Ok(pl) => pl,
        Err(e) => {
            debug.send_message(format!("Failed to load peer list: {}", e));
            return;
        }
    };

    debug.send_message("Peer list loaded");
    sleep(Duration::from_secs(2));

    // Main loop
    loop {
        for node in peer_list.iter() {
            // Skip our own ip
            if node.addr().ip() == conf.bind_ip {
                continue;
            }

            debug.send_message(format!("Connecting to {}", node.addr().ip()));

            let mut s = match TcpStream::connect_timeout(&node.addr(), Duration::from_secs(1)) {
                Ok(s) => s,
                Err(e) => {
                    let msg = format!("Error connecting [{:?}]", e);
                    debug.send_message(msg);
                    continue;
                },
            };

            debug.send_message(format!("Sending message to {}", node.addr().ip()));
            active::send_message(&mut s, &"testing".to_string());
            sleep(Duration::from_secs(5));
        }

        sleep(Duration::from_secs(5));
    }
}
