use std::fs::File;
use std::io::prelude::*;
use samurai::server::passive::Server;

#[test]
fn test_file() {
    let mut file = File::create("peer_list.json").unwrap();
    file.write_all(b"[{\"id\":{\"high\":300297144618627298171492249439993566512,\"low\":16942759680083732508025599418331224582},\"local\":false,\"addr\":\"127.0.0.1:1025\"}]").unwrap();
    file.sync_all().unwrap();

    let server = Server::new();
    server.load("peer_list.json");
}