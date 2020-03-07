use std::net::{TcpStream, SocketAddrV4};
use std::io::Write;

#[macro_export]
macro_rules! debug_send {
    ($msg:expr) => (
        samurai::debug::send_message(std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(192, 168, 35, 1), 9393), $msg.to_string())
    );
}

pub fn send_message(debug_server_addr: SocketAddrV4, msg: String) {
    let mut con = match TcpStream::connect(debug_server_addr) {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to connect to debug server: {:?}", e);
            return;
        }
    };

    con.write_all(msg.as_bytes());
}