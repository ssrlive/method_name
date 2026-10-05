## `::method_name`

> This crate is a fork of <https://github.com/danielhenrymantilla/rust-function_name>

Function attribute `#[named]` that generates a `method_name!` macro
in the scope of the function's body.

The generated `method_name!()` is a macro that expands to
the name of the annotated function, as a string literal.

[![Repository](https://img.shields.io/badge/repository-GitHub-brightgreen.svg)](https://github.com/ssrlive/method_name)
[![Latest version](https://img.shields.io/crates/v/method_name.svg)](https://crates.io/crates/method_name)
[![Documentation](https://docs.rs/method_name/badge.svg)](https://docs.rs/method_name)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![no_std compatible](https://img.shields.io/badge/no__std-compatible-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![License](https://img.shields.io/crates/l/method_name.svg)](https://github.com/ssrlive/method_name/blob/master/LICENSE-ZLIB)
[![CI](https://github.com/ssrlive/method_name/workflows/CI/badge.svg)](https://github.com/method_name/method_name/actions)

### Examples

```rust
use ::method_name::named;

#[named]
fn my_super_duper_function ()
{
    assert_eq!(
        method_name!(),
        "my_super_duper_function",
    );
}
```

For methods, put `#[named]` on the `impl` block to include the enclosing type
in `method_name!()`:

```rust
use ::method_name::named;

struct Session;

#[named]
impl Session {
    fn new() {
        assert_eq!(method_name!(), "Session::new");
    }
}
```

When `#[named]` is placed directly on a method, `method_name!()` continues to
return only the method name.

Since the generated `method_name!` expands to a string literal,
it can be used with other macros such as [`concat!`](https://doc.rust-lang.org/std/macro.concat.html):

```rust
#[macro_use] extern crate method_name;

macro_rules! function_path {() => (concat!(
    module_path!(), "::", method_name!()
))}

pub mod foo {
    pub mod bar {
        #[named]
        pub fn baz ()
        {
            assert_eq!(
                function_path!(),
                [
                    env!("CARGO_PKG_NAME"),
                    "foo", "bar",
                    "baz",
                ].join("::"),
            );
        }
    }
}
```

### Unstable API

The `method_name_unstable!()` macro is available when the `unstable` feature is
enabled. This API may change in future releases.

Enable the feature in your dependency declaration:

```toml
method_name = { version = "0.3", features = ["unstable"] }
```

Then import and call the macro from your crate:

```rust
use method_name::method_name_unstable;

fn foo() {
    assert_eq!(method_name_unstable!(), "foo");
}
```

[Repository]: https://github.com/ssrlive/method_name
[Documentation]: https://docs.rs/method_name
[crates.io]: https://crates.io/crates/method_name
