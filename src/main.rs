use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

mod threadpool;
mod status_code;
mod request_handler;
mod request_type;
mod routes;
mod request;
mod response;

use threadpool::ThreadPool;
use request_type::Rt;
use request_handler::Rh;
use routes::get_routes;
use request::{ Request, stream_to_request, handle_request};
use response::Response;


fn send_response(mut stream: TcpStream, response: &Response) {
	let header = format!(
		"HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
		response.status,
		response.content_type,
		response.content.len()
	);
	stream.write(header.as_bytes()).unwrap();
	if response.content_type.starts_with("image/") {
		stream.write(&response.content).unwrap();
	}
	else {
		stream.write(String::from_utf8_lossy(&response.content).as_bytes()).unwrap();
	}
	stream.flush().unwrap();
}


fn serve (listener: TcpListener, pool: ThreadPool, routes: HashMap<String, Vec<(Rt, Rh)>>) {
	for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				println!("Serving.");
				let routes_local = routes.clone();
				pool.execute(move || {
					let request: Request = stream_to_request(&stream);
					println!("{}", &request);
					let answer: Option<Response> = handle_request(&request, &routes_local);
					match answer {
						Some(response) => {
							send_response(stream, &response);
						}
						None => {
							send_response(stream, &Response::new());
						}
					}
				});
			}
			Err(err) => {
				println!("Error: {}", err);
			}
		}
	}
}


// Main
fn main() {
	let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
	let pool = ThreadPool::new(10);
	let routes: HashMap<String, Vec<(Rt, Rh)>> = get_routes();

	serve(listener, pool, routes);
}
