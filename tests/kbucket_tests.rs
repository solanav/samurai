use kademlia::kbucket::id::Id;

#[test]
fn test_add() {
    // Normal addition
    assert_eq!(Id::new(0, 0) + 1, Id::new(0, 1));
    
    // If low overflows it goes to high
    assert_eq!(Id::new(0, u128::max_value()) + 1, Id::new(1, 0));
    
    // Does not overflow both
    assert_eq!(
        Id::new(u32::max_value(), u128::max_value()) + 1,
        Id::new(u32::max_value(), u128::max_value())
    );
}

#[test]
fn test_sub() {
    // Normal addition
    assert_eq!(Id::new(0, 1) - 1, Id::new(0, 0));
    
    // Does not underflow
    assert_eq!(Id::new(0, 0) - 1, Id::new(0, 0));
}

#[test]
fn test_cmp() {
    assert_eq!(Id::new(0, 1) > Id::new(0, 0), true);
    assert_eq!(Id::new(1, 0) > Id::new(0, 0), true);
    assert_eq!(Id::new(1, 0) > Id::new(0, 1), true);
}