use std::collections::HashMap;
use std::fmt::{ Display, Formatter, Result };
use std::io::Read;
use std::net::TcpStream;

use crate::request_type::Rt;
use crate::request_handler::Rh;
use crate::response::Response;
use crate::status_code::StatusCode;
use crate::utils::{ secure_path, get_content_type_quick };


pub struct Request {
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

pub fn stream_to_request(mut stream: &TcpStream) -> Request {
	let mut buffer: [u8; 1024] = [0; 1024];
	stream.read(&mut buffer).unwrap();
	let request = String::from_utf8_lossy(&buffer[..]);
	let request = request_disassembly(request.to_string());

	return request;
}

// En request.rs, handle_file_request:
pub fn handle_file_request(path: &String, allowed: &Vec<String>) -> Response {
	for base in allowed {
	  if let Some(real_path) = secure_path(base, path.as_str()) {
		match std::fs::read(&real_path) {
		  Ok(data) => {
			return Response {
			  status: StatusCode::Ok.to_string(),
			  content_type: get_content_type_quick(&real_path),
			  content: data,
			};
		  }
		  Err(_) => break,
		}
	  }
	}
	Response::new() // 404
  }
  
pub fn handle_request(request: &Request, routes: &HashMap<String, Rh>, files_sources: &Vec<String>) -> Option<Response> {
	println!("REQUEST:\n{}", request);
	let mut response: Option<Response> = None;

	let key = format!("{}|{}", request.path, request.method);
	// Try with router
	if let Some(handler) = routes.get(&key) {
		let handler_func = handler.handler;
		response = Some(handler_func(request));
	}
	// If not in router, try with files.
	else if request.method == Rt::GET.to_string() {
		response = Some(handle_file_request(&request.path, files_sources));
	}

	return response;
}
