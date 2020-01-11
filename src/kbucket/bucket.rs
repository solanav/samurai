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

        if self.local().is_ok() && node.is_local() {
            return Err("There already is a local node inside the bucket")
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
            println!("Too many nodes");
            return None;
        }

        if self.local().is_err() {
            println!("No local");
            return None;
        }

        // Update the end_id
        let end_id = self.end_id;
        self.end_id = self.end_id.half();

        let mut new_bucket = Bucket::new(self.max_nodes, self.end_id + 1, end_id);

        // Move nodes to corresponding bucket
        let node_list_copy = self.node_list.clone();
        for node in node_list_copy.iter() {
            if new_bucket.add_node(*node).is_ok() {
                self.rm_node(node.id())
            }
        }

        Some(new_bucket)
    }

    pub fn get(&self, i: usize) -> Result<Node, &'static str> {
        if i >= self.node_list.len() {
            return Err("Index out of range");
        }

        Ok(self.node_list[i])
    }

    pub fn get_by_id(&self, id: &Id) -> Result<Node, &'static str> {
        for node in self.node_list.iter() {
            if node.id() == *id {
                return Ok(*node);
            }
        }

        Err("Node not found on bucket")
    }

    pub fn get_closest(&self, id: &Id) -> Vec<Id> {
        let mut xor_vec: Vec<(Id, Id)> = self.node_list.iter()
            .map(|node| (*id ^ node.id(), node.id()))
            .collect();

        xor_vec.sort_by(|a, b| a.0.cmp(&b.0));

        xor_vec.iter()
            .map(|tup| tup.1)
            .collect()
    }
}

impl fmt::Debug for Bucket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = format!(
            "{:?}\t{:?}\n\t{:?}",
            self.local().is_ok(),
            self.start_id,
            self.end_id
        );

        for node in self.node_list.iter() {
            output = format!("{}\n\t{:?}", output, node);
        }

        write!(f, "{}\n", output)
    }
}
