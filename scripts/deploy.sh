#!/bin/bash

# This scripts uploads the necessary files to all the nodes
# listed on the file ip_list.txt

IP_LIST="scripts/ip_list.txt"
CONFIG="scripts/deploy.conf"

# Compile
cd ..
cargo build --release --bin samurai_node

# Get variables from config
source $CONFIG

# Copy server to server ip
scp scripts/server.py $debug_server_user@$debug_server_ip:~

# Iterate through ip_list.txt
for ip in ${node_ip_list[@]}
do
    echo "Working on $node_user@$ip"

    # Create and copy config file
    echo ip = "$ip" > config.toml
    scp config.toml $node_user@$ip:~
    rm config.toml

    # Copy peer list
    scp peer_list.json $node_user@$ip:~

    # Copy compiled binary to nodes
    scp target/release/samurai_node $node_user@$ip:~

    echo
done
