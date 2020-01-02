# What is this?
The intention of this piece of code is to create a p2p protocol on top of UDP.

# Goals
1. Privacy. The protocol should be encrypted end to end and one should not know where communications are comming from.
2. Resistance. The bootstraping should be completely decentrilized and the communications should work even when a large number of nodes are offline.
3. Intelligence. The network will have to identify nodes that are benefitial to the network and nodes that are either trying to attack the network or get benefits without contributing to it.

# Privacy
The protocol should use Elliptic-curve Diffie–Hellman when a new node is found so that we can encrypt all the following communications using the generated private key.

# Resitance
Instead of using traditional methods of bootstrapping, this protocol is going to use an organic way of selecting bootstrapping nodes that can provide a list of updated nodes to bootstrap to.

# Intelligence
When a new node is trying to bootstrap, the nodes should keep a score that represents the trust they have towards the new node. Untrusted nodes will only be able to respond to messages and work towards the benefit of the network. When the trust level has gone up enough, the node will be able to start sending messages to other nodes.
