use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;


fn main() {
	let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
	for stream in listener.incoming() {
		let stream = stream.unwrap();
		handle_connection(stream);
	}
}

fn handle_connection(mut stream: TcpStream) {
	let mut buffer = [0; 1024];
	let get = b"GET / HTTP/1.1\r\n";
	let status: &str;
	let content: String;
	
	stream.read(&mut buffer).unwrap();

	if buffer.starts_with(get) {
		status = "200 OK";
		content = fs::read_to_string("res/html/index.html").unwrap();
	}
	else {
		status = "404 Not Found";
		content = fs::read_to_string("res/html/404.html").unwrap();
	}

	let response = format!(
		"HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
		status,
		content.len(),
		content
	);
	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
}
