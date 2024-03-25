use crate::request::Request;
use crate::response::Response;
use crate::status_code::StatusCode;


pub fn demo_handle_home_get (_request: &Request) -> Response {
	return Response {
		status: StatusCode::Ok.to_string(),
		content_type: String::new(),
		content: "home-get".as_bytes().to_vec(),
	}
}

pub fn demo_handle_test_get (_request: &Request) -> Response {
	return Response {
		status: StatusCode::Ok.to_string(),
		content_type: String::new(),
		content: "test-get".as_bytes().to_vec(),
	}
}
