use httpageboy::test_utils::{run_test, setup_test_server, POOL_SIZE, SERVER_URL};
use httpageboy::{Request, Response, Rt, Server, StatusCode};

fn create_test_server() -> Server {
  let mut server = Server::new(SERVER_URL, POOL_SIZE, None).unwrap();

  server.add_route("/", Rt::GET, demo_handle_home);
  server.add_route("/test", Rt::GET, demo_handle_get);
  server.add_route("/test/{id}", Rt::POST, demo_handle_post);
  server.add_route("/test", Rt::PUT, demo_handle_put);
  server.add_route("/test", Rt::DELETE, demo_handle_delete);
  server.add_files_source("res");

  server
}

#[test]
fn test_home() {
  setup_test_server(|| create_test_server());
  let request = b"GET / HTTP/1.1\r\n\r\n";
  let expected_response = b"home";
  run_test(request, expected_response);
}

#[test]
fn test_get() {
  setup_test_server(|| create_test_server());
  let request = b"GET /test HTTP/1.1\r\n\r\n";
  let expected_response = b"get";
  run_test(request, expected_response);
}

#[test]
fn test_post() {
  setup_test_server(|| create_test_server());
  let request = b"POST /test/123 HTTP/1.1\r\n\r\n";
  let expected_response = b"post";
  // Capture the output of eprintln! in demo_handle_post
  let captured_output = run_test(request, expected_response);
  // Assert that the ID is present in the output
  assert!(captured_output.contains("ID: Some(\"123\")"));
  run_test(request, expected_response);
}

#[test]
fn test_put() {
  setup_test_server(|| create_test_server());
  let request = b"PUT /test HTTP/1.1\r\n\r\n";
  let expected_response = b"put";
  run_test(request, expected_response);
}

#[test]
fn test_delete() {
  setup_test_server(|| create_test_server());
  let request = b"DELETE /test HTTP/1.1\r\n\r\n";
  let expected_response = b"delete";
  run_test(request, expected_response);
}

#[test]
fn test_file_exists() {
  setup_test_server(|| create_test_server());
  let request = b"GET /test.png HTTP/1.1\r\nHost: localhost\r\n\r\n";
  let expected_response = b"HTTP/1.1 200 OK";
  run_test(request, expected_response);
}

#[test]
fn test_file_not_found() {
  setup_test_server(|| create_test_server());
  let request = b"GET /test1.png HTTP/1.1\r\n\r\n";
  let expected_response = b"HTTP/1.1 404 Not Found";
  run_test(request, expected_response);
}

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
  let request_string = format!(
    "Method: {}\nUri: {}\nHeaders: {:?}\nBody: {:?}\nParams: {:?}",
    _request.method, _request.path, _request.headers, _request.body, _request.params
  );
  eprintln!("{}", request_string);
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
fn test_post_no_route() {
  setup_test_server(|| create_test_server());
  let request = b"POST /unknown HTTP/1.1\r\n\r\n";
  let expected_response = b"POST request received without a specific route";
  run_test(request, expected_response);
}

#[test]
fn test_put_no_route() {
  setup_test_server(|| create_test_server());
  let request = b"PUT /unknown HTTP/1.1\r\n\r\n";
  let expected_response = b"PUT request received without a specific route";
  run_test(request, expected_response);
}

#[test]
fn test_delete_no_route() {
  setup_test_server(|| create_test_server());
  let request = b"DELETE /unknown HTTP/1.1\r\n\r\n";
  let expected_response = b"DELETE request received without a specific route";
  run_test(request, expected_response);
}
