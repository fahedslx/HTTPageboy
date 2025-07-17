use std::collections::HashMap;
use std::io::prelude::Write;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

use crate::core::request::{handle_request, Request};
pub use crate::core::request_handler::Rh;
pub use crate::core::request_type::Rt;
use crate::core::response::Response;
pub use crate::runtime::sync::threadpool::ThreadPool;

pub struct Server {
  listener: TcpListener,
  pool: Arc<Mutex<ThreadPool>>,
  routes: HashMap<(Rt, String), Rh>,
  files_sources: Vec<String>,
  auto_close: bool,
}

impl Server {
  pub fn new(
    serving_url: &str,
    pool_size: u8,
    routes_list: Option<HashMap<(Rt, String), Rh>>,
  ) -> Result<Server, std::io::Error> {
    let listener = TcpListener::bind(serving_url)?;
    let pool = Arc::new(Mutex::new(ThreadPool::new(pool_size as usize)));
    let routes = routes_list.unwrap_or_default();

    Ok(Server {
      listener,
      pool,
      routes,
      files_sources: Vec::new(),
      auto_close: true,
    })
  }

  pub fn set_auto_close(&mut self, state: bool) {
    self.auto_close = state;
  }

  pub fn add_route(&mut self, path: &str, rt: Rt, rh: fn(&Request) -> Response) {
    let key = (rt, path.to_string());
    let handler = Rh { handler: rh };
    self.routes.insert(key, handler);
  }

  pub fn add_files_source<S>(&mut self, base: S)
  where
    S: Into<String>,
  {
    self.files_sources.push(base.into());
  }

  pub fn run(&self) {
    {
      println!("Connection autoclose set to {:?}", self.auto_close);

      let addr = format!("http://{}", self.listener.local_addr().unwrap());
      let green_addr = format!("\x1b[32m{}\x1b[0m", addr);

      #[cfg(feature = "sync")]
      println!("Serving (sync) on {}", green_addr);

      #[cfg(feature = "async_tokio")]
      println!("Serving (async_tokio) on {}", green_addr);

      #[cfg(feature = "async_std")]
      println!("Serving (async_std) on {}", green_addr);

      #[cfg(feature = "async_smol")]
      println!("Serving (async_smol) on {}", green_addr);
    }

    for stream in self.listener.incoming() {
      match stream {
        Ok(stream) => {
          let routes_local = self.routes.clone();
          let sources_local = self.files_sources.clone();
          let close_flag = self.auto_close;
          let pool = Arc::clone(&self.pool);
          pool.lock().unwrap().run(move || {
            // 1. Leer request y posible respuesta de error temprana
            let (mut request, early_resp) = Request::parse_stream(&stream, &routes_local, &sources_local);
            // 2. Si hay respuesta temprana (400, 414, 505), la usamos
            let answer = if let Some(resp) = early_resp {
              Some(resp)
            } else {
              handle_request(&mut request, &routes_local, &sources_local)
            };
            match answer {
              Some(response) => Self::send_response(stream, &response, close_flag),
              None => Self::send_response(stream, &Response::new(), close_flag),
            }
          });
        }
        Err(_err) => {
          // podrías loguear el error aquí
        }
      }
    }
  }

  pub fn stop(&self) {
    let mut pool = self.pool.lock().unwrap();
    pool.stop();
  }

  fn send_response(mut stream: TcpStream, response: &Response, close: bool) {
    let connection_header = if close { "Connection: close\r\n" } else { "" };
    let header = format!(
      "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n{}\r\n",
      response.status,
      response.content_type,
      response.content.len(),
      connection_header
    );
    let _ = stream.write_all(header.as_bytes());

    if response.content_type.starts_with("image/") {
      let _ = stream.write_all(&response.content);
    } else {
      let text = String::from_utf8_lossy(&response.content);
      let _ = stream.write_all(text.as_bytes());
    }

    let _ = stream.flush();
    if close {
      let _ = stream.shutdown(Shutdown::Both);
    }
  }
}
