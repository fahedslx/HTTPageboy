mod request;
mod request_handler;
mod request_type;
mod response;
mod server;
mod status_code;
mod threadpool;
mod utils;

pub use request::{handle_request, stream_to_request, Request};
pub use request_handler::Rh;
pub use request_type::Rt;
pub use response::Response;
pub use server::Server;
pub use status_code::StatusCode;

pub type Handler = fn(&Request) -> Response;
