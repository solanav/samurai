pub mod bucket; // Data structures shared by other modules
pub mod server; // Server that listens for packets
pub mod client; // Functions used to send messages to other peers
pub mod bootstrap; // Functions used to get into the network when restarted
pub mod id; // Id of a node in the network
pub mod node; // Structure representing a node in the network
pub mod packet; // Packet structure of the protocol