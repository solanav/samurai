use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BucketError {
    BucketFull,
    IncorrectBucket,
    RepeatedLNode,
    LNodeNotFound,
    NodeNotFound,
    IndexError,
}

impl std::error::Error for BucketError {}

impl fmt::Display for BucketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BucketError::BucketFull => write!(f, "This bucket is full."),
            BucketError::IncorrectBucket => write!(f, "The node does not fit."),
            BucketError::RepeatedLNode => write!(f, "There already is a local node."),
            BucketError::LNodeNotFound => write!(f, "Could not find the local node."),
            BucketError::NodeNotFound => write!(f, "Could not find the node."),
            BucketError::IndexError => write!(f, "Index is out of bounds."),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerError {
    SearchRouter,
    AddPort,
}

impl std::error::Error for ServerError {}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServerError::SearchRouter => write!(f, "Could not find UPnP functions on router."),
            ServerError::AddPort => write!(f, "Could not add a new port redirection."),
        }
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileError {
    OpenFile,
    SaveData,
    LoadData,
}

impl std::error::Error for FileError {}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileError::OpenFile => write!(f, "Failed to open file for saving or loading."),
            FileError::SaveData => write!(f, "Failed to write json peer data to file."),
            FileError::LoadData => write!(f, "Failed to read json peer data from file."),
        }
    }
}