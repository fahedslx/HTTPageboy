use httpageboy::{Request, Response, Rt, Server, StatusCode}; // Rt is alias for ResponseType

fn main() {
  let serving_url: &str = "127.0.0.1:7878";
  let threads_number: u8 = 10;
  let mut server = Server::new(serving_url, threads_number, None).unwrap();
  server.add_route("/", Rt::GET, demo_get);
  server.add_route("/", Rt::POST, demo_post);
  server.run();
}

fn demo_get(_request: &Request) -> Response {
  return Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: "get".as_bytes().to_vec(),
  };
}

fn demo_post(_request: &Request) -> Response {
  return Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: "post".as_bytes().to_vec(),
  };
}
