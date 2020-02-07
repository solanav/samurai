use crate::id::Id;
use std::fmt;
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};

#[derive(fmt::Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Node {
    id: Id, // Id of the node
    local: bool, // Is this node us?
    addr: SocketAddr, // Address
    trust: f64, // Trust
}

impl Node {
    pub fn new(id: Id, local: bool, addr: SocketAddr) -> Self {
        Node {
            id,
            local,
            addr,
            trust: 0f64,
        }
    }

    pub fn is_local(&self) -> bool {
        self.local
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn set_trust(&mut self, trust: f64) {
        self.trust = trust;
    }
}