use crate::types::id::Id;
use std::fmt;
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};

#[derive(fmt::Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Node {
    id: Id, // Id of the node
    local: bool, // Is this node us?
    addr: SocketAddr, // Address
}

impl Node {
    pub fn new(id: Id, local: bool, addr: SocketAddr) -> Self {
        Node {
            id,
            local,
            addr,
        }
    }

    pub fn is_local(&self) -> bool {
        self.local
    }

    pub fn id(self) -> Id {
        self.id
    }
}