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
#[allow(dead_code)]
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
	handler: fn(request: &Request) -> Response,
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


fn handle_home_get (_request: &Request) -> Response {
	return Response {
		status: StatusCode::Ok.to_string(),
		content_type: String::new(),
		content: "home-get".as_bytes().to_vec(),
	}
}

#[allow(dead_code)]
fn handle_home_post (_request: &Request) -> Response {
	return Response {
		status: StatusCode::Ok.to_string(),
		content_type: String::new(),
		content: "home-post".as_bytes().to_vec(),
	}
}

fn handle_test_get (_request: &Request) -> Response {
	return Response {
		status: StatusCode::Ok.to_string(),
		content_type: String::new(),
		content: "test-get".as_bytes().to_vec(),
	}
}

#[allow(dead_code)]
fn handle_test_post (_request: &Request) -> Response {
	return Response {
		status: StatusCode::Ok.to_string(),
		content_type: String::new(),
		content: "test-post".as_bytes().to_vec(),
	}
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
			"Method: {}\nPath: {}\nVersion: {}\nHeaders: {:#?},\nBody: {}\n",
			self.method,
			self.path,
			self.version,
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

	let temp_headers = lines[..blank_line_index].join("\r\n");
	let mut parsed_headers = Vec::new();
	for header_line in temp_headers.lines() {
		let header_parts: Vec<&str> = header_line.split(": ").collect();
		if header_parts.len() == 2 {
			let header_name = header_parts[0].to_string();
			let header_value = header_parts[1].to_string();
			parsed_headers.push((header_name, header_value));
		}
	}
	let headers = parsed_headers;
	let body = lines[blank_line_index + 1..].join("\r\n");
	let split_request: Vec<&str> = request.split_whitespace().collect();
	let method: String = split_request[0].to_string();
	let path: String = split_request[1].to_string();
	let version: String = split_request[2].to_string();

	return Request {
		method,
		path,
		version,
		headers,
		body
	};
}

fn get_content_type_quick(filename: &String) -> String {
	let extension: Option<&str> = filename.split('.').last();

	let content_type: &str = match extension {
		Some("png") => "image/png",
		Some("jpg") | Some("jpeg") => "image/jpeg",
		Some("gif") => "image/gif",
		Some("bmp") => "image/bmp",
		Some("svg") => "image/svg+xml",
		Some("webp") => "image/webp",
		Some("html") => "text/html",
		Some("css") => "text/css",
		Some("js") => "application/javascript",
		Some("json") => "application/json",
		Some("xml") => "application/xml",
		Some("pdf") => "application/pdf",
		Some("doc") | Some("docx") => "application/msword",
		Some("xls") | Some("xlsx") => "application/vnd.ms-excel",
		Some("ppt") | Some("pptx") => "application/vnd.ms-powerpoint",
		Some("zip") => "application/zip",
		Some("rar") => "application/x-rar-compressed",
		Some("txt") => "text/plain",
		Some("csv") => "text/csv",
		Some("mp3") => "audio/mpeg",
		Some("wav") => "audio/wav",
		Some("mp4") => "video/mp4",
		Some("avi") => "video/x-msvideo",
		Some("mov") => "video/quicktime",
		Some("ogg") => "audio/ogg",
		Some("ogv") => "video/ogg",
		Some("oga") => "audio/ogg",
		Some("ico") => "image/x-icon",
		_ => "application/octet-stream",
	};

	return content_type.to_string();
}

fn handle_file_request(filepath: &String) -> Response {
	let mut status: String = StatusCode::NotFound.to_string();
	let mut content_type: String = String::new();
	let mut content: Vec<u8> = Vec::new();

	let data = fs::read(filepath);
	match data {
		Ok(data) => {
			status = StatusCode::Ok.to_string();
			content = data;
			content_type = get_content_type_quick(&filepath);
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
	let mut response: Option<Response> = None;

	let temp_content_type = get_content_type_quick(&request.path);
	if temp_content_type == "application/octet-stream" {
		let route = routes.get(&request.path);
		if let Some(route) = route {
			for (rt, rh) in route {
				if rt == &Rt::GET {
					response = Some((rh.handler)(request));
				}
			}
		}
	}
	else if request.method == "GET" {
		response = Some(handle_file_request(&request.path));
	}

	return response;
}


struct Response {
	status: String,
	content_type: String,
	content: Vec<u8>
}

impl Display for Response {
	fn fmt (&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "{:#?}", self.content)
	}
}

impl Response {
	fn new() -> Response {
		Response {
			status: StatusCode::NotFound.to_string(),
			content_type: String::new(),
			content: "Not found.".as_bytes().to_vec(),
		}
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


fn serve (listener: TcpListener, pool: ThreadPool, routes: HashMap<String, Vec<(Rt, Rh)>>) {
	for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				println!("Serving.");
				let routes_local = routes.clone();
				pool.execute(move || {
					let request: Request = stream_to_request(&stream);
					println!("{}", &request);
					let answer: Option<Response> = handle_request(&request, &routes_local);
					match answer {
						Some(response) => {
							send_response(stream, &response);
						}
						None => {
							send_response(stream, &Response::new());
						}
					}
				});
			}
			Err(err) => {
				println!("Error: {}", err);
			}
		}
	}
}


// Main
fn main() {
	let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
	let pool = ThreadPool::new(10);
	let routes: HashMap<String, Vec<(Rt, Rh)>> = get_routes();

	serve(listener, pool, routes);
}
