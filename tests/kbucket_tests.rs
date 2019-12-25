use kademlia::kbucket::bucket::Bucket;

#[test]
fn test_create() {
    let b = Bucket::new(10);
    println!("{:?}", b);
}