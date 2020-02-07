use samurai::server::passive::Server;
use samurai::bootstrap::file::load;
use std::net::{TcpStream, SocketAddrV4, Ipv4Addr};
use samurai::client::active::send_message;

fn main() {
    let _server = Server::new();
    let peer_list = load("peer_list.txt");

    for _peer in peer_list {
        let mut s = TcpStream::connect(SocketAddrV4::new(Ipv4Addr::new(192,168,2,2), 1024)).unwrap();
        send_message(&mut s, &"testing".to_string());
    }
}
