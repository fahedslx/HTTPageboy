// src/runtime/async/async_std.rs
use super::common::{handle_conn, ServerRuntime};
use crate::core::{Rh, Rt};
use async_std::net::TcpListener;
use std::collections::HashMap;

pub struct Server;

impl ServerRuntime for Server {
  fn run(addr: &str, routes: HashMap<(Rt, String), Rh>, files: Vec<String>, _threads: usize) {
    async_std::task::block_on(async {
      let listener = TcpListener::bind(addr).await.unwrap();
      loop {
        let (stream, _) = listener.accept().await.unwrap();
        let routes = routes.clone();
        let files = files.clone();
        async_std::task::spawn(async move {
          handle_conn(stream, &routes, &files, false);
        });
      }
    });
  }
}
