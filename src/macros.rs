/// Simplifies handler creation for synchronous builds.
///
/// This macro expands to a call to the `sync_h` helper function,
/// which wraps the synchronous handler function to make it compatible
/// with the server's unified handler system.
#[macro_export]
#[cfg(feature = "sync")]
macro_rules! handler {
    ($handler_fn:expr) => {
        $crate::core::handler::sync_h($handler_fn)
    };
}

/// Simplifies handler creation for asynchronous builds.
///
/// This macro expands to a call to the `async_h` helper function,
/// wrapping the user's `async fn` in a closure that pins and boxes the
/// future. This hides the necessary boilerplate from the user, providing
/// a clean API.
#[macro_export]
#[cfg(all(
  any(feature = "async_tokio", feature = "async_std", feature = "async_smol"),
  not(feature = "sync")
))]
macro_rules! handler {
    ($handler_fn:expr) => {
        $crate::core::handler::async_h(move |req| Box::pin($handler_fn(req)))
    };
}
