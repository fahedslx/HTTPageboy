// src/runtime/async/tokio.rs
use super::common::{handle_conn, ServerRuntime};
use crate::core::{Rh, Rt};
use crate::runtime::sync::server::send_response;
use std::collections::HashMap;
use tokio::net::TcpListener; // reusa el mismo envío

pub struct Server;

impl ServerRuntime for Server {
  fn run(addr: &str, routes: HashMap<(Rt, String), Rh>, files: Vec<String>, _threads: usize) {
    let rt = tokio::runtime::Builder::new_multi_thread()
      .enable_all()
      .build()
      .unwrap();
    rt.block_on(async move {
      let listener = TcpListener::bind(addr).await.unwrap();
      loop {
        let (stream, _) = listener.accept().await.unwrap();
        let routes = routes.clone();
        let files = files.clone();
        tokio::spawn(async move {
          handle_conn(stream, &routes, &files, false);
        });
      }
    });
  }
}
