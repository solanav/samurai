use crate::node::Node;

pub struct Bucket {
    node_list: Vec<Node>,
}

impl Bucket {
    pub fn new() -> Bucket {
        Bucket {
            node_list: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.node_list.push(node);
    }
}

#[cfg(test)]
mod bucket_test {
    #[test]
    fn create_bucket() {
        let mut b = crate::bucket::Bucket::new();
        let big = bigint::uint::U256::from_dec_str("123").unwrap();
        b.add_node(crate::node::Node::new(big));
        
    }
}