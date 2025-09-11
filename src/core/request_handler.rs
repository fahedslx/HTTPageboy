#[cfg(any(
  feature = "sync",
  feature = "async_tokio",
  feature = "async_std",
  feature = "async_smol"
))]
mod request_handler_enabled {
  use crate::core::handler::Handler;

  pub type Rh = RequestHandler;

  pub struct RequestHandler {
    pub handler: Handler,
  }

  #[allow(clippy::derivable_impls)]
  impl Clone for RequestHandler {
    fn clone(&self) -> Self {
      RequestHandler {
        handler: self.handler.clone(),
      }
    }
  }

  impl std::fmt::Debug for RequestHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("RequestHandler").finish()
    }
  }
}

#[cfg(any(
  feature = "sync",
  feature = "async_tokio",
  feature = "async_std",
  feature = "async_smol"
))]
pub use request_handler_enabled::*;
