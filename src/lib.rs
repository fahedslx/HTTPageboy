pub mod core;
pub mod macros;

// Always available re-exports
pub use crate::core::{request_type::Rt, response::Response, status_code::StatusCode, test_utils};

// Re-exports only when any handler feature is enabled
#[cfg(any(
  feature = "sync",
  feature = "async_tokio",
  feature = "async_std",
  feature = "async_smol"
))]
pub use crate::core::{
  handler::Handler,
  request::{Request, handle_request},
  request_handler::Rh,
};

pub mod runtime {
  #[cfg(feature = "sync")]
  pub mod sync {
    pub mod server;
    pub mod threadpool;
  }

  #[cfg(any(feature = "async_tokio", feature = "async_smol", feature = "async_std"))]
  pub mod r#async {
    #[cfg(feature = "async_std")]
    pub mod async_std;
    #[cfg(feature = "async_smol")]
    pub mod smol;
    #[cfg(feature = "async_tokio")]
    pub mod tokio;
  }

  pub mod shared;
}

// Consolidated Server export (single block)
mod server_export {
  // Priority: sync > tokio > smol > async_std (unchanged)
  #[cfg(feature = "sync")]
  pub use crate::runtime::sync::server::Server;

  #[cfg(all(not(feature = "sync"), feature = "async_tokio"))]
  pub use crate::runtime::r#async::tokio::Server;

  #[cfg(all(not(feature = "sync"), not(feature = "async_tokio"), feature = "async_smol"))]
  pub use crate::runtime::r#async::smol::Server;

  #[cfg(all(
    not(feature = "sync"),
    not(feature = "async_tokio"),
    not(feature = "async_smol"),
    feature = "async_std"
  ))]
  pub use crate::runtime::r#async::async_std::Server;

  // Fallback if no feature is active
  #[cfg(all(
    not(feature = "sync"),
    not(feature = "async_tokio"),
    not(feature = "async_smol"),
    not(feature = "async_std")
  ))]
  pub struct Server;

  #[cfg(all(
    not(feature = "sync"),
    not(feature = "async_tokio"),
    not(feature = "async_smol"),
    not(feature = "async_std")
  ))]
  impl Server {
    /// Panics with guidance when no feature is selected
    pub fn new() -> Self {
      eprintln!(
        "\n❌ No feature is active.\n\nActivate one when compiling:\n\n  cargo run --features sync\n  cargo run --features async_tokio\n  cargo run --features async_std\n  cargo run --features async_smol\n"
      );
      panic!("No feature selected.");
    }
  }
}
pub use server_export::Server;
