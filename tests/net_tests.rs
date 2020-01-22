use samurai::network::init_network;
use samurai::network::packet::{Packet, TOTAL_SIZE};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::thread::sleep;
use samurai::types::id::Id;
use samurai::bootstrapping::file::save;

#[test] #[ignore]
fn test_sending() {
    let (client, mut server) = init_network();
    server.start();

    // Send UDP packet to the server
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1025);

    client.ping(addr);
    sleep(std::time::Duration::new(2, 0));

    client.find_node(addr, &Id::rand());
    sleep(std::time::Duration::new(2, 0));

    server.save("test.json");
    server.load("test.json");

    server.stop();
}

#[test] #[ignore]
fn test_packet() {
    let buf = [0u8; TOTAL_SIZE];

    let packet = Packet::from_bytes(&buf);
    let packet2 = Packet::from_bytes(&packet.as_bytes());

    assert_eq!(packet.header(), packet2.header());
    assert_eq!(packet.cookie(), packet2.cookie());
    
    for i in 0..packet.data().len() {
        assert_eq!(packet.data()[i], packet2.data()[i]);
    }
}