#[macro_use]
extern crate method_name;

#[test]
#[named]
fn r#if() {
    assert_eq!(function_name!(), "r#if");
}
