use std::fmt::Debug;

use crate::core::handler::Handler;

#[cfg(any(
  feature = "sync",
  feature = "async_tokio",
  feature = "async_std",
  feature = "async_smol"
))]

pub type Rh = RequestHandler;

pub struct RequestHandler {
  pub handler: Handler,
}

impl Clone for RequestHandler {
  fn clone(&self) -> Self {
    RequestHandler {
      handler: self.handler.clone(),
    }
  }
}

impl Debug for RequestHandler {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("RequestHandler").finish()
  }
}
