use httpageboy::test_utils::{run_test, setup_test_server, POOL_SIZE, SERVER_URL};
use httpageboy::{Request, Response, Rt, Server, StatusCode};
use std::sync::Once;

static INIT: Once = Once::new();

fn create_test_server() -> Server {
  let mut server = Server::new(SERVER_URL, POOL_SIZE, None).unwrap();

  server.add_route("/", Rt::GET, demo_handle_home);
  server.add_route("/test", Rt::GET, demo_handle_get);
  server.add_route("/test", Rt::POST, demo_handle_post);
  server.add_route("/test", Rt::PUT, demo_handle_put);
  server.add_route("/test", Rt::DELETE, demo_handle_delete);
  server.add_files_source("res");

  server
}

/// Ensures the test server is started only once
fn ensure_server_running() {
  INIT.call_once(|| {
    let server = create_test_server();
    setup_test_server(server);
  });
}

// Handler functions for different routes
fn demo_handle_home(_request: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: "home".as_bytes().to_vec(),
  }
}

fn demo_handle_get(_request: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: "get".as_bytes().to_vec(),
  }
}

fn demo_handle_post(_request: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: "post".as_bytes().to_vec(),
  }
}

fn demo_handle_put(_request: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: "put".as_bytes().to_vec(),
  }
}

fn demo_handle_delete(_request: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: "delete".as_bytes().to_vec(),
  }
}

#[test]
fn test_home() {
  ensure_server_running();
  let request = b"GET / HTTP/1.1\r\n\r\n";
  let expected_response = b"home";
  run_test(request, expected_response);
}

#[test]
fn test_get() {
  ensure_server_running();
  let request = b"GET /test HTTP/1.1\r\n\r\n";
  let expected_response = b"get";
  run_test(request, expected_response);
}

#[test]
fn test_post() {
  ensure_server_running();
  let request = b"POST /test HTTP/1.1\r\n\r\n";
  let expected_response = b"post";
  run_test(request, expected_response);
}

#[test]
fn test_put() {
  ensure_server_running();
  let request = b"PUT /test HTTP/1.1\r\n\r\n";
  let expected_response = b"put";
  run_test(request, expected_response);
}

#[test]
fn test_delete() {
  ensure_server_running();
  let request = b"DELETE /test HTTP/1.1\r\n\r\n";
  let expected_response = b"delete";
  run_test(request, expected_response);
}

#[test]
fn test_file_exists() {
  ensure_server_running();
  let request = b"GET /test.png HTTP/1.1\r\nHost: localhost\r\n\r\n";
  let expected_response = b"HTTP/1.1 200 OK";
  run_test(request, expected_response);
}

#[test]
fn test_file_not_found() {
  ensure_server_running();
  let request = b"GET /test1.png HTTP/1.1\r\n\r\n";
  let expected_response = b"HTTP/1.1 404 Not Found";
  run_test(request, expected_response);
}
