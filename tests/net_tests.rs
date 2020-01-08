use kademlia::network::active::Client;
use kademlia::network::passive::Server;
use kademlia::network::packet::{Packet, TOTAL_SIZE};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[test]
fn test_start() {
    // Start server in a new thread
    let server = Server::new(4321);
    server.start();

    // Send UDP packet to the server
    let client = Client::new();
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4321);
    
    client.ping(addr);
}

#[test]
fn test_packet() {
    let mut buf = [0; TOTAL_SIZE];
    
    buf[0] = 0;
    buf[1] = 1;
    buf[2] = 0;
    buf[3] = 1;
    buf[4] = 0;
    buf[5] = 1;
    buf[6] = 0;
    buf[7] = 1;

    let packet = Packet::from_bytes(&buf);
    let packet2 = Packet::from_bytes(&packet.as_bytes());

    assert_eq!(packet.header(), packet2.header());
    assert_eq!(packet.cookie(), packet2.cookie());
    
    for i in 0..packet.data().len() {
        assert_eq!(packet.data()[i], packet2.data()[i]);
    }
}