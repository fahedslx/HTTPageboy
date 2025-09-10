#[cfg(any(feature = "async_tokio", feature = "async_std", feature = "async_smol"))]
use std::future::Future;
#[cfg(any(feature = "async_tokio", feature = "async_std", feature = "async_smol"))]
use std::pin::Pin;
#[cfg(any(feature = "async_tokio", feature = "async_std", feature = "async_smol"))]
use std::sync::Arc;

use crate::core::request::Request;
use crate::core::response::Response;

#[cfg(feature = "sync")]
pub type Handler = fn(&Request) -> Response;

#[cfg(any(feature = "async_tokio", feature = "async_std", feature = "async_smol"))]
pub type Handler =
  Arc<dyn for<'a> Fn(&'a Request) -> Pin<Box<dyn Future<Output = Response> + Send + 'a>> + Send + Sync + 'static>;

#[cfg(not(any(
  feature = "sync",
  feature = "async_tokio",
  feature = "async_std",
  feature = "async_smol"
)))]
pub type Handler = fn(&Request) -> Response;
