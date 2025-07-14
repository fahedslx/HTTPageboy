use super::common::Server;
use crate::core::{request_handler::Rh, request_type::Rt};
use std::collections::HashMap;

pub use Server;

// Mantener la misma interfaz que la versión síncrona
pub fn run_server(
  addr: &str,
  pool_size: u8,
  routes: Option<HashMap<(Rt, String), Rh>>,
) -> Result<(), std::io::Error> {
  let server = Server::new(addr, pool_size, routes)?;
  server.run();
  Ok(())
}
