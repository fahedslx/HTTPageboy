use std::collections::HashMap;
use std::fmt::{ Display, Formatter, Result };
use std::fs;
use std::io::Read;
use std::net::TcpStream;

use crate::request_type::Rt;
use crate::request_handler::Rh;
use crate::response::Response;
use crate::status_code::StatusCode;


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
	print!("{}", filepath);
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

pub fn handle_request(request: &Request, routes: &HashMap<String, Vec<(Rt, Rh)>>) -> Option<Response> {
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
	// If not in router, try with files.
	else if request.method == Rt::GET.to_string() {
		response = Some(handle_file_request(&request.path));
	}

	return response;
}
