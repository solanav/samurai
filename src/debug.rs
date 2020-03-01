use std::net::{IpAddr, TcpStream, SocketAddrV4, Shutdown, Ipv4Addr};
use std::io::Write;

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