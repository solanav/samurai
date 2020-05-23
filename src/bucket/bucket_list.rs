use crate::bucket::Bucket;
use crate::id::Id;
use crate::node::Node;
use std::fmt;
use crate::error::BucketError;
use std::net::SocketAddr;

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
    pub fn get_closest(&mut self, id: &Id) -> Vec<Id> {
        let mut g_xor_vec: Vec<(Id, Id)> = Vec::new();

        // Create a list of all nodes
        for bucket in self.buckets.iter_mut() {
            let mut xor_vec: Vec<(Id, Id)> = bucket.nodes().iter()
                .map(|node| (*id ^ node.id(), node.id())).collect();

            g_xor_vec.append(&mut xor_vec)
        }

        // Sort by xor distance
        g_xor_vec.sort_by(|a, b| a.0.cmp(&b.0));

        // Return the ID only, not the xor distances
        g_xor_vec.iter()
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
    pub fn add_node(&mut self, node: Node) -> Result<(), BucketError> {
        let i = self.find_bucket(&node.id());
        self.buckets[i].add_node(node)
    }

    /// Finds a node given their address or None
    pub fn get_node(&mut self, addr: SocketAddr) -> Option<&mut Node> {
        for bucket in self.buckets.iter_mut() {
            for node in bucket.nodes() {
                if node.addr() == addr {
                    return Some(node);
                }
            }
        }

        None
    }

    /// Returns a list of nodes extracted from the bucket list
    pub fn node_list(&mut self) -> Vec<&mut Node> {
        let mut node_list = Vec::new();
        for bucket in self.buckets.iter_mut() {
            for node in bucket.nodes() {
                node_list.push(node);
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
