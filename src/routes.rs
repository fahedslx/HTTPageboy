use std::collections::HashMap;

use crate::request_type::Rt;
use crate::request_handler::Rh;
use crate::request::Request;
use crate::response::Response;
use crate::status_code::StatusCode;


pub fn get_routes() -> HashMap<String, Vec<(Rt, Rh)>> {
	let mut routes: HashMap<String, Vec<(Rt, Rh)>> = HashMap::new();

	routes.insert(
		"/".to_string(),
		vec![
			(Rt::GET, Rh { handler: handle_home_get }),
			// Insert other routes here...
		],
	);
	routes.insert(
		"/test".to_string(),
		vec![
			(Rt::GET, Rh { handler: handle_test_get }),
			// Insert other routes here...
		],
	);

	return routes;
}


fn handle_home_get (_request: &Request) -> Response {
	return Response {
		status: StatusCode::Ok.to_string(),
		content_type: String::new(),
		content: "home-get".as_bytes().to_vec(),
	}
}

fn handle_test_get (_request: &Request) -> Response {
	return Response {
		status: StatusCode::Ok.to_string(),
		content_type: String::new(),
		content: "test-get".as_bytes().to_vec(),
	}
}
