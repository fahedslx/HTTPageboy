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
  pub async fn new(
    serving_url: &str,
    routes_list: Option<HashMap<(Rt, String), Rh>>,
  ) -> std::io::Result<Server> {
    let listener = TcpListener::bind(serving_url).await?;
    let routes = Arc::new(routes_list.unwrap_or_default());

    Ok(Server {
      listener,
      routes,
      files_sources: Arc::new(Vec::new()),
      auto_close: false,
    })
  }

  pub fn set_auto_close(&mut self, active: bool) {
    self.auto_close = active;
  }

  pub fn add_route(&mut self, path: &str, rt: Rt, rh: fn(&Request) -> Response) {
    Arc::get_mut(&mut self.routes)
      .unwrap()
      .insert((rt, path.to_string()), Rh { handler: rh });
  }

  pub fn add_files_source<S>(&mut self, base: S)
  where
    S: Into<String>,
  {
    Arc::get_mut(&mut self.files_sources)
      .unwrap()
      .push(base.into());
  }

  pub async fn run(&self) {
    loop {
      let (stream, _) = match self.listener.accept().await {
        Ok(s) => s,
        Err(_) => continue,
      };

      let routes = self.routes.clone();
      let sources = self.files_sources.clone();
      let close_flag = self.auto_close;

      tokio::spawn(async move {
        let (mut request, early_resp) = Request::parse_stream(&stream, &routes, &sources).await;

        let answer = if let Some(resp) = early_resp {
          Some(resp)
        } else {
          handle_request(&mut request, &routes, &sources)
        };

        match answer {
          Some(response) => send_response(stream, &response, close_flag).await,
          None => send_response(stream, &Response::new(), close_flag).await,
        }
      });
    }
  }
}

async fn send_response(mut stream: TcpStream, response: &Response, close: bool) {
  let connection_header = if close { "Connection: close\r\n" } else { "" };
  let header = format!(
    "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n{}\r\n",
    response.status,
    response.content_type,
    response.content.len(),
    connection_header
  );
  let _ = stream.write_all(header.as_bytes()).await;

  if response.content_type.starts_with("image/") {
    let _ = stream.write_all(&response.content).await;
  } else {
    let text = String::from_utf8_lossy(&response.content);
    let _ = stream.write_all(text.as_bytes()).await;
  }

  let _ = stream.flush().await;
  if close {
    let _ = stream.shutdown().await;
  }
}
