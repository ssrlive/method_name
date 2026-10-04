#[macro_use]
extern crate method_name;

#[test]
#[named]
fn r#if() {
    assert_eq!(method_name!(), "r#if");
}
