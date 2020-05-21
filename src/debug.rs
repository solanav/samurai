use std::net::{TcpStream, IpAddr};
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

pub struct DebugServer {
    ip: IpAddr,
    port: u16,
    con: TcpStream,
}

impl DebugServer {
    pub fn new(ip: IpAddr, port: u16) -> Self {
        // Loop until we connect to the debug server
        let con;
        loop {
            if let Ok(c) = TcpStream::connect(format!("{}:{}", ip, port)) {
                con = c;
                break;
            }

            println!("Failed to connect to debug server, trying again in 5 secs...");
            sleep(Duration::from_secs(5))
        }

        Self {
            ip,
            port,
            con,
        }
    }

    pub fn send_message<T: AsRef<str>>(&mut self, msg: T) {
        // Try 5 times and then return
        for _ in 0..5 {
            let res = self.con.write_all(msg.as_ref().as_bytes());
            if let Ok(_) = res {
                return
            }

            println!("Failed to send debug message, trying again");
            sleep(Duration::from_secs(5))
        }
    }
}