# eqeqo - server_base

## Overview

*Still under development. If you are interested, feel free to report issues or even branch the repository.*

The server base library provides a basic HTTP server implementation. It can be used to host static files and handle HTTP requests.
The server will handle requests according to the setup `route`s, and also static files even if the `route` is not defined.

+ `Request`s can be any HTTP request.
+ `Route`s are formed by the path, the HTTP request type, and the function to handle the request. The function must return a `Response` instance.
+ `Response`s type are identified by the content type and are returned as bytes to browsers.


## Examples

Creating a simple server:

```rust
	use server_base::{ ServerBase, Rt, Request, Response, StatusCode };

	// Demo route handler
	fn demo_handle_test_get (_request: &Request) -> Response {
		return Response {
			status: StatusCode::Ok.to_string(),
			content_type: String::new(),
			content: "test-get".as_bytes().to_vec(),
		}
	}

	fn main() {
		let serving_url: &str  = "127.0.0.1:7878";
		let pool_size: u8 = 10;
		let server = ServerBase::new(serving_url, pool_size, None).unwrap();
	
		server.add_route("/test", Rt::GET, demo_handle_test_get);
	
		server.serve();
		// Runs on http://127.0.0.1:7878
		// It can serve correctly the defined route.
		// It can also serve static files even if the route is not defined.
	}
```

## License

Copyright (c) 2024 [fahedsl](https://gitlab.com/fahedsl)

MIT License
