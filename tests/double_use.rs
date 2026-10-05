#[cfg(feature = "unstable")]
use method_name::method_name_unstable;
use method_name::named;

#[named]
fn foo() {
    assert_eq!(method_name!(), "foo");
    #[cfg(feature = "unstable")]
    assert_eq!(method_name_unstable!(), "foo");
}

#[named]
fn bar() {
    assert_eq!(method_name!(), "bar");
    #[cfg(feature = "unstable")]
    assert_eq!(method_name_unstable!(), "bar");
}

#[named]
async fn async_function() {
    tokio::task::yield_now().await;
    assert_eq!(method_name!(), "async_function");

    #[cfg(feature = "unstable")]
    assert_eq!(method_name_unstable!(), "async_function");
}

fn function_with_closure() {
    let closure = || {
        #[cfg(feature = "unstable")]
        assert_eq!(method_name_unstable!(), "function_with_closure");
    };
    closure();
}

struct LegacySession;

impl LegacySession {
    #[named]
    fn new() {
        assert_eq!(method_name!(), "new");
    }
}

struct Session;

#[named]
impl Session {
    fn new() -> Self {
        assert_eq!(method_name!(), "Session::new");
        Session
    }

    async fn async_new() -> Self {
        tokio::task::yield_now().await;
        assert_eq!(method_name!(), "Session::async_new");

        #[cfg(feature = "unstable")]
        assert_eq!(method_name_unstable!(), "Session::async_new");

        Session
    }

    fn generic<U>(&self, _value: U) {
        #[cfg(feature = "unstable")]
        assert_eq!(method_name_unstable!(), "Session::generic");
    }
}

trait FirstName {
    fn same_name(&self);
}

trait SecondName {
    fn same_name(&self);
}

struct SharedMethods;

#[named]
impl FirstName for SharedMethods {
    fn same_name(&self) {
        #[cfg(feature = "unstable")]
        assert_eq!(method_name_unstable!(), "SharedMethods::same_name");
    }
}

#[named]
impl SecondName for SharedMethods {
    fn same_name(&self) {
        #[cfg(feature = "unstable")]
        assert_eq!(method_name_unstable!(), "SharedMethods::same_name");
    }
}

#[named]
impl Drop for Session {
    fn drop(&mut self) {
        assert_eq!(method_name!(), "Session::drop");

        #[cfg(feature = "unstable")]
        assert_eq!(method_name_unstable!(), "Session::drop");
    }
}

#[test]
fn main() {
    foo();
    bar();
    LegacySession::new();
    let session = Session::new();
    session.generic(42_u8);
    function_with_closure();
    FirstName::same_name(&SharedMethods);
    SecondName::same_name(&SharedMethods);
}

#[tokio::test]
async fn async_names_survive_await() {
    async_function().await;
    Session::async_new().await;
}
