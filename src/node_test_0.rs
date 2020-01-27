use samurai::server::passive::Server;
use std::time::Duration;
use std::thread;
use samurai::client::active::send_node;
use samurai::server::threadpool::ThreadPool;

fn main() {
    let tp = ThreadPool::new(4);
    tp.execute(move || println!("Hello world"));

    thread::sleep(Duration::from_secs(10));
}
