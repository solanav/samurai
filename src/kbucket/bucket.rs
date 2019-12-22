use crate::kbucket::id::Id;
use std::fmt;

pub struct Bucket {
    node_list: Vec<Id>,
    contains_u: bool,
    start_id: Id,
    end_id: Id,
    max_nodes: usize,
}

impl Bucket {
    pub fn new(max_nodes: usize) -> Bucket {
        Bucket {
            node_list: Vec::new(),
            contains_u: true,
            start_id: Id::zero(),
            end_id: Id::max(),
            max_nodes: max_nodes,
        }
    }

    pub fn add_node(&mut self, node: Id) -> Result<(), &'static str> {
        if self.node_list.len() >= self.max_nodes {
            return Err("This bucket is already full");
        }

        if node > self.start_id || node < self.end_id {
            return Err("This bucket should not contain that node");
        }

        self.node_list.push(node);
        Ok(())
    }

    pub fn rm_node(&mut self, node: Id) {
        for i in 0..self.node_list.len() {
            if node == self.node_list[i] {
                self.node_list.remove(i);
                return;
            }
        }
    }
}

impl fmt::Debug for Bucket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = format!(
            "{:?}\t{:?}\n\t{:?}\n",
            self.contains_u, self.start_id, self.end_id
        );

        for node in self.node_list.iter() {
            output = format!("{}\n\t{:?}", output, node);
        }

        write!(f, "{}", output)
    }
}
