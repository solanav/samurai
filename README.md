# What is this?
The intention of this piece of code is to create a p2p protocol on top of TCP. 

# Warning
This project is still in a very early stage. It cannot be used for anything yet.

# Remote testing with docker
1. Get docker on a remote server with ssh.
2. Get docker on your local machine.
3. Create a ssh key with ssh-keygen and copy it to the server (/home/username/.ssh/authorized_keys).
```
(Linux)> ssh-keygen
(Windows)> ssh-keygen
```
4. Add server as remote host on your local machine.
```
(Linux)> export DOCKER_HOST=ssh://usernam@your_server_ip
(Windows)> $env:DOCKER_HOST="ssh://username@your_server_ip"
```
5. Run the script on the docker folder.
```
(Linux)> ./run.sh
(Windows)> .\run.ps1
```

# Contribute
If you want to contribute to this project, either take a look at the issues or send me an email (solanav at qq.com) if you want to collaborate more deeply.

# Goals
1. End to end encryption
2. Decentralized bootstraping
3. Trust system

# Resources I'm using
- [x] [Original Kademlia paper (PDF)](https://pdos.csail.mit.edu/~petar/papers/maymounkov-kademlia-lncs.pdf)
- [x] [A lightweight Approach for Improving the Lookup performance in Kademlia-type Systems](https://arxiv.org/pdf/1408.3079.pdf)
- [ ] [Overbot (PDF)](https://seclab.ccs.neu.edu/static/publications/securecomm2008overbot.pdf)
- [ ] [Enhancing the Kademlia P2P Network (PDF)](https://www.researchgate.net/publication/274547077_Enhancing_the_Kademlia_P2P_Network/link/570fa3b808ae38897ba2c903/download)
