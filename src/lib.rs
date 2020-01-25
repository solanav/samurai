pub mod types; // Data structures shared by other modules
pub mod server; // Server that listens for packets
pub mod client; // Functions used to send messages to other peers
pub mod bootstrap; // Functions used to get into the network when restarted
pub mod upkeep; // Functions to clean and update the peer structures