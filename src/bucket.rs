use std::fmt;
use crate::node::{Node, ID_SIZE};

pub struct Bucket {
    node_list: Vec<Node>,
    contains_u: bool,
    start_id: Vec<u8>,
    end_id: Vec<u8>,
    max_nodes: u32,
}

pub fn add_vec(v: &Vec<u8>) -> Vec<u8> {
    let mut r = v.clone();
    
    for i in (0..r.len()).rev() {
        if r[i] == 255 {
            r[i] = 0;
        }
        else {
            r[i] += 1;
            break;
        }
    }

    r
}

pub fn between_id(start: &Vec<u8>, end: &Vec<u8>, v: &Vec<u8>) -> bool {
    if start.len() != end.len() || start.len() != v.len() {
        return false
    }

    // Check if bigger than the start    
    for i in 0..start.len() {
        if v[0] < start[i] {
            return false;
        } else if v[0] > start[i] {
            break;
        } else {
            continue;
        }
    }

    // Check if smaller than the end    
    for i in 0..end.len() {
        if v[0] > end[i] {
            return false;
        } else if v[0] < end[i] {
            break;
        } else {
            continue;
        }
    }
    
    true
}

impl Bucket {
    pub fn new(max_nodes: u32) -> Bucket {
        Bucket {
            node_list: Vec::new(),
            contains_u: true,
            start_id: vec![0; 20],
            end_id: vec![255; 20],
            max_nodes: max_nodes,
        }
    }

    pub fn divide(&mut self) -> Result<Bucket, &'static str> {
        if self.contains_u == false {
            return Err("This bucket does not contain the self")
        }

        let old_end = self.end_id.clone();
        self.end_id[0] /= 2;

        let mut nu_bucket = Bucket {
            node_list: Vec::new(),
            contains_u: false,
            start_id: add_vec(&self.end_id),
            end_id: old_end,
            max_nodes: self.max_nodes,
        };

        for i in 0..self.node_list.len() {
            match self.add_node(self.node_list[i].clone()) {
                Ok(_) => {},
                Err(_) => match nu_bucket.add_node(self.node_list[i].clone()) {
                    Ok(_) => {},
                    Err(_) => println!("Failed to add node to divided bucket"),
                },
            };
        }

        Ok(nu_bucket)
    }

    pub fn add_node(&mut self, node: Node) -> Result<(), &'static str> {
        if self.node_list.len() >= self.max_nodes as usize {
            return Err("This bucket is already full");
        }

        if between_id(&self.start_id, &self.end_id, &node.get_id()) == false {
            return Err("This bucket should not contain that node")
        }

        self.node_list.push(node);
        Ok(())
    }

    pub fn rm_node(&mut self, node_id: &Vec<u8>) -> Result<(), &'static str> {
        for i in 0..self.node_list.len() {
            let mut eq = true;
            let other_id = self.node_list[i].get_id();

            // Check if this is the node we are looking for
            for j in 0..ID_SIZE {
                if other_id[j] != node_id[j] {
                    println!("{} > {}", other_id[j], node_id[j]);
                    eq = false;
                    break;
                }
            }
            
            // If we find the node, delete it
            if eq == true {
                self.node_list.remove(i);
                return Ok(())
            }
        }

        Err("Failed to remove node")
    }
}

impl fmt::Debug for Bucket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = format!("{:?}\t{:?}\n\t{:?}\n", self.contains_u, self.start_id, self.end_id);

        for node in self.node_list.iter() {
            output = format!("{}\n\t{:?}", output, node);
        }

        write!(f, "{}", output)
    }
}