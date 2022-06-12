use Integration;

mod common;

#[test]
fn test_adder2() {
    common::setup();
    assert_eq!(4, Integration::adder(2, 2));
}