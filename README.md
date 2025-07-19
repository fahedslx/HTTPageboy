# HTTPageboy

Minimal HTTP server package for handling request/response transmission.
Focuses only on transporting a well formed HTTP message; does not process or decide how the server behaves.
Aspires to become runtime-agnostic, with minimal, solid, and flexible dependencies.

## Example

The core logic resides in `src/lib.rs`.

The following example is executable. Run `cargo run` to see the available variants and navigate to [http://127.0.0.1:7878](http://127.0.0.1:7878) in your browser.

A basic server setup:

```rust
use httpageboy::{Request, Response, Rt, Server, StatusCode}; // Rt is alias for ResponseType

fn main() {
  let serving_url: &str = "127.0.0.1:7878";
  let threads_number: u8 = 10;
  let mut server = Server::new(serving_url, threads_number, None).unwrap();
  server.add_route("/", Rt::GET, demo_get);
  server.add_files_source("res"); // this points to the /res folder in the project root
  server.run();
}

fn demo_get(_request: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: "<!DOCTYPE html><html><head>\
<meta charset=\"utf-8\">\
</head><body>🤓: Hi, this is Pageboy working.
<br>Do you like the <a href=\"/HTTPageboy.svg\">new icon</a>?</body></html>"
      .as_bytes()
      .to_vec(),
  }
}
```

## Testing

For synchronous tests:
```bash
cargo test --features sync
```

For asynchronous tests:
```bash
cargo test --features async_tokio --test test_async_tokio
cargo test --features async_std --test test_async_std
// or
cargo test --features async_smol --test test_async_smol
```

## Examples

Additional examples can be found within the tests.

## License

Copyright (c) 2025 [fahedsl](https://gitlab.com/fahedsl).
This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).
