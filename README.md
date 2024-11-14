# eqeqo - HTTPageboy

A basic package for handling HTTP request and response transmission on the server side, without additional processing.

+ `Request`s can be any HTTP request.
+ `Route`s are formed by the path, the HTTP request type, and the function to handle the request. The function must return a `Response` instance.
+ `Response`s type are identified by the content type and are returned as bytes to browsers.


## Example

Creating a simple server:

```rust
	use server_base::{ ServerBase, Rt, Request, Response, StatusCode };

	// Demo route handler (valid empty answer)
	fn demo_handle_test_get (_request: &Request) -> Response {
		return Response {
			status: StatusCode::Ok.to_string(),
			content_type: String::new(),
			content: "test-get".as_bytes().to_vec(),
		}
	}
	// Another demo route handler (also empty)
	fn demo_handle_test_post (_request: &Request) -> Response {
		return Response {
			status: StatusCode::Ok.to_string(),
			content_type: String::new(),
			content: "test-post".as_bytes().to_vec(),
		}
	}

	fn main() {
		// Location to run, in this example is defined to an arbitrary ip and port.
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

## Dependencies

There are no external dependencies for production. :)
There are two deps for testing only.

## License

Copyright (c) 2024 [fahedsl](https://gitlab.com/fahedsl)
This project is licensed under the MIT License. For more details, refer to the [MIT License](https://opensource.org/licenses/MIT).
