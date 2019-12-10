use std::fmt;

pub const ID_SIZE: usize = 20;

#[derive(Clone)]
pub struct Node {
    id: Vec<u8>,
}

impl Node {
    pub fn new(id: Vec<u8>) -> Node {
        Node {
            id: id,
        }
    }

    pub fn get_id(&self) -> Vec<u8> {
        self.id.clone()
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.get_id())
    }
}