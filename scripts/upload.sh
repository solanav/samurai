#!/bin/bash

# Compile
cd ..
cargo build --release --bin samurai_node

# Copy config file
ssh -t root@192.168.35.130 'echo "ip = \"192.168.35.130\"" > config.toml'
ssh -t root@192.168.35.131 'echo "ip = \"192.168.35.131\"" > config.toml'

# Copy peer list
scp peer_list.json root@192.168.35.130:~
scp peer_list.json root@192.168.35.131:~

# Copy compiled binary to nodes
scp target/release/samurai_node root@192.168.35.130:~
scp target/release/samurai_node root@192.168.35.131:~

