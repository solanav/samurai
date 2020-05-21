#!/bin/bash

# Node data
debug_server_user=solanav
debug_server_ip=192.168.1.123
debug_server_port=9393
node_user=solanav
node_ip_list=(
192.168.1.122
192.168.1.121
)

# Compile
cd ..
cargo build --release --bin samurai_node

# Copy server to server ip
scp scripts/server.py $debug_server_user@$debug_server_ip:~

# Iterate through ip_list.txt
for ip in ${node_ip_list[@]}
do
    echo "Working on $node_user@$ip"

    # Create config file
    echo bind_ip = \"$ip\" >> config.toml
    echo debug_ip = \"$debug_server_ip\" >> config.toml
    echo debug_port = $debug_server_port >> config.toml
    echo "" >> config.toml
    scp config.toml $node_user@$ip:~
    rm config.toml

    # Copy peer list
    scp peer_list.json $node_user@$ip:~

    # Copy compiled binary to nodes
    scp target/release/samurai_node $node_user@$ip:~

    echo
done
