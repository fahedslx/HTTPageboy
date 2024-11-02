# eqeqo - server_base

Welcome to our HTTP server project! This is a simple server built in Rust that handles HTTP requests and responses.

*Yet another HTTP server implementation. Not trying to reinvent the wheel, but other existing libs add too many unwanted dependencies to my code.*

The eqeqo server base library aims to provide a basic HTTP server implementation to make it easy to create minimal api servers avoiding external dependencies. It can be used to handle HTTP requests and static files.
The server will handle requests according to the setup `route`s, and also static files even if the `route` is not defined.

+ `Request`s can be any HTTP request.
+ `Route`s are formed by the path, the HTTP request type, and the function to handle the request. The function must return a `Response` instance.
+ `Response`s type are identified by the content type and are returned as bytes to browsers.


## Example

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
	// Another demo route handler
	fn demo_handle_test_post (_request: &Request) -> Response {
		return Response {
			status: StatusCode::Ok.to_string(),
			content_type: String::new(),
			content: "test-post".as_bytes().to_vec(),
		}
	}

	fn main() {
		// Location to run, in this example is defined to local.
		let serving_url: &str  = "127.0.0.1:7878";
		// Define number of requests the server will be able to run at the same time.
		let threads_number: u8 = 10;
		// Create a new server
		let server = ServerBase::new(serving_url, threads_number, None).unwrap();
		// Define routes and http method allowed, and link them to request handlers defined previously.
		server.add_route("/test_route", Rt::GET, demo_handle_test_get);
		server.add_route("/test_route", Rt::POST, demo_handle_test_post);
		// Start serving
		server.run();
		// ...
		// Stop and kill the server
		server.stop();
	}
```
