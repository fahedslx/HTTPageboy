use std::fmt::{ Display, Formatter, Result };

use crate::status_code::StatusCode;


pub struct Response {
	pub status: String,
	pub content_type: String,
	pub content: Vec<u8>
}

impl Display for Response {
	fn fmt (&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "{:#?}", self.content)
	}
}

impl Response {
	pub fn new() -> Response {
		Response {
			status: StatusCode::NotFound.to_string(),
			content_type: String::new(),
			content: "Not found.".as_bytes().to_vec(),
		}
	}
}
