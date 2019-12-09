extern crate hex;
use std::fmt;
use crate::node::Node;

pub struct Bucket {
    node_list: Vec<Node>,
    contains_u: bool,
    start_id: Vec<u8>,
    end_id: Vec<u8>,
    max_nodes: u32,
}

impl Bucket {
    pub fn new(max_nodes: u32) -> Bucket {
        Bucket {
            node_list: Vec::new(),
            contains_u: true,
            start_id: vec![0; 20],
            end_id: vec![255; 20],
            max_nodes: max_nodes,
        }
    }

    pub fn divide(&mut self) -> Result<(), &'static str> {
        if self.contains_u == false {
            return Err("This bucket does not contain the self")
        }
        
        Ok(())
    }

    pub fn add_node(&mut self, node: Node) -> Result<(), &'static str> {
        if self.node_list.len() >= self.max_nodes as usize {
            return Err("This bucket is already full");
        }

        self.node_list.push(node);
        Ok(())
    }
}

impl fmt::Debug for Bucket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = format!("{:?}\t{:?}\n\t{:?}\n", self.contains_u, self.start_id, self.end_id);

        for node in self.node_list.iter() {
            output = format!("{}\n\t{:?}", output, node);
        }

        write!(f, "{}", output)
    }
}