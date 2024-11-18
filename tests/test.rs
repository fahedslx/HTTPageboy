#[cfg(test)]

mod tests {
	use std::sync::Once;
	use std::io::{Read, Write};
	use std::net::TcpStream;
	use std::thread;
	use std::time::Duration;
	extern crate httpageboy;
	use httpageboy::{ Server, Rt, Request, Response, StatusCode };

	const SERVER_URL: &str  = "127.0.0.1:7878";
	const POOL_SIZE: u8 = 10;
	const INTERVAL: Duration = Duration::from_millis(250);

	static INIT: Once = Once::new();

	fn setup() {
		INIT.call_once(|| {
			thread::spawn(|| {
				let mut server: Server = Server::new(SERVER_URL, POOL_SIZE, None)
					.expect("Failed to create server");
				server.add_route("/", Rt::GET, demo_handle_home);
				server.add_route("/test", Rt::GET, demo_handle_get);
				server.add_route("/test", Rt::POST, demo_handle_post);
				server.add_route("/test", Rt::PUT, demo_handle_put);
				server.add_route("/test", Rt::DELETE, demo_handle_delete);
				server.add_files_source("/res".to_string());
				server.add_files_source("./res".to_string());
				server.run();
			});
			thread::sleep(INTERVAL);
		});
	}

	fn test_server(request: &[u8], expected_response: &[u8]) {
		let mut stream: Option<TcpStream> = None;
		if let Ok(local_stream) = TcpStream::connect(SERVER_URL) {
			stream = Some(local_stream);
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
		assert!(buffer_string.contains(&expected_response_string),
			"ASSERT FAILED:\n\nRESPONSE: {} \nEXPECTED: {} \n\n", buffer_string, expected_response_string);
	}

	fn destroy() {
		// Lógica para detener el servidor si es necesario
	}

	fn run_test(request: &[u8], expected_response: &[u8]) {
		setup();
		test_server(request, expected_response);
		destroy();
	}

	pub fn demo_handle_home(_request: &Request) -> Response {
		return Response {
			status: StatusCode::Ok.to_string(),
			content_type: String::new(),
			content: "home".as_bytes().to_vec(),
		};
	}

	pub fn demo_handle_get(_request: &Request) -> Response {
		return Response {
			status: StatusCode::Ok.to_string(),
			content_type: String::new(),
			content: "get".as_bytes().to_vec(),
		};
	}

	pub fn demo_handle_post(_request: &Request) -> Response {
		return Response {
			status: StatusCode::Ok.to_string(),
			content_type: String::new(),
			content: "post".as_bytes().to_vec(),
		};
	}

	pub fn demo_handle_put(_request: &Request) -> Response {
		return Response {
			status: StatusCode::Ok.to_string(),
			content_type: String::new(),
			content: "put".as_bytes().to_vec(),
		};
	}

	pub fn demo_handle_delete(_request: &Request) -> Response {
		return Response {
			status: StatusCode::Ok.to_string(),
			content_type: String::new(),
			content: "delete".as_bytes().to_vec(),
		};
	}

	#[test]
	fn test_home() {
		let request = b"GET / HTTP/1.1\r\n\r\n";
		let expected_response = b"home";
		run_test(request, expected_response);
	}

	#[test]
	fn test_get() {
		let request = b"GET /test HTTP/1.1\r\n\r\n";
		let expected_response = b"get";
		run_test(request, expected_response);
	}

	#[test]
	fn test_post() {
		let request = b"POST /test HTTP/1.1\r\n\r\n";
		let expected_response = b"post";
		run_test(request, expected_response);
	}

	#[test]
	fn test_put() {
		let request = b"PUT /test HTTP/1.1\r\n\r\n";
		let expected_response = b"put";
		run_test(request, expected_response);
	}

	#[test]
	fn test_delete() {
		let request = b"DELETE /test HTTP/1.1\r\n\r\n";
		let expected_response = b"delete";
		run_test(request, expected_response);
	}

	#[test]
	fn test_file_true() {
		// Existing file
		let request = b"GET ./res/test.png HTTP/1.1\r\n\r\n";
		let expected_response = b"Length: 6743";
		run_test(request, expected_response);
	}

	#[test]
	fn test_file_false() {
		// Not existing file
		let request = b"GET /res/test.png HTTP/1.1\r\n\r\n";
		let expected_response = b"HTTP/1.1 404 Not Found\r\nContent-Type: \r\nContent-Length: 0\r\n\r\n";
		run_test(request, expected_response);
	}
}
