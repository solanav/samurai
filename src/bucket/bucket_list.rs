use crate::bucket::Bucket;
use crate::id::Id;
use crate::node::Node;
use std::fmt;
use serde::{Deserialize, Serialize};
use crate::error::BucketError;

#[derive(Serialize, Deserialize)]
pub struct BucketList {
    buckets: Vec<Bucket>,
    max_buckets: usize,
}

impl BucketList {
    /// Create a new bucket list.
    /// max_bucket: maximum number of buckets
    /// max_per_bucket: maximum number of nodes per bucket
    pub fn new(max_buckets: usize, max_per_bucket: usize) -> Self {
        let mut buckets = Vec::new();
        buckets.push(Bucket::new(max_per_bucket, Id::zero(), Id::max()));

        BucketList {
            buckets,
            max_buckets,
        }
    }

    /// Return the ID of all nodes in the bucket_list
    /// sorted by xor distance.
    pub fn get_closest(&self, id: &Id) -> Vec<Id> {
        // Create a list of all nodes
        let mut global_node_list = Vec::new();
        for bucket in self.buckets.iter() {
            global_node_list.append(&mut bucket.nodes().clone());
        }

        // Create vector of xor distances and their corresponding ID
        let mut xor_vec: Vec<(Id, Id)> = global_node_list.iter()
            .map(|node| (*id ^ node.id(), node.id()))
            .collect();

        // Sort by xor distance
        xor_vec.sort_by(|a, b| a.0.cmp(&b.0));

        // Return the ID only, not the xor distances
        xor_vec.iter()
            .map(|tup| tup.1)
            .collect()
    }

    /// Get the bucket of a node by its ID
    fn find_bucket(&self, id: &Id) -> usize {
        // Find the bucket for a given node id
        let mut i: usize = 0;
        for bucket in self.buckets.iter() {
            if bucket.fits(id) {
                return i;
            }

            i += 1;
        }

        panic!("Bucket list is not well built");
    }

    /// Add a new node to the bucket_list
    pub fn add_node(&mut self, node: &Node) -> Result<(), BucketError> {
        let i = self.find_bucket(&node.id());
        self.buckets[i].add_node(node)
    }

    /// Check if you can add more nodes to the bucket_list
    pub fn empty_space(&self) -> bool {
        // Call this function to know if you can add more nodes
        self.buckets.len() < self.max_buckets
    }

    pub fn node_list(&self) -> Vec<Node> {
        let mut node_list = Vec::new();
        for bucket in self.buckets.iter() {
            for node in bucket.nodes().iter() {
                node_list.push(*node);
            }
        }

        node_list
    }
}

impl fmt::Debug for BucketList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::from("");
        for bucket in self.buckets.iter() {
            output = format!("{}\n{:?}", output, bucket);
        }

        write!(f, "{}\n", output)
    }
}
