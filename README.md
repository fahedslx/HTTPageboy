**EQEQO - SERVER_BASE**

## Overview
The server base library provides a basic server implementation. The server can be used to host static files and handle HTTP requests.
The server will be able to handle a request and return the response to the client according to the routes provided by the developer.


+ Requests can be any HTTP request.
+ Routes are formed by the path, the HTTP request type, and the function to handle the request. The function must return a Response instance.
+ Responses type are identified by the content type and are returned as bytes for browsers.

## Examples

Creating a simple server:

```rust
	// Server setup
	let serving_url: &str  = "127.0.0.1:7878";
	let pool_size: u8 = 10;
	let server = ServerBase::new(serving_url, pool_size, None).unwrap();

	// Routes setup
	server.add_route("/test", Rt::GET, demo_handle_test_get);

	// Server start, running on http://127.0.0.1:7878
	// It can already serve correctly the routes and static files within the server.
	server.serve();

	pub fn demo_handle_home_get (_request: &Request) -> Response {
		return Response {
			status: StatusCode::Ok.to_string(),
			content_type: String::new(),
			content: "home-get".as_bytes().to_vec(),
		}
	}
```

## License

Copyright (c) 2024 [fahedsl](https://gitlab.com/fahedsl)

MIT License
