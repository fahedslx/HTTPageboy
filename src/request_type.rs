use std::fmt::{self, Display, Formatter};

pub type Rt = RequestType;

#[derive(Clone, PartialEq, Debug)]
pub enum RequestType {
  GET,
  POST,
  PUT,
  DELETE,
  HEAD,
  OPTIONS,
  CONNECT,
  PATCH,
}

impl Display for RequestType {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}
