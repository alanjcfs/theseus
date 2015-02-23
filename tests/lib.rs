extern crate theseus;
/* Sanity Test */
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[should_fail]
fn it_fails() {
    assert_eq!(2 + 2, 5);
}

#[test]
fn add_two_works() {
    assert_eq!(4, theseus::add_two(2));
}
