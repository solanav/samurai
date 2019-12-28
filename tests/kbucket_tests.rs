use kademlia::kbucket::bucket::Bucket;
use kademlia::kbucket::node::Node;
use kademlia::kbucket::id::Id;

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

    println!("{:?}", bucket_list);

    match bucket_list[0].divide() {
        Some(val) => bucket_list.push(val),
        None => panic!("Failed to divide"),
    };

    println!("{:?}", bucket_list);
}
