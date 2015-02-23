/* Sanity Test */
#[test]
fn it_works() {
    assert!(2 + 2 == 4);
}

#[test]
#[should_fail]
fn it_fails() {
    assert!(2 + 2 == 5);
}
