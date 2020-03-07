use std::fs::File;
use std::io::{BufWriter, BufReader};
use crate::node::Node;
use crate::error::FileError;

pub fn save(path: &str, node_list: &Vec<Node>) -> Result<(), FileError> {
    let file = match File::create(&path) {
        Err(_) => return Err(FileError::OpenFile),
        Ok(file) => file,
    };

    let writer = BufWriter::new(file);
    if let Err(_) = serde_json::to_writer_pretty(writer, node_list) {
        return Err(FileError::SaveData);
    }

    Ok(())
}

pub fn load(path: &str) -> Result<Vec<Node>, FileError> {
    let file = match File::open(&path) {
        Err(_) => return Err(FileError::OpenFile),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    match serde_json::from_reader(reader) {
        Ok(nodes) => Ok(nodes),
        Err(_) => Err(FileError::LoadData),
    }
}