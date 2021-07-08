use adder;

#[test]
fn add_2_works_publically() {
    assert_eq!(adder::add_2(3), 5);
}