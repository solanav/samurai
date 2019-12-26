use kademlia::kbucket::bucket::Bucket;
use kademlia::kbucket::id::Id;

#[test]
fn test_add() {
    let mut b = Bucket::new(10);
    let i = Id::new(1, 1);
    
    match b.add_node(i) {
        Ok(_) => {},
        Err(e) => panic!(e),
    }

    println!("{:?}", b);
}

#[test]
fn test_rm() {
    let mut b = Bucket::new(10);
    let i = Id::new(0, 1);
    assert_eq!(b.add_node(i.clone()).is_ok(), true);
    println!("{:?}", b);
    b.rm_node(i);
    println!("{:?}", b);
}
