// src/runtime/async/smol.rs
use super::common::{handle_conn, ServerRuntime};
use crate::core::{Rh, Rt};
use smol::Async;
use std::collections::HashMap;
use std::net::TcpListener as StdListener;

pub struct Server;

impl ServerRuntime for Server {
  fn run(addr: &str, routes: HashMap<(Rt, String), Rh>, files: Vec<String>, _threads: usize) {
    smol::block_on(async {
      let listener = Async::<StdListener>::bind(addr).unwrap();
      loop {
        let (stream, _) = listener.accept().await.unwrap();
        let routes = routes.clone();
        let files = files.clone();
        smol::spawn(async move {
          handle_conn(stream, &routes, &files, false);
        })
        .detach();
      }
    });
  }
}
