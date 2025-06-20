use httpageboy::test_utils::{run_test, setup_test_server, POOL_SIZE, SERVER_URL};
use httpageboy::{Request, Response, Rt, Server, StatusCode};

fn create_test_server() -> Server {
  let mut server = Server::new(SERVER_URL, POOL_SIZE, None).unwrap();

  server.add_route("/", Rt::GET, demo_handle_home);
  server.add_route("/test", Rt::GET, demo_handle_get);
  server.add_route("/test", Rt::POST, demo_handle_post);
  server.add_route("/test/{id}", Rt::POST, demo_handle_post);
  server.add_route("/test/{id}/{name}", Rt::POST, demo_handle_post);
  server.add_route("/test", Rt::PUT, demo_handle_put);
  server.add_route("/test", Rt::DELETE, demo_handle_delete);
  server.add_files_source("res");

  server
}

fn demo_handle_home(_request: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: "home".as_bytes().to_vec(),
  }
}

#[test]
fn test_home() {
  setup_test_server(|| create_test_server());
  let request = b"GET / HTTP/1.1\r\n\r\n";
  let expected_response = b"home";
  run_test(request, expected_response);
}

fn demo_handle_get(_request: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: "get".as_bytes().to_vec(),
  }
}

#[test]
fn test_get() {
  setup_test_server(|| create_test_server());
  let request = b"GET /test HTTP/1.1\r\n\r\n";
  let expected_response = b"get";
  run_test(request, expected_response);
}

#[test]
fn test_post_with_query_params() {
  setup_test_server(|| create_test_server());
  let request = b"POST /test?name=perrito&id=1234 HTTP/1.1\r\n\r\n";
  let expected_response = b"Some(\"1234\"), Some(\"perrito\")";
  run_test(request, expected_response);
}

fn demo_handle_post(_request: &Request) -> Response {
  let request_string = format!(
    "Method: {}\nUri: {}\nHeaders: {:?}\nBody: {:?}\nParams: {:?}",
    _request.method, _request.path, _request.headers, _request.body, _request.params
  );
  let id = _request.params.get("id").map(|s| s.clone());
  let name = _request.params.get("name").map(|s| s.clone());
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: format!("{:?}, {:?}", id, name).as_bytes().to_vec(),
  }
}

#[test]
fn test_post() {
  setup_test_server(|| create_test_server());
  let request = b"POST /test/1234/perrito HTTP/1.1\r\n\r\n";
  let expected_response = b"Some(\"1234\"), Some(\"perrito\")";
  run_test(request, expected_response);
}

fn demo_handle_put(_request: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: "put".as_bytes().to_vec(),
  }
}

#[test]
fn test_put() {
  setup_test_server(|| create_test_server());
  let request = b"PUT /test HTTP/1.1\r\n\r\n";
  let expected_response = b"put";
  run_test(request, expected_response);
}

fn demo_handle_delete(_request: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: "delete".as_bytes().to_vec(),
  }
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
