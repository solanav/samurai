use samurai::types::bucket::Bucket;
use samurai::types::node::Node;
use samurai::types::id::Id;

static BUCKET_SIZE: usize = 10;

#[test]
fn test_add() {
    let mut b = Bucket::new(BUCKET_SIZE, Id::zero(), Id::max());
    let n = Node::new(Id::new(1, 1), true);
    assert_eq!(b.add_node(n).is_ok(), true);
}

#[test]
fn test_bucket_list() {
    let mut bucket_list: Vec<Bucket> = Vec::new();
    bucket_list.push(Bucket::new(BUCKET_SIZE, Id::zero(), Id::max()));
    
    // Add local node
    match bucket_list[0].add_node(Node::new(Id::rand(), true)) {
        Ok(_) => {},
        Err(_) => panic!("Failed to add random node to bucket list"),
    };

    // Add more nodes
    for _i in 0..5 {
        match bucket_list[0].add_node(Node::new(Id::rand(), false)) {
            Ok(_) => {},
            Err(_) => panic!("Failed to add random node to bucket list"),
        };
    }
    
    println!("{:?}", bucket_list);

    // Divide and add the new bucket to the list
    match bucket_list[0].divide() {
        Some(val) => bucket_list.push(val),
        None => panic!("Failed to divide"),
    };

    println!("{:?}", bucket_list);
}

#[test]
fn test_xor_distance() {
    let mut bucket_list: Vec<Bucket> = Vec::new();
    bucket_list.push(Bucket::new(BUCKET_SIZE, Id::zero(), Id::max()));

    // Add local node
    match bucket_list[0].add_node(Node::new(Id::rand(), true)) {
        Ok(_) => {},
        Err(_) => panic!("Failed to add random node to bucket list"),
    };

    // Add more nodes
    for _i in 0..5 {
        match bucket_list[0].add_node(Node::new(Id::rand(), false)) {
            Ok(_) => {},
            Err(_) => panic!("Failed to add random node to bucket list"),
        };
    }

    println!("{:?}", bucket_list);

    let id = Id::rand();
    println!("{:?}", bucket_list[0].get_closest(&id));
}
