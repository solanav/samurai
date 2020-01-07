use kademlia::network::active::Client;
use kademlia::network::passive::Server;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[test]
fn test_start() {
    // Start server in a new thread
    let server = Server::new(4321);
    server.start();

    // Send UDP packet to the server
    let client = Client::new();
    let buf: &[u8] = &[1, 2, 3];
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4321);
    client.send(addr, buf);
}
