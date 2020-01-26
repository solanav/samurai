use samurai::server::passive::Server;
use std::time::Duration;
use std::thread;
use samurai::client::active::send_node;

fn main() {
    // Start listening
    let server = Server::new();

    thread::sleep(Duration::from_secs(20));
    server.save("test.json");
    thread::sleep(Duration::from_secs(20));
}
