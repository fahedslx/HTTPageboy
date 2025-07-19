use std::io::{Read, Write};
use std::net::TcpStream;
#[cfg(any(
  feature = "sync",
  feature = "async_tokio",
  feature = "async_smol",
  feature = "async_std"
))]
use std::sync::Once;
#[allow(unused_imports)]
use std::thread;
use std::time::Duration;

#[cfg(feature = "sync")]
use crate::runtime::sync::server::Server;

#[cfg(feature = "async_tokio")]
use crate::runtime::r#async::tokio::Server;

#[cfg(feature = "async_smol")]
use crate::runtime::r#async::smol::Server;

#[cfg(feature = "async_std")]
use crate::runtime::r#async::async_std::Server;

pub const SERVER_URL: &str = "127.0.0.1:7878";
pub const POOL_SIZE: u8 = 10;
pub const INTERVAL: Duration = Duration::from_millis(250);
#[cfg(any(
  feature = "sync",
  feature = "async_tokio",
  feature = "async_smol",
  feature = "async_std"
))]
static INIT: Once = Once::new();

#[cfg(feature = "sync")]
pub fn setup_test_server<F>(server_factory: F)
where
  F: FnOnce() -> Server + Send + 'static,
{
  INIT.call_once(|| {
    let server = server_factory();
    thread::spawn(move || {
      server.run();
    });
    thread::sleep(INTERVAL);
  });
}

// async_tokio
#[cfg(feature = "async_tokio")]
pub async fn setup_test_server<F, Fut>(server_factory: F)
where
  F: FnOnce() -> Fut + Send + 'static,
  Fut: std::future::Future<Output = Server> + Send + 'static,
{
  INIT.call_once(|| {
    thread::spawn(move || {
      // Arranca un runtime Tokio en este hilo
      let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
      rt.block_on(async move {
        let server = server_factory().await;
        server.run().await;
      });
    });
    thread::sleep(INTERVAL);
  });
}

// async_std
#[cfg(feature = "async_std")]
pub async fn setup_test_server<F, Fut>(server_factory: F)
where
  F: FnOnce() -> Fut + Send + 'static,
  Fut: std::future::Future<Output = Server> + Send + 'static,
{
  INIT.call_once(|| {
    thread::spawn(move || {
      // Arranca async-std en este hilo
      async_std::task::block_on(async move {
        let server = server_factory().await;
        server.run().await;
      });
    });
    thread::sleep(INTERVAL);
  });
}

#[cfg(feature = "async_smol")]
pub async fn setup_test_server<F, Fut>(server_factory: F)
where
  F: FnOnce() -> Fut + Send + 'static,
  Fut: std::future::Future<Output = Server> + Send + 'static,
{
  INIT.call_once(|| {
    smol::spawn(async move {
      let server = server_factory().await;
      server.run().await;
    })
    .detach();
  });
  // Timer de smol para dar tiempo a bind()
  smol::Timer::after(INTERVAL).await;
}

pub fn run_test(request: &[u8], expected_response: &[u8]) -> String {
  let mut stream = TcpStream::connect(SERVER_URL).expect("Failed to connect to server");

  stream.write_all(request).unwrap();
  stream.shutdown(std::net::Shutdown::Write).unwrap();

  let mut buffer = Vec::new();
  stream.read_to_end(&mut buffer).unwrap();

  let buffer_string = String::from_utf8_lossy(&buffer).to_string();
  let expected_response_string = String::from_utf8_lossy(expected_response).to_string();

  assert!(
    buffer_string.contains(&expected_response_string),
    "ASSERT FAILED:\n\nRESPONSE: {} \nEXPECTED: {} \n\n",
    buffer_string,
    expected_response_string
  );
  buffer_string
}
