from mininet.topo import Topo
from mininet.net import Mininet
from mininet.node import Node
from mininet.log import setLogLevel, info
from mininet.cli import CLI

class MyTopo( Topo ):
    def __init__( self ):
        Topo.__init__( self )

        h0 = self.addHost('h0')
        h1 = self.addHost('h1')
        h2 = self.addHost('h2')
        h3 = self.addHost('h3')
        h4 = self.addHost('h4')

        s0 = self.addSwitch('s0')
        s1 = self.addSwitch('s1')

        self.addLink(h0, s0)
        self.addLink(h1, s0)
        self.addLink(h2, s0)

        self.addLink(h3, s1)
        self.addLink(h4, s1)

        self.addLink(s0, s1)

def run():
    topo = MyTopo()
    net = Mininet(topo=topo)
    net.start()
    CLI(net)
    net.stop()

if __name__ == '__main__':
    setLogLevel('info')
    run()