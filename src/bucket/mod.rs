pub mod bucket_list;
pub mod error;

use crate::id::Id;
use crate::node::Node;
use std::fmt;
use serde::{Deserialize, Serialize};
use crate::bucket::error::BucketError;

#[derive(Serialize, Deserialize)]
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
            start_id,
            end_id,
            max_nodes,
        }
    }

    pub fn add_node(&mut self, node: &Node) -> Result<(), BucketError> {
        if self.node_list.len() >= self.max_nodes {
            return Err(BucketError::BucketFull);
        }

        if self.start_id > node.id() || node.id() > self.end_id {
            return Err(BucketError::IncorrectBucket);
        }

        if self.local().is_ok() && node.is_local() {
            return Err(BucketError::RepeatedLNode);
        }

        self.node_list.push(*node);
        Ok(())
    }

    pub fn rm_node(&mut self, id: Id) {
        if let Some(i) = self.node_list.iter().position(|&i| i.id() == id) {
            self.node_list.remove(i);
        }
    }

    pub fn local(&self) -> Result<Node, BucketError> {
        for node in self.node_list.iter() {
            if node.is_local() {
                return Ok(*node);
            }
        }

        Err(BucketError::LNodeNotFound)
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
            if new_bucket.add_node(node).is_ok() {
                self.rm_node(node.id())
            }
        }

        Some(new_bucket)
    }

    pub fn get(&self, i: usize) -> Result<Node, BucketError> {
        if i >= self.node_list.len() {
            return Err(BucketError::IndexError);
        }

        Ok(self.node_list[i])
    }

    pub fn get_by_id(&self, id: &Id) -> Result<Node, BucketError> {
        for node in self.node_list.iter() {
            if node.id() == *id {
                return Ok(*node);
            }
        }

        Err(BucketError::NodeNotFound)
    }

    pub fn fits(&self, id: &Id) -> bool {
        *id > self.start_id && *id < self.end_id
    }

    pub fn nodes(&self) -> &Vec<Node> {
        &self.node_list
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
