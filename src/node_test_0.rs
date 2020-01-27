use samurai::server::passive::Server;
use std::time::Duration;
use std::thread;
use samurai::client::active::{send_node, ping};
use samurai::server::threadpool::ThreadPool;
use std::net::{TcpStream, Ipv4Addr, IpAddr, SocketAddr};

fn main() {
    let server = Server::new();
}
