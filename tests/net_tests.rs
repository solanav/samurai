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
    let buf: &[u8] = &[1, 2, 3];
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4321);
    
    //client.send(addr, buf);
    //client.send(addr, buf);
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
    println!("{:?}", packet.as_bytes().to_vec());
    let packet2 = Packet::from_bytes(&packet.as_bytes());
    println!("{:?}", packet2);
}