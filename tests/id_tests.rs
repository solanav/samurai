use samurai::id::Id;

#[test]
fn add() {
    // Normal addition
    assert_eq!(Id::new(0, 0) + 1, Id::new(0, 1));
    // If low overflows it goes to high
    assert_eq!(Id::new(0, u128::max_value()) + 1, Id::new(1, 0));
    // Does not overflow
    assert_eq!(
        Id::new(u128::max_value(), u128::max_value()) + 1,
        Id::new(u128::max_value(), u128::max_value())
    );
}

#[test]
fn sub() {
    // Normal substraction
    assert_eq!(Id::new(0, 1) - 1, Id::new(0, 0));
    assert_eq!(Id::new(1, 0) - 1, Id::new(0, u128::max_value()));
    // Does not underflow
    assert_eq!(Id::new(0, 0) - 1, Id::new(0, 0));
}

#[test]
fn cmp() {
    assert!(Id::new(0, 1) > Id::new(0, 0));
    assert!(Id::new(1, 0) > Id::new(0, 0));
    assert!(Id::new(1, 0) > Id::new(0, 1));
}

#[test]
fn zero() {
    let id = Id::zero();
    assert_eq!(id, Id::new(0, 0));
}

#[test]
fn max() {
    let id = Id::max();
    assert_eq!(id, Id::new(u128::max_value(), u128::max_value()));
}

#[test]
fn rand() {
    let id0 = Id::rand();
    let id1 = Id::rand();
    assert_ne!(id0, id1);
}

#[test]
fn half() {
    let id = Id::new(0, 9);
    assert_eq!(id.half(), Id::new(0, 4));
}

#[test]
fn bytes() {
    let buf = [0u8; 32];
    let id0 = Id::from_bytes(&buf);
    assert_eq!(id0, Id::zero());

    let buf = [255u8; 32];
    let id0 = Id::from_bytes(&buf);
    assert_eq!(id0, Id::max());
}