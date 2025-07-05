pub mod core {
  pub mod request;
  pub mod request_handler;
  pub mod request_type;
  pub mod response;
  pub mod status_code;
  pub mod test_utils;
  pub mod utils;
}

pub use core::request::{handle_request, stream_to_request, Request};
pub use core::request_handler::Rh;
pub use core::request_type::Rt;
pub use core::response::Response;
pub use core::status_code::StatusCode;
pub use core::test_utils;

pub type Handler = fn(&Request) -> Response;

#[cfg(feature = "sync")]
pub mod runtime {
  pub mod sync {
    pub mod server;
    pub mod threadpool;
  }
}

#[cfg(feature = "async_tokio")]
pub mod runtime {
  pub mod async_tokio {
    pub mod tokio;
  }
}

#[cfg(feature = "async_smol")]
pub mod runtime {
  pub mod async_smol {
    pub mod smol;
  }
}

#[cfg(feature = "async_std")]
pub mod runtime {
  pub mod async_std {
    pub mod async_std;
  }
}

#[cfg(feature = "sync")]
pub use runtime::sync::server::Server;
#[cfg(feature = "sync")]
pub use runtime::sync::threadpool::ThreadPool;

#[cfg(feature = "async_smol")]
pub use runtime::async_smol::smol::{Server, ThreadPool};
#[cfg(feature = "async_std")]
pub use runtime::async_std::async_std::{Server, ThreadPool};
#[cfg(feature = "async_tokio")]
pub use runtime::async_tokio::tokio::{Server, ThreadPool};
