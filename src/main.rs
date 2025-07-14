#[cfg(feature = "sync")]
use httpageboy::Server;
use httpageboy::{Request, Response, Rt, StatusCode};

fn main() {
  let serving_url: &str = "127.0.0.1:7878";
  let threads_number: u8 = 10;
  let mut server = Server::new(serving_url, threads_number, None).unwrap();
  server.add_route("/", Rt::GET, demo_get);
  server.add_files_source("res"); //this points to the /res folder in the project root
  server.run();
}

fn demo_get(_request: &Request) -> Response {
  return Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: "<!DOCTYPE html><html><head>
        <meta charset=\"utf-8\">\
        </head><body>🤓👉 <a href=\"/test.png\">IMG</a></body></html>"
      .as_bytes()
      .to_vec(),
  };
}
