use ch_11_automated_tests::adder::add_2;

#[test]
fn it_adds_two() {
    let result = add_2(2);
    assert_eq!(result, 4);
}
