#!/bin/bash
REMOTE_IP=192.168.139.133

scp mininet_script mininet@$REMOTE_IP:~
scp peer_list.txt mininet@$REMOTE_IP:~
cd ..
cargo build --release --bin samurai_node
scp target/release/samurai_node mininet@$REMOTE_IP:~
