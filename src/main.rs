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

// Consolidated Server export
mod server_export {
  use cfg_if::cfg_if;

  cfg_if! {
    if #[cfg(feature = "sync")] {
      pub use crate::runtime::sync::server::Server;
    } else if #[cfg(feature = "async_tokio")] {
      pub use crate::runtime::r#async::tokio::Server;
    } else if #[cfg(feature = "async_smol")] {
      pub use crate::runtime::r#async::smol::Server;
    } else if #[cfg(feature = "async_std")] {
      pub use crate::runtime::r#async::async_std::Server;
    } else {
      // Fallback if no feature is active
      pub struct Server;

// DEFAULT (NO FEATURES)
#[cfg(all(
  not(feature = "sync"),
  not(feature = "async_tokio"),
  not(feature = "async_std"),
  not(feature = "async_smol")
))]
fn main() {
  eprintln!(
    "\n❌ No feature is active.\n\nActivate a feature when compiling:\n\n    cargo run --features sync\n    cargo run --features async_tokio\n    cargo run --features async_std\n    cargo run --features async_smol\n"
  );
}
pub use server_export::Server;
