#[macro_use]
extern crate method_name;

#[named]
fn foo() {
    assert_eq!(function_name!(), "foo");
    assert_eq!(method_name_full!(), "foo");
}

#[named]
fn bar() {
    assert_eq!(function_name!(), "bar");
    assert_eq!(method_name_full!(), "bar");
}

#[test]
fn main() {
    foo();
    bar();
}
