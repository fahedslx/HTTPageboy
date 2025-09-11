#[cfg(any(
  feature = "sync",
  feature = "async_tokio",
  feature = "async_std",
  feature = "async_smol"
))]
use crate::{Request, Response};

#[cfg(any(feature = "async_tokio", feature = "async_std", feature = "async_smol"))]
use std::{future::Future, pin::Pin};

#[cfg(feature = "sync")]
pub type Handler = fn(&Request) -> Response;

#[cfg(any(feature = "async_tokio", feature = "async_std", feature = "async_smol"))]
pub type Handler = fn(&Request) -> Pin<Box<dyn Future<Output = Response> + Send>>;
