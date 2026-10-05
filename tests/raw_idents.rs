#[macro_use]
extern crate method_name;

#[test]
#[named]
fn r#if() {
    assert_eq!(method_name!(), "r#if");

    #[cfg(feature = "unstable")]
    assert_eq!(method_name_unstable!(), "if");
}
