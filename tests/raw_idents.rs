#[cfg(feature = "std")]
#[macro_use]
extern crate method_name;

#[test]
#[cfg(feature = "std")]
#[named]
fn r#if() {
    assert_eq!(method_name!(), "r#if");

    #[cfg(feature = "unstable")]
    assert_eq!(method_name_unstable!(), "if");
}

#[test]
#[cfg(feature = "std")]
#[named]
fn r#match() {
    assert_eq!(method_name!(), "r#match");

    #[cfg(feature = "unstable")]
    assert_eq!(method_name_unstable!(), "match");
}
