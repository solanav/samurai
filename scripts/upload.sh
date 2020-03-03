#!/bin/bash

# Compile
cd ..
cargo build --release --bin samurai_node

# Copy peer list
scp peer_list.json root@192.168.35.130:~
scp peer_list.json root@192.168.35.131:~

# Copy compiled binary to nodes
scp target/release/samurai_node root@192.168.35.130:~
scp target/release/samurai_node root@192.168.35.131:~

