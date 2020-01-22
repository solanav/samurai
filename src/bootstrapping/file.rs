use std::fs::File;
use std::io::{BufWriter, BufReader};
use crate::types::node::Node;

pub fn save(path: &str, node_list: &Vec<Node>) {
    let file = match File::create(&path) {
        Err(e) => panic!("Failed to open {}: {}", path, e),
        Ok(file) => file,
    };

    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, node_list);
}

pub fn load(path: &str) -> Vec<Node> {
    let file = match File::open(&path) {
        Err(e) => panic!("Failed to open {}: {}", path, e),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}