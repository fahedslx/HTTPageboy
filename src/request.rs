use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::io::Read;
use std::net::TcpStream;

use crate::request_handler::Rh;
use crate::request_type::{RequestType, Rt};
use crate::response::Response;
use crate::status_code::StatusCode;
use crate::utils::{get_content_type_quick, secure_path};

pub struct Request {
  pub method: RequestType,
  pub path: String,
  pub version: String,
  pub headers: Vec<(String, String)>,
  pub body: String,
}

impl Display for Request {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(
      f,
      "Method: {}\nPath: {}\nVersion: {}\nHeaders: {:#?},\nBody: {}\n",
      self.method, self.path, self.version, self.headers, self.body
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
  let method = RequestType::from_str(split_request[0]);
  let path: String = split_request[1].to_string();
  let version: String = split_request[2].to_string();

  return Request {
    method,
    path,
    version,
    headers,
    body,
  };
}

pub fn stream_to_request(mut stream: &TcpStream) -> Request {
  let mut buf = [0; 1024];
  stream.read(&mut buf).unwrap();
  let raw = String::from_utf8_lossy(&buf).to_string();
  request_disassembly(raw)
}

pub fn handle_file_request(path: &String, allowed: &[String]) -> Response {
  for base in allowed {
    if let Some(real_path) = secure_path(base, path.as_str()) {
      println!("📄 buscando archivo en: {:?}", real_path);
      if let Ok(data) = std::fs::read(&real_path) {
        return Response {
          status: StatusCode::Ok.to_string(),
          content_type: get_content_type_quick(&real_path),
          content: data,
        };
      } else {
        println!("❌ archivo no encontrado: {:?}", real_path);
      }
      break;
    }
  }
  Response::new()
}

pub fn handle_request(
  req: &Request,
  routes: &HashMap<(Rt, String), Rh>,
  file_bases: &[String],
) -> Option<Response> {
  let key = (req.method.clone(), req.path.clone());

  let mut output = None;
  if let Some(h) = routes.get(&key) {
    output = Some((h.handler)(req));
  } else if req.method == Rt::GET {
    output = Some(handle_file_request(&req.path, file_bases));
  }
  output
}
