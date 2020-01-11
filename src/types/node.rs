use crate::types::id::Id;
use std::fmt;

#[derive(fmt::Debug, Copy, Clone)]
pub struct Node {
    id: Id, // Id of the node
    local: bool, // Is this node us?
}

impl Node {
    pub fn new(id: Id, local: bool) -> Self {
        Node { id, local }
    }

    pub fn is_local(self) -> bool {
        self.local
    }

    pub fn id(self) -> Id {
        self.id
    }
}