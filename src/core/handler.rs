#![allow(unused_imports)]
use crate::Request;
use crate::Response;

#[cfg(feature = "sync")]
pub type Handler = fn(&Request) -> Response;

#[cfg(any(feature = "async_tokio", feature = "async_std", feature = "async_smol"))]
pub type Handler = fn(&Request) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>>;
