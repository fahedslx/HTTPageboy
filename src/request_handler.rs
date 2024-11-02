use std::fmt::Debug;

use crate::request::Request;
use crate::response::Response;

pub type Rh = RequestHandler;

pub struct RequestHandler {
	pub handler: fn(request: &Request) -> Response,
}

impl Clone for RequestHandler {
	fn clone(&self) -> Self {
		RequestHandler { handler: self.handler }
	}
}

impl Debug for RequestHandler {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("RequestHandler").finish()
	}
}