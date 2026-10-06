#![cfg_attr(feature = "better-docs",
    cfg_attr(all(), doc = include_str!("../README.md")),
)]
#![doc(test(attr(deny(warnings), allow(unused),)))]
//! Attribute macro exposing the current function's name.
//!
//! Apply `#[named]` to an `impl` block to include the enclosing type in
//! `method_name!()` within its methods, such as `Session::new`.
#![warn(missing_docs)]
#![no_std]

/// Entry point of the crate.
///
/** ```rust
use ::method_name::named;

#[named]
fn foo()
{
    assert_eq!(method_name!(), "foo");
}

fn main() {
    foo();
}
``` */
pub use ::method_name_proc_macro::named;

#[doc(hidden)]
#[cfg(feature = "unstable")]
pub mod __private {
    extern crate alloc;
    pub use alloc::{borrow::Cow, format};
}

/// Returns the fully qualified name of the current function.
///
/// This macro is available when the `unstable` feature is enabled.
/// It means the result may be not fit the expected format in the future.
#[cfg(feature = "unstable")]
#[macro_export]
macro_rules! method_name_unstable {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            core::any::type_name::<T>()
        }
        let prefix = concat!(module_path!(), "::");
        let name = type_name_of(f);
        let name = name.strip_suffix("::f").unwrap_or(name);
        let name = name.trim_end_matches("::{{closure}}");
        if let Some((type_name, trait_and_method)) = name
            .strip_prefix('<')
            .and_then(|name| name.split_once(" as "))
        {
            match type_name.strip_prefix(prefix) {
                Some(type_name) => match trait_and_method.rsplit_once(">::") {
                    Some((_, method_name)) => $crate::__private::Cow::Owned(
                        $crate::__private::format!("{type_name}::{method_name}"),
                    ),
                    None => $crate::__private::Cow::Borrowed(name),
                },
                None => $crate::__private::Cow::Borrowed(name),
            }
        } else {
            $crate::__private::Cow::Borrowed(name.strip_prefix(prefix).unwrap_or(name))
        }
    }};
}
