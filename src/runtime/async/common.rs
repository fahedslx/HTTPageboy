use crate::handle_request;
use crate::runtime::sync::server;
use crate::{Request, Response, Rh, Rt};
use std::collections::HashMap;

/// Trait that abstracts over sync or async runtimes
pub trait ServerRuntime {
  /// Arranca el listener y despacha cada conexión a `handle_conn`
  fn run(addr: &str, routes: HashMap<(Rt, String), Rh>, files: Vec<String>, threads: usize);
}

/// Función común para procesar una conexión
pub fn handle_conn(
  raw: impl std::io::Read + std::io::Write + Send + 'static,
  routes: &HashMap<(Rt, String), Rh>,
  files: &[String],
  auto_close: bool,
) {
  let stream = &raw; // abstrae tanto TcpStream como conexiones async
  let (mut req, early) = Request::parse_stream(stream, routes, files);
  let resp = early
    .unwrap_or_else(|| handle_request(&mut req, routes, files).unwrap_or_default());
  server::send_response(stream, &resp, auto_close);
}
