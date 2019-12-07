extern crate bigint;

use bigint::uint::U256 as u256;

pub struct Node {
    id: u256,
}

impl Node {
    pub fn new(id: u256) -> Node {
        Node {
            id: id,
        }
    }

    pub fn get_id(&self) -> u256 {
        self.id
    }
}

#[cfg(test)]
mod node_test {
    use bigint::uint::U256 as u256;

    #[test]
    fn create_node() {
        let big = u256::from_dec_str("123").unwrap();
        let node = crate::node::Node::new(big);
        assert_eq!(node.get_id(), big); 
    }
}