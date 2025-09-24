use crate::core::handler::Handler;
use crate::core::request::{handle_request_async, Request};
use crate::core::request_handler::Rh;
use crate::core::request_type::Rt;
use crate::core::response::Response;
use crate::runtime::shared::print_server_info;
use crate::runtime::r#async::shared::{send_response, AsyncStream};
use async_trait::async_trait;
use async_std::io::prelude::*;
use async_std::net::{Shutdown, TcpListener, TcpStream};
use async_std::task::spawn;
use std::collections::HashMap;
use std::sync::Arc;

#[async_trait]
impl AsyncStream for TcpStream {
    async fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        WriteExt::write_all(self, buf).await
    }

    async fn flush(&mut self) -> std::io::Result<()> {
        WriteExt::flush(self).await
    }

    async fn shutdown(&mut self) -> std::io::Result<()> {
        std::future::ready(async_std::net::TcpStream::shutdown(self, Shutdown::Both)).await
    }
}


/// A non‑blocking HTTP server powered by async‑std.
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
      auto_close: true,
    })
  }

  /// Enable or disable `Connection: close` header.
  pub fn set_auto_close(&mut self, active: bool) {
    self.auto_close = active;
  }

  /// Add a handler for `method` + `path`.
  pub fn add_route(&mut self, path: &str, method: Rt, handler: Arc<dyn Handler>) {
    Arc::get_mut(&mut self.routes)
      .unwrap()
      .insert((method, path.to_string()), Rh { handler });
  }

  /// Add a directory for static file serving.
  pub fn add_files_source<S: Into<String>>(&mut self, base: S) {
    Arc::get_mut(&mut self.file_sources).unwrap().push(base.into());
  }

  /// Start accepting connections and dispatching handlers.
  pub async fn run(&self) {
    print_server_info(self.listener.local_addr().unwrap(), self.auto_close);
    while let Ok((mut stream, _)) = self.listener.accept().await {
      let routes = self.routes.clone();
      let files = self.file_sources.clone();
      let close_flag = self.auto_close;

      spawn(async move {
        let (mut req, early) = Request::parse_stream_async_std(&mut stream, &routes, &files).await;
        let resp = match early {
          Some(r) => r,
          None => handle_request_async(&mut req, &routes, &files)
            .await
            .unwrap_or_else(Response::new),
        };
        send_response(&mut stream, &resp, close_flag).await;
      });
    }
  }
}
