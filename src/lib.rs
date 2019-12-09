extern crate rand;

pub mod node;
pub mod bucket;

#[cfg(test)]
mod bucket_test {
    use crate::bucket::{Bucket, between_id};
    use crate::node::{Node, ID_SIZE};
    use rand::prelude::*;

    #[test]
    fn bucket_division() {
        let mut b = Bucket::new(3);
        let mut rng = rand::thread_rng();
        let mut last_id: Vec<u8> = vec![0; ID_SIZE];

        for _i in 0..3 {
            let mut random_id = Vec::<u8>::with_capacity(ID_SIZE);

            for _ in 0..ID_SIZE {
                random_id.push(rng.gen_range(0, 255));
            }

            last_id = random_id.clone();

            match b.add_node(Node::new(random_id)) {
                Ok(_) => {},
                Err(_) => match b.divide() {
                    Ok(divided) => println!("{:?}", divided),
                    Err(_) => println!("Failed to divide bucket"),
                },
            }
        }
        
        println!("{:?}", b);

        match b.rm_node(&last_id) {
            Ok(_) => println!("Removed correctly the node"),
            Err(msg) => println!("{}", msg),
        };
     
        println!("{:?}", b);
    }

    #[test]
    fn between() {
        assert_eq!(between_id(&vec![0; ID_SIZE], &vec![2; ID_SIZE], &vec![1; ID_SIZE]), true);
        assert_eq!(between_id(&vec![0; ID_SIZE], &vec![2; ID_SIZE], &vec![6; ID_SIZE]), false);
    }
}