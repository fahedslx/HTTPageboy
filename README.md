# HTTPageboy

Minimal HTTP server package for handling request/response transmission.
Focuses only on transporting a well formed HTTP message; does not process or decide how the server behaves.
Aspires to become runtime-agnostic, with minimal, solid, and flexible dependencies.

## Example

The core logic resides in `src/lib.rs`.

### See it working out of the box on [this video](https://www.youtube.com/watch?v=VwRYWJ33C4o)

The following example is executable. Run `cargo run` to see the available variants and navigate to [http://127.0.0.1:7878](http://127.0.0.1:7878) in your browser.

A basic async server setup example:
Execute using `cargo run --features async_tokio`

```rust
#![cfg(feature = "async_tokio")]
use httpageboy::{Handler, Request, Response, Rt, Server, StatusCode};
use tokio::io::AsyncWriteExt;

/// GET /hello?name=Foo -> HTML with "Foo"
async fn hello(req: &Request) -> Response {
  let name = query_param(&req.path, "name").unwrap_or_else(|| "friend".into());
  let html = format!("<!DOCTYPE html><meta charset=utf-8><body>Hello, {}! 🤓</body>", name);
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/html; charset=utf-8".into(),
    content: html.into_bytes(),
  }
}

/// Tiny query string parser (no URL-decoding for brevity)
fn query_param(path: &str, key: &str) -> Option<String> {
  let q = path.splitn(2, '?').nth(1)?;
  for pair in q.split('&') {
    let mut it = pair.splitn(2, '=');
    if let (Some(k), Some(v)) = (it.next(), it.next()) {
      if k == key {
        return Some(v.to_string());
      }
    }
  }
  None
}

#[tokio::main]
async fn main() {
  let mut srv = Server::new("127.0.0.1:7878", None).await.unwrap();
  srv.add_route("/hello", Rt::GET, handler!(hello));
  srv.add_files_source("res"); // optional: static files from ./res
  srv.run().await;
}

```

## Testing

For synchronous tests:
```bash
cargo test --features sync --test test_sync
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
