use adder;

mod common;

#[test]
fn add_2_works_publically() {
    common::setup();
    assert_eq!(adder::add_2(3), 5);
}