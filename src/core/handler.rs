// src/core/handler.rs
#![cfg(any(
  feature = "sync",
  feature = "async_tokio",
  feature = "async_std",
  feature = "async_smol"
))]

use crate::{Request, Response};
use async_trait::async_trait;
use futures::future::BoxFuture;
use std::sync::Arc;

/// Unified async Handler.
#[async_trait]
pub trait Handler: Send + Sync {
  async fn handle(&self, request: &Request) -> Response;
}

#[async_trait]
impl Handler for Arc<dyn Handler> {
  async fn handle(&self, request: &Request) -> Response {
    (**self).handle(request).await
  }
}

// Wrap a sync function.
struct SyncFnHandler<F>(F);
#[async_trait]
impl<F> Handler for SyncFnHandler<F>
where
  F: for<'a> Fn(&'a Request) -> Response + Send + Sync,
{
  async fn handle(&self, request: &Request) -> Response {
    (self.0)(request)
  }
}
pub fn sync_h<F>(f: F) -> Arc<dyn Handler>
where
  F: for<'a> Fn(&'a Request) -> Response + Send + Sync + 'static,
{
  Arc::new(SyncFnHandler(f))
}

// Wrap an async BoxFuture fn.
struct AsyncFnHandler<F>(F);
#[async_trait]
impl<F> Handler for AsyncFnHandler<F>
where
  F: for<'a> Fn(&'a Request) -> BoxFuture<'a, Response> + Send + Sync,
{
  async fn handle(&self, request: &Request) -> Response {
    (self.0)(request).await
  }
}
pub fn async_h<F>(f: F) -> Arc<dyn Handler>
where
  F: for<'a> Fn(&'a Request) -> BoxFuture<'a, Response> + Send + Sync + 'static,
{
  Arc::new(AsyncFnHandler(f))
}

/// Simplifies handler creation for synchronous builds.
///
/// This macro expands to a call to the `sync_h` helper function,
/// which wraps the synchronous handler function to make it compatible
/// with the server's unified handler system.
#[cfg(feature = "sync")]
#[macro_export]
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
#[cfg(all(
  any(feature = "async_tokio", feature = "async_std", feature = "async_smol"),
  not(feature = "sync")
))]
#[macro_export]
macro_rules! handler {
    ($handler_fn:expr) => {
        $crate::core::handler::async_h(move |req| Box::pin($handler_fn(req)))
    };
}