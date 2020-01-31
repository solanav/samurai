use samurai::server::passive::Server;
use samurai::bootstrap::random::*;

fn main() {
    let a = random_ipv6();
    println!("{}", a);
    //let _server = Server::new();
}
