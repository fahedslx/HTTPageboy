use std::collections::HashMap;
use std::fmt::{ Display, Formatter, Result };
use std::fs;
use std::io::prelude::*;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
mod utils {
	pub mod threadpool;
}
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

impl Clone for RequestType {
	fn clone(&self) -> Self {
		match self {
			Rt::GET => Rt::GET,
			Rt::POST => Rt::POST,
			Rt::PUT => Rt::PUT,
			Rt::DELETE => Rt::DELETE,
			Rt::HEAD => Rt::HEAD,
			Rt::OPTIONS => Rt::OPTIONS,
			Rt::CONNECT => Rt::CONNECT,
			Rt::PATCH => Rt::PATCH,
		}
	}
}


// Functions that will handle requests.
type Rh = RequestHandler;

struct RequestHandler {
	handler: fn(request: &[u8]) -> (String, String, Vec<u8>),
}

impl Clone for RequestHandler {
	fn clone(&self) -> Self {
		RequestHandler { handler: self.handler }
	}
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

impl Display for Request {
	fn fmt (&self, f: &mut Formatter<'_>) -> Result {
		write!(
			f,
			"{} {}\n{:#?}\n{}",
			self.method,
			self.path,
			self.headers,
			self.body
		)
	}
}

fn stream_to_request(mut stream: &TcpStream) -> Request {
	let mut buffer: [u8; 1024] = [0; 1024];
	stream.read(&mut buffer).unwrap();
	let request = String::from_utf8_lossy(&buffer[..]);
	let request = request_disassembly(request.to_string());

	return request;
}

fn request_disassembly(request: String) -> Request {
	// Divide la solicitud en líneas
	let lines: Vec<&str> = request.split("\r\n").collect();

	// Busca el índice de la línea en blanco que separa los headers del body
	let mut blank_line_index = 0;
	for (i, line) in lines.iter().enumerate() {
		if line.trim().is_empty() {
			blank_line_index = i;
			break;
		}
	}

	let headers = lines[..blank_line_index].join("\r\n");
	let body = lines[blank_line_index + 1..].join("\r\n");

	let split_request: Vec<&str> = request.split_whitespace().collect();
	let method: String = split_request[0].to_string();
	let path: String = split_request[1].to_string();
	let version: String = split_request[2].to_string();

	let mut parsed_headers = Vec::new();
	for header_line in headers.lines() {
			let header_parts: Vec<&str> = header_line.split(": ").collect();
			if header_parts.len() == 2 {
					let header_name = header_parts[0].to_string();
					let header_value = header_parts[1].to_string();
					parsed_headers.push((header_name, header_value));
			}
	}

	return Request {
		method,
		path,
		version,
		headers: parsed_headers,
		body
	};
}

fn get_request_content_type (file: &String) -> String {
	let content_type: &str;

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

fn handle_file_request(request: &str) -> Response {
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

	return Response{
		status, content_type, content
	};
}

fn handle_request(request: &Request, routes: &HashMap<String, Vec<(Rt, Rh)>>) -> Option<Response> {
	let _status: String = StatusCode::Ok.to_string();
	let _content_type: String = String::new();
	let _content: Vec<u8> = Vec::new();
	let response: Response = Response {
		status: _status,
		content_type: _content_type,
		content: _content
	};

	return Some(response);
}


struct Response {
	status: String,
	content_type: String,
	content: Vec<u8>
}

impl Display for Response {
	fn fmt (&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "{}", self.content.len())
	}
}

fn send_response(mut stream: TcpStream, response: &Response) {
	let header = format!(
		"HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
		response.status,
		response.content_type,
		response.content.len()
	);
	stream.write(header.as_bytes()).unwrap();
	if response.content_type.starts_with("image/") {
		stream.write(&response.content).unwrap();
	}
	else {
		stream.write(String::from_utf8_lossy(&response.content).as_bytes()).unwrap();
	}
	stream.flush().unwrap();
}


// Main
fn main() {
	let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
	let pool = ThreadPool::new(10);
	let routes: HashMap<String, Vec<(Rt, Rh)>> = get_routes();

	for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				let routes_local = routes.clone();
				pool.execute(move || {
					let request: Request = stream_to_request(&stream);
					println!("{} - {}", 1, &request);
					let response = handle_request(&request, &routes_local).unwrap();
					send_response(stream, &response);
				});
			}
			Err(err) => {
				println!("Error: {}", err);
			}
		}
	}
}
