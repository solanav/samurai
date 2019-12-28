use crate::kbucket::id::Id;
use crate::kbucket::node::Node;
use std::fmt;

pub struct Bucket {
    node_list: Vec<Node>,
    start_id: Id,
    end_id: Id,
    max_nodes: usize,
}

impl Bucket {
    pub fn new(max_nodes: usize, start_id: Id, end_id: Id) -> Self {
        Self {
            node_list: Vec::new(),
            start_id: start_id,
            end_id: end_id,
            max_nodes: max_nodes,
        }
    }

    pub fn add_node(&mut self, node: Node) -> Result<(), &'static str> {
        if self.node_list.len() >= self.max_nodes {
            return Err("This bucket is already full");
        }

        if self.start_id > node.id() || node.id() > self.end_id {
            return Err("This bucket should not contain that node");
        }

        self.node_list.push(node);
        Ok(())
    }

    pub fn rm_node(&mut self, id: Id) {
        match self.node_list.iter().position(|&i| i.id() == id) {
            Some(i) => {
                self.node_list.remove(i);
                ()
            }
            None => {}
        }
    }

    pub fn local(&self) -> Result<Node, &'static str> {
        for node in self.node_list.iter() {
            if node.is_local() {
                return Ok(*node);
            }
        }

        Err("Local node not found in this bucket")
    }

    pub fn divide(&mut self) -> Option<Self> {
        if self.node_list.len() >= self.max_nodes {
            return None;
        }

        if !self.local().is_err() {
            return None;
        }

        // Update the end_id
        let end_id = self.end_id;
        self.end_id = self.end_id.half();

        let mut new_bucket = Bucket::new(self.max_nodes, self.end_id + 1, end_id);

        // Try to add nodes to new bucket
        for i in 0..self.node_list.len() {
            match new_bucket.add_node(self.node_list[i]) {
                Ok(_) => self.rm_node(self.node_list[i].id()),
                Err(_) => {}
            };
        }

        Some(new_bucket)
    }
}

impl fmt::Debug for Bucket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = format!(
            "{:?}\t{:?}\n\t{:?}",
            self.local().is_err(),
            self.start_id,
            self.end_id
        );

        for node in self.node_list.iter() {
            output = format!("{}\n\t{:?}", output, node);
        }

        write!(f, "{}\n", output)
    }
}
