use samurai::server::passive::Server;
use samurai::bootstrap::file::load;
use std::net::{TcpStream, SocketAddrV4, Ipv4Addr};
use samurai::client::active::send_message;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let _server = Server::new();
    let peer_list = load("peer_list.txt");

    let mut s = match TcpStream::connect(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 1024)) {
        Ok(s) => s,
        Err(e) => {
            println!("Error connecting {:?}", e);
            sleep(Duration::from_secs(5));
            return;
        },
    };

    send_message(&mut s, &"testing".to_string());
    sleep(Duration::from_secs(5));
}
