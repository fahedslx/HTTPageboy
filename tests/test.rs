#[cfg(test)]

use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread::{ self, JoinHandle };
use std::time::Duration;
use std::collections::HashMap;

extern crate eqeqo_server_base;
use eqeqo_server_base::{ ServerBase, Rt, Rh, Request, Response, StatusCode };


const SERVER_URL: &str  = "127.0.0.1:7878";
const POOL_SIZE: u8 = 10;
const INTERVAL: Duration = Duration::from_millis(250);


pub fn demo_handle_home (_request: &Request) -> Response {
	return Response {
		status: StatusCode::Ok.to_string(),
		content_type: String::new(),
		content: "home".as_bytes().to_vec(),
	}
}

pub fn demo_handle_get (_request: &Request) -> Response {
	return Response {
		status: StatusCode::Ok.to_string(),
		content_type: String::new(),
		content: "get".as_bytes().to_vec(),
	}
}

pub fn demo_handle_post (_request: &Request) -> Response {
	return Response {
		status: StatusCode::Ok.to_string(),
		content_type: String::new(),
		content: "post".as_bytes().to_vec(),
	}
}

pub fn demo_handle_put (_request: &Request) -> Response {
	return Response {
		status: StatusCode::Ok.to_string(),
		content_type: String::new(),
		content: "put".as_bytes().to_vec(),
	}
}

pub fn demo_handle_delete (_request: &Request) -> Response {
	return Response {
		status: StatusCode::Ok.to_string(),
		content_type: String::new(),
		content: "delete".as_bytes().to_vec(),
	}
}


fn run_server(serving_url: &str, pool_size: u8, routes_list: Option<HashMap<String, Rh>>) -> JoinHandle<()> {
	let server_result = ServerBase::new(serving_url, pool_size, routes_list);
	let mut server = match server_result {
		Ok(server) => server,
		Err(e) => panic!("Error while creating server: {}", e),
	};

	server.add_route("/", Rt::GET, demo_handle_home);
	server.add_route("/test", Rt::GET, demo_handle_get);
	server.add_route("/test", Rt::POST, demo_handle_post);
	server.add_route("/test", Rt::PUT, demo_handle_put);
	server.add_route("/test", Rt::DELETE, demo_handle_delete);

	return thread::spawn(move || {
		server.serve();
	});
}

fn test_server(request:&[u8], expected_response:&[u8] ) {
	let server: JoinHandle<()> = thread::spawn(|| {
		run_server(SERVER_URL, POOL_SIZE, None);
	});
	thread::sleep(INTERVAL);

	#[allow(unused_assignments)]
	let mut stream: Option<TcpStream> = None;
	if let Ok(local_stream) = TcpStream::connect(SERVER_URL) {
		println!("{:?}", local_stream);
		stream = Some(local_stream);
	}
	else {
		panic!("Failed to connect");
	}
	let mut stream = stream.expect("Failed to connect to server");

	let local_request = request;
	stream.write_all(local_request).unwrap();
	let mut buffer = Vec::new();
	stream.read_to_end(&mut buffer).unwrap();
	println!(
		"\n\nRESPONSE: {:#?} \nEXPECTED: {:#?} \n\n",
		String::from_utf8_lossy(&buffer),
		String::from_utf8_lossy(&expected_response)
	);
	let buffer_string = String::from_utf8_lossy(&buffer).to_string();
	let expected_response_string = String::from_utf8_lossy(expected_response).to_string();
	assert!(buffer_string.contains(&expected_response_string));

	server.join().unwrap();
}









#[test]
fn test_home() {
	let request = b"GET / HTTP/1.1\r\n\r\n";
	let expected_response = b"home";
	test_server(request, expected_response);
}

#[test]
fn test_get() {
	let request = b"GET /test HTTP/1.1\r\n\r\n";
	let expected_response = b"get";
	test_server(request, expected_response);
}

#[test]
fn test_post() {
	let request = b"POST /test HTTP/1.1\r\n\r\n";
	let expected_response = b"post";
	test_server(request, expected_response);
}

#[test]
fn test_put() {
	let request = b"PUT /test HTTP/1.1\r\n\r\n";
	let expected_response = b"put";
	test_server(request, expected_response);
}

#[test]
fn test_delete() {
	let request = b"DELETE /test HTTP/1.1\r\n\r\n";
	let expected_response = b"delete";
	test_server(request, expected_response);
}

#[test]
fn test_file() {
	let request = b"GET /res/test.png HTTP/1.1\r\n\r\n";
	let expected_response = b"Length: 6743";
	test_server(request, expected_response);
}
