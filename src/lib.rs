pub mod node;
pub mod bucket;

#[cfg(test)]
mod bucket_test {
    #[test]
    fn test0() {
        let mut b = crate::bucket::Bucket::new(3);
        
        for _i in 0..4 {
            match b.add_node(crate::node::Node::new(vec![0; 20])) {
                Ok(_) => {},
                Err(_) => println!("We have to divide this bucket"),
            }
        }
        
        println!("{:?}", b);
    }
}