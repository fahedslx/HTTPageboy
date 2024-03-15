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
