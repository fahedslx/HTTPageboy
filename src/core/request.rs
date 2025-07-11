use std::collections::HashMap;
use std::net::TcpStream;

use crate::core::request_handler::Rh;
use crate::core::request_type::{RequestType, Rt};
use crate::core::response::Response;
use crate::core::status_code::StatusCode;
use crate::core::utils::{get_content_type_quick, secure_path};

/// Represents an HTTP request.
pub struct Request {
  pub method: RequestType,
  pub path: String,
  pub version: String,
  pub headers: Vec<(String, String)>,
  pub body: String,
  pub params: HashMap<String, String>,
}

impl Request {
  /// Extracts path parameters according to a route pattern.
  fn extract_params(route: &str, path: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();
    let route_parts: Vec<&str> = route.split('/').collect();
    let path_parts: Vec<&str> = path.split('/').collect();
    if route_parts.len() != path_parts.len() {
      return params;
    }
    for (i, part) in route_parts.iter().enumerate() {
      if part.starts_with('{') && part.ends_with('}') {
        let key = part.trim_matches(|c| c == '{' || c == '}').to_string();
        params.insert(key, path_parts[i].to_string());
      } else if *part != path_parts[i] {
        return HashMap::new();
      }
    }
    params
  }

  /// Reads from the stream, handles early errors, and returns a Request plus optional error Response.
  pub fn parse_stream(
    stream: &TcpStream,
    routes: &HashMap<(Rt, String), Rh>,
    file_bases: &[String],
  ) -> (Self, Option<Response>) {
    use std::io::{BufRead, BufReader, Read};

    let mut reader = BufReader::new(stream);
    let mut raw = String::new();

    // read headers
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

    // read body
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
      let _ = reader.read_exact(&mut buf);
      raw.push_str(&String::from_utf8_lossy(&buf));
    } else {
      let mut rest = String::new();
      let _ = reader.read_to_string(&mut rest);
      raw.push_str(&rest);
    }

    // early error: empty or malformed request
    if raw.trim().is_empty() {
      let resp = Response {
        status: StatusCode::BadRequest.to_string(),
        content_type: String::new(),
        content: Vec::new(),
      };
      return (Self::default(), Some(resp));
    }
    let parts: Vec<&str> = raw.split_whitespace().collect();
    if parts.len() < 3 {
      let resp = Response {
        status: StatusCode::BadRequest.to_string(),
        content_type: String::new(),
        content: Vec::new(),
      };
      return (Self::default(), Some(resp));
    }

    let method_str = parts[0];
    let path_str = parts[1];
    let version = parts[2];

    // early error: method not allowed
    let allowed = ["GET", "POST", "PUT", "DELETE"];
    if !allowed.contains(&method_str) {
      let resp = Response {
        status: StatusCode::MethodNotAllowed.to_string(),
        content_type: String::new(),
        content: Vec::new(),
      };
      return (Self::default(), Some(resp));
    }
    // early error: unsupported HTTP version
    if version != "HTTP/1.1" {
      let resp = Response {
        status: StatusCode::HttpVersionNotSupported.to_string(),
        content_type: String::new(),
        content: Vec::new(),
      };
      return (Self::default(), Some(resp));
    }
    // early error: URI too long
    const MAX_URI: usize = 2000;
    if path_str.len() > MAX_URI {
      let resp = Response {
        status: StatusCode::UriTooLong.to_string(),
        content_type: String::new(),
        content: Vec::new(),
      };
      return (Self::default(), Some(resp));
    }

    // normal request parsing
    let mut req = Self::parse_raw(raw, routes);
    let early = req.route(routes, file_bases);
    (req, early)
  }

  /// Parses raw HTTP text into a Request, extracting headers, body, path, and parameters.
  fn parse_raw(raw: String, routes: &HashMap<(Rt, String), Rh>) -> Self {
    let lines: Vec<&str> = raw.split("\r\n").collect();
    let mut cut = 0;
    for (i, &l) in lines.iter().enumerate() {
      if l.trim().is_empty() {
        cut = i;
        break;
      }
    }

    let headers = lines[..cut]
      .iter()
      .filter_map(|&h| {
        let p: Vec<&str> = h.split(": ").collect();
        (p.len() == 2).then(|| (p[0].to_string(), p[1].to_string()))
      })
      .collect();

    let body = lines[cut + 1..].join("\r\n");
    let parts: Vec<&str> = raw.split_whitespace().collect();

    // separate query string
    let mut path = parts[1].to_string();
    let mut params = HashMap::new();
    let query_opt = if let Some(qpos) = path.find('?') {
      let qs = path[qpos + 1..].to_string();
      path.truncate(qpos);
      Some(qs)
    } else {
      None
    };

    // extract route parameters
    for ((m, rp), _) in routes {
      if *m == RequestType::from_str(parts[0]) {
        for (k, v) in Self::extract_params(rp, &path) {
          params.insert(k, v);
        }
        break;
      }
    }

    // insert query parameters
    if let Some(qs) = query_opt {
      for p in qs.split('&') {
        if let Some(eq) = p.find('=') {
          params.insert(p[..eq].to_string(), p[eq + 1..].to_string());
        }
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

  /// Routes the request or serves a file for GET if no route matches.
  pub fn route(
    &mut self,
    routes: &HashMap<(Rt, String), Rh>,
    file_bases: &[String],
  ) -> Option<Response> {
    if let Some(rh) = routes.get(&(self.method.clone(), self.path.clone())) {
      return Some((rh.handler)(self));
    }
    for ((m, rp), rh) in routes {
      if *m == self.method {
        let path_p = Self::extract_params(rp, &self.path);
        if !path_p.is_empty() {
          let mut merged = HashMap::new();
          for (k, v) in path_p {
            merged.insert(k, v);
          }
          for (k, v) in self.params.drain() {
            merged.insert(k, v);
          }
          self.params = merged;
          return Some((rh.handler)(self));
        }
      }
    }
    if self.method == Rt::GET {
      return Some(self.serve_file(file_bases));
    }
    None
  }

  /// Serves a static file from allowed bases or returns 404.
  fn serve_file(&self, bases: &[String]) -> Response {
    for base in bases {
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

use std::fmt::{Display, Formatter};

/// Allows printing a Request in the required format with sorted parameters.
impl Display for Request {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let mut keys: Vec<&String> = self.params.keys().collect();
    keys.sort();
    let params_str = {
      let parts: Vec<String> = keys
        .into_iter()
        .map(|k| format!("\"{}\": \"{}\"", k, self.params[k]))
        .collect();
      format!("{{{}}}", parts.join(", "))
    };

    write!(
      f,
      "Method: {}\n\
       Path: {}\n\
       Version: {}\n\
       Headers: {:#?},\n\
       Body: {}\n\
       Params: {}",
      self.method, self.path, self.version, self.headers, self.body, params_str
    )
  }
}

/// Routes the Request or serves static files.
pub fn handle_request(
  req: &mut Request,
  routes: &HashMap<(Rt, String), Rh>,
  file_bases: &[String],
) -> Option<Response> {
  req.route(routes, file_bases)
}
