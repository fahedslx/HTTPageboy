use std::collections::HashMap;
use server_base::{ ServerBase, Rt, Rh, Request, Response, StatusCode };


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


fn run_server(serving_url: &str, pool_size: u8, routes_list: Option<HashMap<String, Vec<(Rt, Rh)>>>) {
	let mut server = ServerBase::new(serving_url, pool_size, routes_list);

	server.add_route("/", Rt::GET, demo_handle_home_get);
	server.add_route("/test", Rt::GET, demo_handle_test_get);

	server.serve();
}


fn main() {
	run_server("127.0.0.1:7878", 10, None);
}
