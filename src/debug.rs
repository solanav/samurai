use std::net::{IpAddr, TcpStream};
use std::io::Write;

pub fn send_msg(debug_server_addr: IpAddr, msg: String) {
    let mut con = match TcpStream::connect(debug_server_addr) {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to connect to debug server: {:?}", e);
            return;
        }
    };

    con.write_all(msg.as_bytes());
}