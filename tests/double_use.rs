use method_name::{method_name_unstable, named};

#[named]
fn foo() {
    assert_eq!(method_name!(), "foo");
    assert_eq!(method_name_unstable!(), "foo");
}

#[named]
fn bar() {
    assert_eq!(method_name!(), "bar");
    assert_eq!(method_name_unstable!(), "bar");
}

#[named]
async fn async_function() {
    tokio::task::yield_now().await;
    assert_eq!(method_name!(), "async_function");
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
        Session
    }
}

#[named]
impl Drop for Session {
    fn drop(&mut self) {
        assert_eq!(method_name!(), "Session::drop");
        assert_eq!(method_name_unstable!(), "Session::drop");
    }
}

#[test]
fn main() {
    foo();
    bar();
    LegacySession::new();
    Session::new();
}

#[tokio::test]
async fn async_names_survive_await() {
    async_function().await;
    Session::async_new().await;
}
