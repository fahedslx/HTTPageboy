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
  pub params: HashMap<String, String>,
}

impl Request {
  fn extract_path_params(route: &str, path: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();

    let route_parts: Vec<&str> = route.split('/').collect();
    let path_parts: Vec<&str> = path.split('/').collect();

    if route_parts.len() != path_parts.len() {
      return params;
    }

    for (i, part) in route_parts.iter().enumerate() {
      if part.starts_with('{') && part.ends_with('}') {
        let param_name = part.trim_matches(|c| c == '{' || c == '}').to_string();
        let param_value = path_parts[i].to_string();
        params.insert(param_name, param_value);
      } else if *part != path_parts[i] {
        return params;
      }
    }

    params
  }
}

impl Display for Request {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(
      f,
      "Method: {}\nPath: {}\nVersion: {}\nHeaders: {:#?},\nBody: {}\nParams: {:#?}",
      self.method, self.path, self.version, self.headers, self.body, self.params
    )
  }
}

pub fn stream_to_request(mut stream: &TcpStream, routes: &HashMap<(Rt, String), Rh>) -> Request {
  let mut raw = String::new();
  stream.read_to_string(&mut raw).unwrap();
  request_disassembly(raw, routes)
}

// TODO:  first extract query params, extract path params, extract the rest of parts
fn request_disassembly(request: String, routes: &HashMap<(Rt, String), Rh>) -> Request {
  let lines: Vec<&str> = request.split("\r\n").collect();

  // Headers
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
  let mut path: String = split_request[1].to_string();
  let version: String = split_request[2].to_string();

  // Params
  let mut params: HashMap<String, String> = HashMap::new();
  if let Some(query_start) = path.find('?') {
    let base_path = path[..query_start].to_string();
    let query_string = path[query_start + 1..].to_string();
    path = base_path;

    for param in query_string.split('&') {
      if let Some(eq_pos) = param.find('=') {
        let key = param[..eq_pos].to_string();
        let value = param[eq_pos + 1..].to_string();
        params.insert(key, value);
      }
    }
  }

  for ((route_method, route_path), _) in routes {
    if *route_method == method {
      let extracted_params = Request::extract_path_params(route_path, &path);
      if !extracted_params.is_empty() {
        for (key, value) in extracted_params {
          params.insert(key, value);
        }
      }
    }
  }

  return Request {
    method,
    path,
    version,
    headers,
    body,
    params,
  };
}

// TODO: handle as file, else return None
pub fn handle_file_request(path: &String, allowed: &[String]) -> Response {
  let mut output = Response::new();
  for base in allowed {
    if let Some(real_path) = secure_path(base, path.as_str()) {
      if let Ok(data) = std::fs::read(&real_path) {
        output = Response {
          status: StatusCode::Ok.to_string(),
          content_type: get_content_type_quick(&real_path),
          content: data,
        };
      }
      break;
    }
  }

  output
}

// TODO: check as route (in route.handler), else handle as file (handle_file_request), else return 404 (in thi function)
pub fn handle_request(
  req: &Request,
  routes: &HashMap<(Rt, String), Rh>,
  file_bases: &[String],
) -> Option<Response> {
  let output;
  if let Some(route) = routes.get(&(req.method.clone(), req.path.clone())) {
    output = Some((route.handler)(req));
  } else {
    output = Some(handle_file_request(&req.path, file_bases));
  }

  output
}
