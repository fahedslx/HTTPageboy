use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
mod utils {
	pub mod threadpool;
}
// use http::request;
use utils::threadpool::ThreadPool;

// Statuses that will be returned.
enum StatusCode {
	Ok = 200,
	BadRequest = 400,
	Unauthorized = 401,
	Forbidden = 403,
	NotFound = 404,
	MethodNotAllowed = 405,
	InternalServerError = 500,
}

impl ToString for StatusCode {
	fn to_string (&self) -> String {
		let status_code = match self {
			StatusCode::Ok => "200 OK",
			StatusCode::BadRequest => "400 Bad Request",
			StatusCode::Unauthorized => "401 Unauthorized",
			StatusCode::Forbidden => "403 Forbidden",
			StatusCode::NotFound => "404 Not Found",
			StatusCode::MethodNotAllowed => "405 Method Not Allowed",
			StatusCode::InternalServerError => "500 Internal Server Error",
		}.to_string();

		return status_code;
	}
}


// Define request types that will be handled.
type Rt = RequestType;

enum RequestType {
	GET,
	POST,
	PUT,
	DELETE,
	HEAD,
	OPTIONS,
	CONNECT,
	PATCH,
}

impl ToString for RequestType {
	fn to_string (&self) -> String {
		let request_type = match self {
			Rt::GET => "GET",
			Rt::POST => "POST",
			Rt::PUT => "PUT",
			Rt::DELETE => "DELETE",
			Rt::HEAD => "HEAD",
			Rt::OPTIONS => "OPTIONS",
			Rt::CONNECT => "CONNECT",
			Rt::PATCH => "PATCH",
		}.to_string();

		return request_type;
	}
}

impl PartialEq for RequestType {
	fn eq (&self, other: &Self) -> bool {
		return self.to_string() == other.to_string();
	}		
}


// Functions that will handle requests.
type Rh = RequestHandler;

struct RequestHandler {
	handler: fn(request: &[u8]) -> (String, String, Vec<u8>),
}


// Define routes that will be handled.
fn get_routes() -> HashMap<String, Vec<(Rt, Rh)>> {
	let mut routes: HashMap<String, Vec<(RequestType, RequestHandler)>> = HashMap::new();
	
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


fn handle_home_get (_request: &[u8]) -> (String, String, Vec<u8>) {
	return (StatusCode::Ok.to_string(), String::new(), "home-get".as_bytes().to_vec());
}

fn handle_home_post (_request: &[u8]) -> (String, String, Vec<u8>) {
	return (StatusCode::Ok.to_string(), String::new(), "home-post".as_bytes().to_vec());
}

fn handle_test_get (_request: &[u8]) -> (String, String, Vec<u8>) {
	return (StatusCode::Ok.to_string(), String::new(), "test-get".as_bytes().to_vec());
}

fn handle_test_post (_request: &[u8]) -> (String, String, Vec<u8>) {
	return (StatusCode::Ok.to_string(), String::new(), "test-post".as_bytes().to_vec());
}


// Definition and implementation of Request struct.
struct Request {
	method: String,
	path: String,
	version: String,
	headers: Vec<(String, String)>,
	body: String
}


fn request_disassembly(request: String) -> Request {
	let split_request: Vec<&str> = request.split_whitespace().collect();
	let method: String = split_request[0].to_string();
	let path: String = split_request[1].to_string();
	let version: String = split_request[2].to_string();
	let headers = Vec::new();
	let body = String::new();

	return Request {
		method,
		path,
		version,
		headers,
		body
	};
}


fn get_request_content_type (file: &String) -> String {
	let mut content_type: &str;

	if file.contains(".png HTTP/1.1") {
		content_type = "image/png";
	}
	else if file.contains(".jpg HTTP/1.1") {
		content_type = "image/jpeg";
	}
	else if file.contains(".svg HTTP/1.1") {
		content_type = "image/svg+xml";
	}
	else if file.contains(".webp HTTP/1.1") {
		content_type = "image/webp";
	}
	else if file.contains(".html HTTP/1.1") {
		content_type = "text/html";
	}
	else if file.contains(".css HTTP/1.1") {
		content_type = "text/css";
	}
	else if file.contains(".js HTTP/1.1") {
		content_type = "application/javascript";
	}
	else if file.contains(".json HTTP/1.1") {
		content_type = "application/json";
	}
	else if file.contains(".xml HTTP/1.1") {
		content_type = "application/xml";
	}
	else {
		content_type = "text/plain";
	}

	return content_type.to_string();
}


fn handle_file_request(request: &str) -> (String, String, Vec<u8>) {
	let mut status = StatusCode::NotFound.to_string();
	let mut content_type = String::new();
	let mut content = Vec::new();

	match fs::read(request) {
		Ok(data) => {
			status = StatusCode::Ok.to_string();
			content = data;
			content_type = get_request_content_type(&request.to_string());
		}
		Err(_) => {
			println!("{}", status);
		}
	}

	return (status, content_type, content);
}


fn handle_function_request (_request: &[u8]) {}


fn handle_request(mut stream: TcpStream) {
	let mut buffer: [u8; 1024] = [0; 1024];
	stream.read(&mut buffer).unwrap();
	let request = String::from_utf8_lossy(&buffer[..]);
	let request = request_disassembly(request.to_string());

	let _status: String;
	let mut _content_type = String::new();

	if ROUTES.contains_key(&request.path) {
		let route = ROUTES.get(&request.path);
		if let Some(route) = route {
			for (request_type, _) in route {
				println!("Request type: {}", request_type.to_string());
			}
		}
	}
}


fn send_response(mut stream: TcpStream, status: String, content: Vec<u8>, content_type: String) {
	let header = format!(
		"HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
		status,
		content_type,
		content.len()
	);
	stream.write(header.as_bytes()).unwrap();
	if content_type.starts_with("image/") {
		stream.write(&content).unwrap();
	}
	else {
		stream.write(String::from_utf8_lossy(&content).as_bytes()).unwrap();
	}
	stream.flush().unwrap();
}


// Main
fn main() {
	let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
	let pool = ThreadPool::new(10);
	let mut routes: HashMap<String, Vec<(Rt, Rh)>> = HashMap::new();

	for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				pool.execute(move || {
					handle_request(stream.try_clone().unwrap());
					// send_response(stream, status, content, content_type);
				});
			}
			Err(err) => {
				println!("Error: {}", err);
			}
		}
	}
}
