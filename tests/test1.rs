#[cfg(test)]

use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread::{ self, JoinHandle };
use std::time::Duration;
use std::collections::HashMap;
use std::sync::Once;

extern crate eqeqo_server_base;
use eqeqo_server_base::{ ServerBase, Rt, Rh, Request, Response, StatusCode };


const SERVER_URL: &str  = "127.0.0.1:7878";
const INTERVAL: Duration = Duration::from_millis(250);
static INIT: Once = Once::new();


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


fn run_server(serving_url: &str, pool_size: u8, routes_list: Option<HashMap<String, Vec<(Rt, Rh)>>>) -> JoinHandle<()> {
	let server_result = ServerBase::new(serving_url, pool_size, routes_list);
	let mut server = match server_result {
		Ok(server) => server,
		Err(e) => panic!("Error while creating server: {}", e),
	};

	server.add_route("/", Rt::GET, demo_handle_home_get);
	server.add_route("/test", Rt::GET, demo_handle_test_get);

	return thread::spawn(move || {
		server.serve();
	});
}

fn test_server(request:&[u8], expected_response:&[u8] ) {
	let _server: JoinHandle<()> = thread::spawn(|| {
		run_server(SERVER_URL, 10, None);
	});
	thread::sleep(INTERVAL);

	let mut stream: Option<TcpStream> = None;
	if let Ok(local_stream) = TcpStream::connect(SERVER_URL) {
		println!("{:?}", local_stream);
		stream = Some(local_stream);
	}
	else {
		println!("Failed to connect");
	}
	let mut stream = stream.expect("Failed to connect to server");
	
	let local_request = request;
	stream.write_all(local_request).unwrap();
	let mut buffer = Vec::new();
	stream.read_to_end(&mut buffer).unwrap();
	println!("\n\nRESPONSE: {:#?} \nRECEIVED: {:#?} \n\n",
		String::from_utf8_lossy(&buffer), String::from_utf8_lossy(&expected_response));
	let buffer_string = String::from_utf8_lossy(&buffer).to_string();
	let expected_response_string = String::from_utf8_lossy(expected_response).to_string();
	assert!(buffer_string.contains(&expected_response_string));
}

fn setup() {
	INIT.call_once(|| {
		let _ = thread::spawn(|| {
			run_server(SERVER_URL, 10, None);
		});
		thread::sleep(INTERVAL);
	});
}


#[test]
fn test_home() {
	setup();
	let request = b"GET / HTTP/1.1\r\n\r\n";
	let expected_response = b"home-get";
	test_server(request, expected_response);
}

#[test]
fn test_test() {
	setup();
	let request = b"GET /test HTTP/1.1\r\n\r\n";
	let expected_response = b"test-get";
	test_server(request, expected_response);
}

#[test]
fn test_file() {
	setup();
	let request = b"GET /res/test.png HTTP/1.1\r\n\r\n";
	let expected_response = b"Length: 6743";
	test_server(request, expected_response);
}
