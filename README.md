# What is this?
The intention of this piece of code is to create a p2p protocol on top of UDP.

# Goals
1. Privacy and security. The protocol should be encrypted end to end and one should not know where communications are comming from.
2. Decentrilized bootstraping. The bootstraping should be completely decentrilized and the communications should work even when a large number of nodes are offline.
3. Attack detection. The network will have to identify nodes that are benefitial to the network and nodes that are either trying to attack the network or get benefits without contributing to it.

# Resources
- [x] [Original Kademlia paper (PDF)](https://pdos.csail.mit.edu/~petar/papers/maymounkov-kademlia-lncs.pdf)
- [x] [A lightweight Approach for Improving the Lookup performance in Kademlia-type Systems](https://arxiv.org/pdf/1408.3079.pdf)
- [ ] [Overbot (PDF)](https://seclab.ccs.neu.edu/static/publications/securecomm2008overbot.pdf)
- [ ] [Enhancing the Kademlia P2P Network (PDF)](https://www.researchgate.net/publication/274547077_Enhancing_the_Kademlia_P2P_Network/link/570fa3b808ae38897ba2c903/download)
