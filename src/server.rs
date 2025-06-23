use std::collections::HashMap;
use std::io::prelude::Write;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

use crate::request::{handle_request, stream_to_request, Request};
pub use crate::request_handler::Rh;
pub use crate::request_type::Rt;
use crate::response::Response;
pub use crate::threadpool::ThreadPool;

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
    let routes = if let Some(r) = routes_list {
      r
    } else {
      HashMap::new()
    };

    Ok(Server {
      listener,
      pool,
      routes,
      files_sources: Vec::new(),
      auto_close: true,
    })
  }

  pub fn set_auto_close(&mut self, active: bool) {
    self.auto_close = active;
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
    for stream in self.listener.incoming() {
      match stream {
        Ok(stream) => {
          let routes_local = self.routes.clone();
          let sources_local = self.files_sources.clone();
          let close_flag = self.auto_close;
          let pool = Arc::clone(&self.pool);
          let routes = self.routes.clone();
          pool.lock().unwrap().run(move || {
            let mut request: Request = stream_to_request(&stream, &routes); // Pass routes here
            let answer: Option<Response> =
              handle_request(&mut request, &routes_local, &sources_local);
            match answer {
              Some(response) => {
                send_response(stream, &response, close_flag);
              }
              None => {
                send_response(stream, &Response::new(), close_flag);
              }
            }
          });
        }
        Err(err) => {
          println!("Error: {}", err);
        }
      }
    }
  }

  pub fn stop(&self) {
    let mut pool = self.pool.lock().unwrap();
    println!("server stop");
    pool.stop();
  }
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
  stream.write_all(header.as_bytes()).unwrap();
  if response.content_type.starts_with("image/") {
    stream.write_all(&response.content).unwrap();
  } else {
    stream
      .write_all(String::from_utf8_lossy(&response.content).as_bytes())
      .unwrap();
  }
  stream.flush().unwrap();

  if close {
    let _ = stream.shutdown(Shutdown::Both);
  }
}
