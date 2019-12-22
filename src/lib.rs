extern crate rand;

pub mod id;

#[cfg(test)]
mod id_test {
    use crate::id::Id;

    #[test]
    fn add_id() {
        assert_eq!(Id::new(0, 0) + 1, Id::new(0, 1));
        assert_eq!(Id::new(0, u128::max_value()) + 1, Id::new(1, 0));
        assert_eq!(Id::new(u32::max_value(), u128::max_value()) + 1, Id::new(0, 1));
    }
}