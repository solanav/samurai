use std::fs::File;
use std::io::{BufWriter, BufReader, Write, BufRead};
use crate::node::Node;
use crate::error::FileError;
use std::net::{SocketAddr, IpAddr};
use std::str::FromStr;
use crate::id::Id;

pub fn save(path: &str, node_list: Vec<&Node>) -> Result<(), FileError> {
    let file = match File::create(&path) {
        Err(_) => return Err(FileError::OpenFile),
        Ok(file) => file,
    };

    let mut writer = BufWriter::new(file);

    for node in node_list {
        let data = format!("{},{},{},{},{}",
            node.id().high(),
            node.id().low(),
            node.is_local(),
            node.addr().ip(),
            node.addr().port(),
        );

        let _ = writer.write(data.as_bytes());
    }

    Ok(())
}

pub fn load(path: &str) -> Result<Vec<Node>, FileError> {
    let file = match File::open(&path) {
        Err(_) => return Err(FileError::OpenFile),
        Ok(file) => file,
    };

    let mut node_list = Vec::new();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Err(e) = line {
            println!("Error reading file: {}", e);
            return Err(FileError::LoadData);
        }

        let line = line.unwrap();
        let raw_node: Vec<&str> = line.split(",").collect();

        if raw_node.len() != 5 {
            break;
        }

        let idh = u128::from_str(raw_node[0]).unwrap();
        let idl = u128::from_str(raw_node[1]).unwrap();
        let id = Id::new(idh, idl);

        let local = bool::from_str(raw_node[2]).unwrap();
        let ip = IpAddr::from_str(raw_node[3]).unwrap();
        let port = u16::from_str(raw_node[4]).unwrap();

        let node = Node::new(id, local, SocketAddr::new(ip, port));
        node_list.push(node);
    }

    Ok(node_list)
}