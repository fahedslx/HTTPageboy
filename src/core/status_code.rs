use std::fmt::{self, Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum StatusCode {
  Ok = 200,
  BadRequest = 400,
  Unauthorized = 401,
  Forbidden = 403,
  NotFound = 404,
  MethodNotAllowed = 405,
  UriTooLong = 414,
  InternalServerError = 500,
  HttpVersionNotSupported = 505,
}

impl Display for StatusCode {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let text = match self {
      StatusCode::Ok => "200 OK",
      StatusCode::BadRequest => "400 Bad Request",
      StatusCode::Unauthorized => "401 Unauthorized",
      StatusCode::Forbidden => "403 Forbidden",
      StatusCode::NotFound => "404 Not Found",
      StatusCode::MethodNotAllowed => "405 Method Not Allowed",
      StatusCode::UriTooLong => "414 URI Too Long",
      StatusCode::InternalServerError => "500 Internal Server Error",
      StatusCode::HttpVersionNotSupported => "505 HTTP Version Not Supported",
    };
    write!(f, "{}", text)
  }
}
