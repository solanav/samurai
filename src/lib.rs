pub mod node;
pub mod bucket;

#[cfg(test)]
mod node_test {
    #[test]
    fn create_node() {
        let big = vec![0; 20];
        let node = crate::node::Node::new(big.clone());
        assert_eq!(node.get_id(), big);
    }
}

#[cfg(test)]
mod bucket_test {
    #[test]
    fn create_bucket() {
        let mut b = crate::bucket::Bucket::new();
        b.add_node(crate::node::Node::new(vec![0; 20]));
    }
}