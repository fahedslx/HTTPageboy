use std::collections::HashMap;
use std::sync::Arc;

use futures_lite::io::AsyncWriteExt;
use smol::net::{TcpListener, TcpStream};
use smol::spawn;

use crate::core::request::{handle_request, Request};
use crate::core::request_handler::Rh;
use crate::core::request_type::Rt;
use crate::core::response::Response;

/// A non‑blocking HTTP server powered by Smol + async-net.
pub struct Server {
  listener: TcpListener,
  routes: Arc<HashMap<(Rt, String), Rh>>,
  file_sources: Arc<Vec<String>>,
  auto_close: bool,
}

impl Server {
  /// Bind to `serving_url` and prepare route map.
  pub async fn new(
    serving_url: &str,
    routes_list: Option<HashMap<(Rt, String), Rh>>,
  ) -> std::io::Result<Self> {
    let listener = TcpListener::bind(serving_url).await?;
    Ok(Self {
      listener,
      routes: Arc::new(routes_list.unwrap_or_default()),
      file_sources: Arc::new(Vec::new()),
      auto_close: false,
    })
  }

  /// Enable or disable `Connection: close` header.
  pub fn set_auto_close(&mut self, active: bool) {
    self.auto_close = active;
  }

  /// Add a handler for `method` + `path`.
  pub fn add_route(&mut self, path: &str, method: Rt, handler: fn(&Request) -> Response) {
    Arc::get_mut(&mut self.routes)
      .unwrap()
      .insert((method, path.to_string()), Rh { handler });
  }

  /// Add a directory for static file serving.
  pub fn add_files_source<S: Into<String>>(&mut self, base: S) {
    Arc::get_mut(&mut self.file_sources)
      .unwrap()
      .push(base.into());
  }

  /// Start accepting connections and dispatching handlers.
  pub async fn run(&self) {
    loop {
      let (mut stream, _) = match self.listener.accept().await {
        Ok(pair) => pair,
        Err(_) => continue,
      };
      let routes = self.routes.clone();
      let files = self.file_sources.clone();
      let close_flag = self.auto_close;

      spawn(async move {
        let (mut req, early) = Request::parse_stream_async(&mut stream, &routes, &files).await;
        let resp = early
          .or_else(|| handle_request(&mut req, &routes, &files))
          .unwrap_or_else(Response::new);
        send_response(&mut stream, &resp, close_flag).await;
      })
      .detach();
    }
  }
}

async fn send_response(stream: &mut TcpStream, resp: &Response, close: bool) {
  let conn_hdr = if close { "Connection: close\r\n" } else { "" };
  let header = format!(
    "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n{}\
\r\n",
    resp.status,
    resp.content_type,
    resp.content.len(),
    conn_hdr,
  );

  let _ = stream.write_all(header.as_bytes()).await;
  if resp.content_type.starts_with("image/") {
    let _ = stream.write_all(&resp.content).await;
  } else {
    let text = String::from_utf8_lossy(&resp.content);
    let _ = stream.write_all(text.as_bytes()).await;
  }
  let _ = stream.flush().await;

  if close {
    let _ = stream.close().await;
  }
}
