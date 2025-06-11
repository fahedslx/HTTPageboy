use crate::Server;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Once;
use std::thread;
use std::time::Duration;

pub const SERVER_URL: &str = "127.0.0.1:7878";
pub const POOL_SIZE: u8 = 10;
pub const INTERVAL: Duration = Duration::from_millis(250);
static INIT: Once = Once::new();

pub fn setup_test_server(server: Server) {
  INIT.call_once(|| {
    thread::spawn(move || {
      server.run();
    });
    thread::sleep(INTERVAL);
  });
}

pub fn test_server_request(request: &[u8], expected_response: &[u8]) {
  let mut stream = TcpStream::connect(SERVER_URL).expect("Failed to connect to server");

  stream.write_all(request).unwrap();
  stream.shutdown(std::net::Shutdown::Write).unwrap();

  let mut buffer = Vec::new();
  stream.read_to_end(&mut buffer).unwrap();

  let buffer_string = String::from_utf8_lossy(&buffer).to_string();
  let expected_response_string = String::from_utf8_lossy(expected_response).to_string();

  assert!(
    buffer_string.contains(&expected_response_string),
    "ASSERT FAILED:\\n\\nRESPONSE: {} \\nEXPECTED: {} \\n\\n",
    buffer_string,
    expected_response_string
  );
}

pub fn run_test(request: &[u8], expected_response: &[u8]) {
  test_server_request(request, expected_response);
}
