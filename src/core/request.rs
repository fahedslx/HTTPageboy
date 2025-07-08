use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::net::TcpStream;

use crate::core::request_handler::Rh;
use crate::core::request_type::{RequestType, Rt};
use crate::core::response::Response;
use crate::core::status_code::StatusCode;
use crate::core::utils::{get_content_type_quick, secure_path};

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

// TODO: review
pub fn stream_to_request(stream: &TcpStream, routes: &HashMap<(Rt, String), Rh>) -> Request {
  use std::io::{BufRead, BufReader, Read};

  let mut reader = BufReader::new(stream);
  let mut raw = String::new();

  loop {
    let mut line = String::new();
    match reader.read_line(&mut line) {
      Ok(0) => break,
      Ok(_) => {
        raw.push_str(&line);
        if raw.contains("\r\n\r\n") {
          break;
        }
      }
      Err(_) => break,
    }
  }

  let content_length = {
    let headers_part = raw.clone();
    headers_part
      .lines()
      .find_map(|line| {
        if line.to_ascii_lowercase().starts_with("content-length:") {
          line.split(':').nth(1)?.trim().parse::<usize>().ok()
        } else {
          None
        }
      })
      .unwrap_or(0)
  };

  if content_length > 0 {
    let mut body_buf = vec![0; content_length];
    if reader.read_exact(&mut body_buf).is_ok() {
      raw.push_str(&String::from_utf8_lossy(&body_buf));
    }
  }

  request_disassembly(raw, routes)
}

fn request_disassembly(raw: String, routes: &HashMap<(Rt, String), Rh>) -> Request {
  // 1) separar head/body
  let (head, body) = split_head_body(&raw);

  // 2) parsear línea de request
  let (method, uri, version) = match parse_request_line(head) {
    Ok(t) => t,
    Err(resp) => return make_error_request(resp),
  };

  // 3) headers
  let headers = parse_headers(head);

  // 4) cuerpo
  let body = body.to_string();

  // 5) params de ruta y query
  let mut params = HashMap::new();
  let (path_for_routes, query) = split_path_query(&uri);
  extract_path_params(method, path_for_routes, routes, &mut params);
  if let Some(qs) = query {
    extract_query_params(qs, &mut params);
  }

  Request {
    method,
    path: path_for_routes.to_string(),
    version,
    headers,
    body,
    params,
  }
}

// --- auxiliares ---

/// 1) separa antes/después de "\r\n\r\n"
fn split_head_body(raw: &str) -> (&str, &str) {
  raw.split_once("\r\n\r\n").unwrap_or((raw, ""))
}

/// 2) parsea y valida la primera línea
fn parse_request_line(head: &str) -> Result<(RequestType, String, String), Response> {
  let mut parts = head.lines().next().unwrap_or("").split_whitespace();
  let m = parts.next().unwrap_or("");
  let u = parts.next().unwrap_or("");
  let v = parts.next().unwrap_or("");
  // 2.1) sintaxis básica
  if m.is_empty() || u.is_empty() || v.is_empty() {
    return Err(bad_request());
  }
  let method = RequestType::from_str(m);
  // 2.2) versión
  if v != "HTTP/1.1" {
    return Err(version_not_supported());
  }
  // 2.3) longitud de URI
  if u.len() > 8192 {
    return Err(uri_too_long());
  }
  Ok((method, u.to_string(), v.to_string()))
}

/// 3) headers en Vec<(String,String)>
fn parse_headers(head: &str) -> Vec<(String, String)> {
  head
    .lines()
    .skip(1)
    .filter_map(|l| {
      l.split_once(": ")
        .map(|(k, v)| (k.to_string(), v.to_string()))
    })
    .collect()
}

/// 5a) separa ruta limpia y query
fn split_path_query(uri: &str) -> (&str, Option<&str>) {
  uri
    .split_once('?')
    .map_or((uri, None), |(p, q)| (p, Some(q)))
}

/// 5b) extrae params de ruta
fn extract_path_params(
  method: RequestType,
  path: &str,
  routes: &HashMap<(Rt, String), Rh>,
  params: &mut HashMap<String, String>,
) {
  for ((rt, rp), rh) in routes {
    if *rt == method {
      let extracted = Request::extract_path_params(rp, path);
      for (k, v) in extracted {
        params.insert(k, v);
      }
      break;
    }
  }
}

/// 5c) extrae params de query
fn extract_query_params(qs: &str, params: &mut HashMap<String, String>) {
  for pair in qs.split('&') {
    if let Some((k, v)) = pair.split_once('=') {
      params.insert(k.to_string(), v.to_string());
    }
  }
}

// --- funciones para generar Responses de error ---

fn make_error_request(resp: Response) -> Request {
  // se devuelve un Request con método GET y el body=error response
  // tu Server debe detectar esto y enviar directamente `resp`
  Request {
    method: RequestType::GET,
    path: String::new(),
    version: String::new(),
    headers: Vec::new(),
    body: String::new(),
    params: HashMap::new(),
  }
}

fn bad_request() -> Response {
  Response {
    status: StatusCode::BadRequest.to_string(),
    content_type: String::new(),
    content: Vec::new(),
  }
}
fn version_not_supported() -> Response {
  Response {
    status: StatusCode::HttpVersionNotSupported.to_string(),
    content_type: String::new(),
    content: Vec::new(),
  }
}
fn uri_too_long() -> Response {
  Response {
    status: StatusCode::UriTooLong.to_string(),
    content_type: String::new(),
    content: Vec::new(),
  }
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

pub fn handle_request(
  req: &mut Request, // notar mut para poder escribir params
  routes: &HashMap<(Rt, String), Rh>,
  file_bases: &[String],
) -> Option<Response> {
  // exact match
  if let Some(route) = routes.get(&(req.method.clone(), req.path.clone())) {
    return Some((route.handler)(req));
  }

  // match with path params
  for ((route_method, route_path), rh) in routes {
    if *route_method == req.method {
      let path_params = Request::extract_path_params(route_path, &req.path);
      if !path_params.is_empty() {
        req.params = path_params;
        return Some((rh.handler)(req));
      }
    }
  }

  // fallback to files
  if req.method == Rt::GET {
    return Some(handle_file_request(&req.path, file_bases));
  }

  None
}
