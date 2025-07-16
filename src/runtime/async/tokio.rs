use std::collections::HashMap;
use std::sync::Arc;

use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

use crate::core::request::{handle_request, Request};
use crate::core::request_handler::Rh;
use crate::core::request_type::Rt;
use crate::core::response::Response;

pub struct Server {
  listener: TcpListener,
  routes: Arc<HashMap<(Rt, String), Rh>>,
  files_sources: Arc<Vec<String>>,
  auto_close: bool,
}

impl Server {
  /// Crea un servidor y enlaza la url
  pub async fn new(
    serving_url: &str,
    routes_list: Option<HashMap<(Rt, String), Rh>>,
  ) -> std::io::Result<Server> {
    let listener = TcpListener::bind(serving_url).await?;
    Ok(Server {
      listener,
      routes: Arc::new(routes_list.unwrap_or_default()),
      files_sources: Arc::new(Vec::new()),
      auto_close: false,
    })
  }

  /// Activa o desactiva cierre automático de conexión
  pub fn set_auto_close(&mut self, active: bool) {
    self.auto_close = active;
  }

  /// Agrega ruta HTTP
  pub fn add_route(&mut self, path: &str, rt: Rt, rh: fn(&Request) -> Response) {
    Arc::get_mut(&mut self.routes)
      .unwrap()
      .insert((rt, path.to_string()), Rh { handler: rh });
  }

  /// Agrega carpeta para servir archivos estáticos
  pub fn add_files_source<S>(&mut self, base: S)
  where
    S: Into<String>,
  {
    Arc::get_mut(&mut self.files_sources)
      .unwrap()
      .push(base.into());
  }

  /// Inicia el servidor
  pub async fn run(&self) {
    loop {
      let (mut stream, _) = match self.listener.accept().await {
        Ok(p) => p,
        Err(_) => continue,
      };
      let routes = self.routes.clone();
      let sources = self.files_sources.clone();
      let close_flag = self.auto_close;

      tokio::spawn(async move {
        let (mut req, early) = Request::parse_stream_async(&mut stream, &routes, &sources).await;
        let resp = early
          .or_else(|| handle_request(&mut req, &routes, &sources))
          .unwrap_or_else(Response::new);
        send_response(&mut stream, &resp, close_flag).await;
      });
    }
  }
}

async fn send_response(stream: &mut TcpStream, resp: &Response, close: bool) {
  let conn_hdr = if close { "Connection: close\r\n" } else { "" };
  let head = format!(
    "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n{}\
\r\n",
    resp.status,
    resp.content_type,
    resp.content.len(),
    conn_hdr,
  );
  let _ = stream.write_all(head.as_bytes()).await;
  if resp.content_type.starts_with("image/") {
    let _ = stream.write_all(&resp.content).await;
  } else {
    let text = String::from_utf8_lossy(&resp.content);
    let _ = stream.write_all(text.as_bytes()).await;
  }
  let _ = stream.flush().await;
  if close {
    let _ = stream.shutdown().await;
  }
}
