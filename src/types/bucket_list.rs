use crate::types::bucket::Bucket;
use crate::types::id::Id;
use crate::types::node::Node;
use std::fmt;


pub struct BucketList {
    buckets: Vec<Bucket>,
}

impl BucketList {
    pub fn new() -> Self {
        BucketList {
            buckets: Vec::new(),
        }
    }

    pub fn get_closest(&self, id: &Id) -> Vec<Id> {
        // Create a list of all nodes
        let mut global_node_list = Vec::new();
        for bucket in self.buckets.iter() {
            global_node_list.append(&mut bucket.node_list().clone());
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

    pub fn add_bucket(&mut self, bucket: Bucket) {
        self.buckets.push(bucket);
    }

    pub fn rm_bucket(&mut self, i: usize) {
        self.buckets.remove(i);
    }

    fn find_bucket(&self, id: &Id) -> usize {
        let mut i: usize = 0;
        for bucket in self.buckets.iter() {
            if bucket.fits(id) {
                return i;
            }

            i += 1;
        }

        panic!("Bucket list is not well built");
    }

    pub fn add_node(&mut self, node: &Node) -> Result<(), &'static str> {
        let i = self.find_bucket(&node.id());
        self.buckets[i].add_node(node)
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
