#!/bin/bash
cd ..
cargo build --release --bin samurai_node
scp target/release/samurai_node mininet@192.168.139.133:~
