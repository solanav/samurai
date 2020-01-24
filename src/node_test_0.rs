use samurai::network::passive::Server;

fn main() {
    let mut server = Server::new(10);
    server.start();
}