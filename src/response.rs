use std::fmt::{Display, Formatter, Result};

use crate::status_code::StatusCode;

#[derive(Debug)]
pub struct Response {
  pub status: String,
  pub content_type: String,
  pub content: Vec<u8>,
}

impl Default for Response {
  fn default() -> Self {
    Response {
      status: StatusCode::NotFound.to_string(),
      content_type: String::new(),
      content: b"Not found.".to_vec(),
    }
  }
}

impl Display for Response {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{:?}", self.content)
  }
}

impl Response {
  pub fn new() -> Self {
    Self::default()
  }
}
