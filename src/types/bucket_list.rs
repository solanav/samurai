use crate::types::bucket::Bucket;
use crate::types::id::Id;
use crate::types::node::Node;
use std::fmt;


pub struct BucketList {
    buckets: Vec<Bucket>,
    max_buckets: usize,
    max_per_bucket: usize,
}

impl BucketList {
    pub fn new(max_buckets: usize, max_per_bucket: usize) -> Self {
        let mut buckets = Vec::new();
        buckets.push(Bucket::new(max_per_bucket, Id::zero(), Id::max()));

        BucketList {
            buckets,
            max_buckets,
            max_per_bucket,
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

    pub fn empty_space(&self) -> bool {
        // Call this function to know if you can add more nodes
        self.buckets.len() < self.max_buckets
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
