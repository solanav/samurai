use samurai::types::bucket::Bucket;
use samurai::types::bucket_list::BucketList;
use samurai::types::node::Node;
use samurai::types::id::Id;
use std::net::{Ipv4Addr, SocketAddr, IpAddr};

static BUCKET_SIZE: usize = 10;

macro_rules! zero_addr {
    () => {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0,0,0,0)), 1234)
    }
}

#[test]
fn test_add() {
    let mut b = Bucket::new(BUCKET_SIZE, Id::zero(), Id::max());
    let n = Node::new(Id::new(1, 1), true, zero_addr!());
    assert_eq!(b.add_node(&n).is_ok(), true);
}

#[test]
fn test_bucket_list() {
    let mut bucket_list: BucketList = BucketList::new();
    bucket_list.add_bucket(Bucket::new(BUCKET_SIZE, Id::zero(), Id::max()));
    
    // Add local node
    match bucket_list.add_node(&Node::new(Id::rand(), true, zero_addr!())) {
        Ok(_) => {},
        Err(_) => panic!("Failed to add random node to bucket list"),
    };

    // Add more nodes
    for _i in 0..5 {
        match bucket_list.add_node(&Node::new(Id::rand(), false, zero_addr!())) {
            Ok(_) => {},
            Err(_) => panic!("Failed to add random node to bucket list"),
        };
    }
    
    println!("{:?}", bucket_list);
}

#[test]
fn test_xor_distance() {
    let mut bucket_list = BucketList::new();
    bucket_list.add_bucket(Bucket::new(BUCKET_SIZE, Id::zero(), Id::max()));

    // Add local node
    match bucket_list.add_node(&Node::new(Id::rand(), true, zero_addr!())) {
        Ok(_) => {},
        Err(_) => panic!("Failed to add random node to bucket list"),
    };

    // Add more nodes
    for _i in 0..5 {
        match bucket_list.add_node(&Node::new(Id::rand(), false, zero_addr!())) {
            Ok(_) => {},
            Err(_) => panic!("Failed to add random node to bucket list"),
        };
    }

    println!("{:?}", bucket_list);

    let id = Id::rand();
    println!("{:?}", bucket_list.get_closest(&id));
}
