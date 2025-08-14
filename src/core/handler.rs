#![allow(unused_imports)]
use crate::Request;
use crate::Response;

#[cfg(any(feature = "async_tokio", feature = "async_std", feature = "async_smol"))]
use std::{future::Future, pin::Pin};

// sync
#[cfg(feature = "sync")]
pub type Handler = fn(&Request) -> Response;

// async
#[cfg(any(feature = "async_tokio", feature = "async_std", feature = "async_smol"))]
pub type Handler = fn(&Request) -> Pin<Box<dyn Future<Output = Response> + Send>>;
