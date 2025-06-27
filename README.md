# HTTPageboy Server

Minimal HTTP server package for handling request/response transmission.

`Request`: any HTTP request.
`Route`: path + method + handler → returns Response.
`Response`: bytes with content-type, sent to browser.


## Example


`lib.rs` is the actual implementation of the server.


`main.rs` is this following example ready for execution. Just run `cargo run` on the terminal and go to http://127.0.0.1:7878.


Creating a simple server:

```rust
use httpageboy::{Request, Response, Rt, Server, StatusCode}; // Rt is alias for ResponseType

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
```

## Dependencies

There are no external dependencies for production. :)

There are two deps used on the tests only. Just remove them.

## License

Copyright (c) 2025 [fahedsl](https://gitlab.com/fahedsl).
This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).
