use httpageboy::test_utils::{run_test, setup_test_server, POOL_SIZE, SERVER_URL};
use httpageboy::{Request, Response, Rt, Server, StatusCode};

fn create_test_server() -> Server {
  let mut server = Server::new(SERVER_URL, POOL_SIZE, None).unwrap();

  server.add_route("/", Rt::GET, demo_handle_home);
  server.add_route("/test", Rt::GET, demo_handle_get);
  server.add_route("/test", Rt::POST, demo_handle_post);
  server.add_route("/test/{param1}", Rt::POST, demo_handle_post);
  server.add_route("/test/{param1}/{param2}", Rt::POST, demo_handle_post);
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
fn test_get_with_query() {
  setup_test_server(|| create_test_server());
  let request = b"GET /test?foo=bar&baz=qux HTTP/1.1\r\n\r\n";
  let expected_response = b"get"; // mismo handler
  run_test(request, expected_response);
}

fn demo_handle_post(_request: &Request) -> Response {
  let request_string = format!(
    "Method: {}\nUri: {}\nParams: {:?}\nBody: {:?}",
    _request.method, _request.path, _request.params, _request.body
  );
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: request_string.as_bytes().to_vec(),
  }
}

#[test]
fn test_post() {
  setup_test_server(|| create_test_server());
  let request = b"POST /test HTTP/1.1\r\n\r\nmueve tu cuerpo";
  let expected_response = b"Method: POST\nUri: /test\nParams: {}\nBody: \"mueve tu cuerpo\"";
  run_test(request, expected_response);
}

#[test]
fn test_post_with_query() {
  setup_test_server(|| create_test_server());
  let request = b"POST /test?foo=bar HTTP/1.1\r\n\r\nmueve tu cuerpo";
  let expected_response =
    b"Method: POST\nUri: /test\nParams: {\"foo\": \"bar\"}\nBody: \"mueve tu cuerpo\"";
  run_test(request, expected_response);
}

#[test]
fn test_post_with_content_length() {
  setup_test_server(|| create_test_server());
  let request = b"POST /test HTTP/1.1\r\nContent-Length: 15\r\n\r\nmueve tu cuerpo";
  let expected_response = b"Method: POST\nUri: /test\nParams: {}\nBody: \"mueve tu cuerpo\"";
  run_test(request, expected_response);
}

#[test]
fn test_post_with_params() {
  setup_test_server(|| create_test_server());
  let request = b"POST /test/hola/que?param4=hoy&param3=hace HTTP/1.1\r\n\r\nmueve tu cuerpo";
  let expected_response =
    b"Method: POST\n\
      Uri: /test/hola/que\n\
      Params: {\"param1\": \"hola\", \"param2\": \"que\", \"param3\": \"hace\", \"param4\": \"hoy\"}\n\
      Body: \"mueve tu cuerpo\"";
  run_test(request, expected_response);
}

#[test]
fn test_post_with_incomplete_path_params() {
  setup_test_server(|| create_test_server());
  let request = b"POST /test/hola HTTP/1.1\r\n\r\nmueve tu cuerpo";
  let expected_response =
    b"Method: POST\nUri: /test/hola\nParams: {\"param1\": \"hola\"}\nBody: \"mueve tu cuerpo\"";
  run_test(request, expected_response);
}

fn demo_handle_put(_request: &Request) -> Response {
  let request_string = format!(
    "Method: {}\nUri: {}\nParams: {:?}\nBody: {:?}",
    _request.method, _request.path, _request.params, _request.body
  );
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: request_string.as_bytes().to_vec(),
  }
}

#[test]
fn test_put() {
  setup_test_server(|| create_test_server());
  let request = b"PUT /test HTTP/1.1\r\n\r\nmueve tu cuerpo";
  let expected_response = b"Method: PUT\nUri: /test\nParams: {}\nBody: \"mueve tu cuerpo\"";
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

#[test]
fn test_method_not_allowed() {
  setup_test_server(|| create_test_server());
  let request = b"BREW /coffee HTTP/1.1\r\n\r\n";
  let expected_response = b"HTTP/1.1 405 Method Not Allowed";
  run_test(request, expected_response);
}

#[test]
fn test_empty_request() {
  setup_test_server(|| create_test_server());
  let request = b"";
  let expected_response = b"HTTP/1.1 400 Bad Request";
  run_test(request, expected_response);
}

#[test]
fn test_malformed_request() {
  setup_test_server(|| create_test_server());
  let request = b"THIS_IS_NOT_HTTP\r\n\r\n";
  let expected_response = b"HTTP/1.1 400 Bad Request";
  run_test(request, expected_response);
}

#[test]
fn test_unsupported_http_version() {
  setup_test_server(|| create_test_server());
  let request = b"GET / HTTP/0.9\r\n\r\n";
  let expected_response = b"HTTP/1.1 505 HTTP Version Not Supported";
  run_test(request, expected_response);
}

#[test]
fn test_long_path() {
  setup_test_server(|| create_test_server());
  let long_path = "/".to_string() + &"a".repeat(10_000);
  let request = format!("GET {} HTTP/1.1\r\n\r\n", long_path);
  let expected_response = b"HTTP/1.1 414 URI Too Long";
  run_test(request.as_bytes(), expected_response);
}

#[test]
fn test_missing_method() {
  setup_test_server(|| create_test_server());
  let request = b"/ HTTP/1.1\r\n\r\n";
  let expected_response = b"HTTP/1.1 400 Bad Request";
  run_test(request, expected_response);
}
