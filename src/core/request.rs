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
        let name = part.trim_matches(|c| c == '{' || c == '}').to_string();
        params.insert(name, path_parts[i].to_string());
      } else if *part != path_parts[i] {
        return HashMap::new();
      }
    }
    params
  }

  pub fn from_stream(
    stream: &TcpStream,
    routes: &HashMap<(Rt, String), Rh>,
    file_bases: &[String],
  ) -> (Self, Option<Response>) {
    use std::io::{BufRead, BufReader, Read};
    let mut reader = BufReader::new(stream);
    let mut raw = String::new();
    loop {
      let mut line = String::new();
      if reader
        .read_line(&mut line)
        .ok()
        .filter(|&n| n > 0)
        .is_none()
      {
        break;
      }
      raw.push_str(&line);
      if raw.contains("\r\n\r\n") {
        break;
      }
    }
    let content_length = raw
      .lines()
      .find_map(|l| {
        if l.to_ascii_lowercase().starts_with("content-length:") {
          l.split(':').nth(1)?.trim().parse::<usize>().ok()
        } else {
          None
        }
      })
      .unwrap_or(0);
    if content_length > 0 {
      let mut buf = vec![0; content_length];
      if reader.read_exact(&mut buf).is_ok() {
        raw.push_str(&String::from_utf8_lossy(&buf));
      }
    }
    if raw.trim().is_empty() {
      let resp = Response {
        status: StatusCode::BadRequest.to_string(),
        content_type: String::new(),
        content: Vec::new(),
      };
      return (Self::default(), Some(resp));
    }
    let mut req = Self::from_raw(raw, routes);
    let res = req.handle_request(routes, file_bases);
    (req, res)
  }

  fn from_raw(raw: String, routes: &HashMap<(Rt, String), Rh>) -> Self {
    let lines: Vec<&str> = raw.split("\r\n").collect();
    let mut blank = 0;
    for (i, &l) in lines.iter().enumerate() {
      if l.trim().is_empty() {
        blank = i;
        break;
      }
    }
    let headers = lines[..blank]
      .iter()
      .filter_map(|&h| {
        let p: Vec<&str> = h.split(": ").collect();
        (p.len() == 2).then(|| (p[0].to_string(), p[1].to_string()))
      })
      .collect();
    let body = lines[blank + 1..].join("\r\n");
    let parts: Vec<&str> = raw.split_whitespace().collect();
    let mut path = parts[1].to_string();
    let mut params = HashMap::new();
    if let Some(q) = path.find('?') {
      for p in path[q + 1..].split('&') {
        if let Some(eq) = p.find('=') {
          params.insert(p[..eq].to_string(), p[eq + 1..].to_string());
        }
      }
      path = path[..q].to_string();
    }
    for ((m, rp), _rh) in routes {
      if *m == RequestType::from_str(parts[0]) {
        for (k, v) in Self::extract_path_params(rp, &path) {
          params.insert(k, v);
        }
        break;
      }
    }
    Request {
      method: RequestType::from_str(parts[0]),
      path,
      version: parts[2].to_string(),
      headers,
      body,
      params,
    }
  }

  pub fn handle_request(
    &mut self,
    routes: &HashMap<(Rt, String), Rh>,
    file_bases: &[String],
  ) -> Option<Response> {
    if let Some(rh) = routes.get(&(self.method.clone(), self.path.clone())) {
      return Some((rh.handler)(self));
    }
    for ((m, rp), rh) in routes {
      if *m == self.method {
        let ex = Self::extract_path_params(rp, &self.path);
        if !ex.is_empty() {
          self.params = ex;
          return Some((rh.handler)(self));
        }
      }
    }
    if self.method == Rt::GET {
      return Some(self.handle_file(file_bases));
    }
    None
  }

  fn handle_file(&self, allowed: &[String]) -> Response {
    for base in allowed {
      if let Some(real) = secure_path(base, &self.path) {
        if let Ok(data) = std::fs::read(&real) {
          return Response {
            status: StatusCode::Ok.to_string(),
            content_type: get_content_type_quick(&real),
            content: data,
          };
        }
      }
    }
    Response::new()
  }
}

impl Default for Request {
  fn default() -> Self {
    Request {
      method: RequestType::GET,
      path: String::new(),
      version: String::new(),
      headers: vec![],
      body: String::new(),
      params: HashMap::new(),
    }
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

pub fn handle_request(
  req: &mut Request,
  routes: &HashMap<(Rt, String), Rh>,
  file_bases: &[String],
) -> Option<Response> {
  req.handle_request(routes, file_bases)
}
