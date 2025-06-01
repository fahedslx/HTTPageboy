use std::collections::HashMap;
use std::io::prelude::Write;
use std::net::{TcpListener, TcpStream};
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
}

impl Server {
  pub fn new(
    serving_url: &str,
    pool_size: u8,
    routes_list: Option<HashMap<(Rt, String), Rh>>,
  ) -> Result<Server, std::io::Error> {
    let listener = TcpListener::bind(serving_url)?;
    let pool = Arc::new(Mutex::new(ThreadPool::new(pool_size as usize)));
    let routes: HashMap<(Rt, String), Rh>;

    if let Some(routes_list) = routes_list {
      routes = routes_list;
    } else {
      routes = HashMap::new();
    }

    return Ok(Server {
      listener,
      routes,
      pool,
      files_sources: Vec::new(),
    });
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
          let pool = Arc::clone(&self.pool);
          pool.lock().unwrap().run(move || {
            let request: Request = stream_to_request(&stream);
            let answer: Option<Response> = handle_request(&request, &routes_local, &sources_local);
            match answer {
              Some(response) => {
                send_response(stream, &response);
              }
              None => {
                send_response(stream, &Response::new());
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

fn send_response(mut stream: TcpStream, response: &Response) {
  let header = format!(
    "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
    response.status,
    response.content_type,
    response.content.len()
  );
  stream.write(header.as_bytes()).unwrap();
  if response.content_type.starts_with("image/") {
    stream.write(&response.content).unwrap();
  } else {
    stream
      .write(String::from_utf8_lossy(&response.content).as_bytes())
      .unwrap();
  }
  stream.flush().unwrap();
}
