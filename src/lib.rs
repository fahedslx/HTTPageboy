pub mod core {
  pub mod request;
  pub mod request_handler;
  pub mod request_type;
  pub mod response;
  pub mod status_code;
  pub mod test_utils;
  pub mod utils;
}

pub use core::request::{handle_request, Request};
pub use core::request_handler::Rh;
pub use core::request_type::Rt;
pub use core::response::Response;
pub use core::status_code::StatusCode;
pub use core::test_utils;

pub type Handler = fn(&Request) -> Response;

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
}

#[cfg(feature = "sync")]
pub use runtime::sync::server::Server;

#[cfg(all(not(feature = "sync"), feature = "async_tokio"))]
pub use runtime::r#async::tokio::Server;

#[cfg(all(
  not(feature = "sync"),
  not(feature = "async_tokio"),
  feature = "async_smol"
))]
pub use runtime::r#async::smol::Server;

#[cfg(all(
  not(feature = "sync"),
  not(feature = "async_tokio"),
  not(feature = "async_smol"),
  feature = "async_std"
))]
pub use runtime::r#async::async_std::Server;
