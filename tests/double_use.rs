#[cfg(feature = "unstable")]
use method_name::method_name_unstable;

#[cfg(feature = "std")]
use method_name::named;

#[cfg(feature = "std")]
#[named]
fn foo() {
    assert_eq!(method_name!(), "foo");
    #[cfg(feature = "unstable")]
    assert_eq!(method_name_unstable!(), "foo");
}

#[cfg(feature = "std")]
#[named]
fn bar() {
    assert_eq!(method_name!(), "bar");
    #[cfg(feature = "unstable")]
    assert_eq!(method_name_unstable!(), "bar");
}

#[cfg(feature = "std")]
#[named]
async fn async_function() {
    tokio::task::yield_now().await;
    assert_eq!(method_name!(), "async_function");

    #[cfg(feature = "unstable")]
    assert_eq!(method_name_unstable!(), "async_function");
}

#[cfg(feature = "unstable")]
fn function_with_closure() {
    let closure = || {
        assert_eq!(method_name_unstable!(), "function_with_closure");
    };
    closure();
}

#[cfg(feature = "std")]
struct LegacySession;

#[cfg(feature = "std")]
impl LegacySession {
    #[named]
    fn new() {
        assert_eq!(method_name!(), "new");
    }
}

#[cfg(feature = "std")]
struct Session;

#[cfg(feature = "std")]
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

#[cfg(feature = "std")]
trait FirstName {
    fn same_name(&self);
}

#[cfg(feature = "std")]
trait SecondName {
    fn same_name(&self);
}

#[cfg(feature = "std")]
struct SharedMethods;

#[cfg(feature = "std")]
#[named]
impl FirstName for SharedMethods {
    fn same_name(&self) {
        #[cfg(feature = "unstable")]
        assert_eq!(method_name_unstable!(), "SharedMethods::same_name");
    }
}

#[cfg(feature = "std")]
#[named]
impl SecondName for SharedMethods {
    fn same_name(&self) {
        #[cfg(feature = "unstable")]
        assert_eq!(method_name_unstable!(), "SharedMethods::same_name");
    }
}

#[cfg(feature = "std")]
#[named]
impl Drop for Session {
    fn drop(&mut self) {
        assert_eq!(method_name!(), "Session::drop");

        #[cfg(feature = "unstable")]
        assert_eq!(method_name_unstable!(), "Session::drop");
    }
}

#[cfg(feature = "std")]
#[test]
fn main() {
    foo();
    bar();
    LegacySession::new();
    let session = Session::new();
    session.generic(42_u8);
    #[cfg(feature = "unstable")]
    function_with_closure();
    FirstName::same_name(&SharedMethods);
    SecondName::same_name(&SharedMethods);
}

#[cfg(feature = "std")]
#[tokio::test]
async fn async_names_survive_await() {
    async_function().await;
    Session::async_new().await;
}
