extern crate hex;

use crate::node::Node;

pub struct Bucket {
    node_list: Vec<Node>,
    contains_u: bool,
    start_id: Vec<u8>,
    end_id: Vec<u8>,
}

impl Bucket {
    pub fn new() -> Bucket {
        Bucket {
            node_list: Vec::new(),
            contains_u: true,
            start_id: vec![0; 20],
            end_id: vec![255; 20],
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.node_list.push(node);
    }
}