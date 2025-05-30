# HTTPageboy Server

Minimal HTTP server package for handling request/response transmission.

`Request`: any HTTP request.
`Route`: path + method + handler → returns Response.
`Response`: bytes with content-type, sent to browser.


## Example

Creating a simple server:

```rust
use httpageboy::{Request, Response, Rt, Server, StatusCode}; // Rt is alias for ResponseType

fn main() {
    let serving_url: &str = "127.0.0.1:7878";
    let threads_number: u8 = 10;
    let mut server = Server::new(serving_url, threads_number, None).unwrap();
    server.add_route("/test", Rt::GET, demo_handle_test_get);
    server.add_route("/test", Rt::POST, demo_handle_test_post);
    server.run();
}

fn demo_handle_test_get(_request: &Request) -> Response {
    return Response {
        status: StatusCode::Ok.to_string(),
        content_type: String::new(),
        content: "test-get".as_bytes().to_vec(),
    };
}

fn demo_handle_test_post(_request: &Request) -> Response {
    return Response {
        status: StatusCode::Ok.to_string(),
        content_type: String::new(),
        content: "test-post".as_bytes().to_vec(),
    };
}
```

## Dependencies

There are no external dependencies for production. :)

There are two deps for testing only.

## License

Copyright (c) 2024 [fahedsl](https://gitlab.com/fahedsl)
This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).
