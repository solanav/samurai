use std::net::{IpAddr, TcpStream, SocketAddrV4, Shutdown, Ipv4Addr};
use std::io::Write;

#[macro_export]
macro_rules! debug_send {
    ($ip:expr, $port:expr, $msg:expr) => (
        let ip: Ipv4Addr = std::str::FromStr::from_str($ip).unwrap();
        samurai::debug::send_message(SocketAddrV4::new(ip, $port), $msg.to_string())
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