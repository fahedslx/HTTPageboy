use std::fs;
use std::io::prelude::*;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
mod utils {
	pub mod threadpool;
}
use utils::threadpool::ThreadPool;


fn handle_request(mut stream: TcpStream) -> (String, Vec<u8>, String) {
	let mut buffer = [0; 1024];
	stream.read(&mut buffer).unwrap();
	let request = String::from_utf8_lossy(&buffer[..]);
	let status: &str;
	let mut content = Vec::new();
	let mut content_type = String::new();

	if request.starts_with("GET / HTTP/1.1") {
		status = "200 OK";
		fs::File::open("res/html/index.html")
			.and_then(|mut file| file.read_to_end(&mut content))
			.unwrap_or_else(|_| {
				println!("Error al leer el archivo index.html");
				0
			});
		content_type = "text/html".to_string();
	}
	else if request.starts_with("GET /sleep HTTP/1.1") {
		status = "200 OK";
		fs::File::open("res/html/index.html")
			.and_then(|mut file| file.read_to_end(&mut content))
			.unwrap_or_else(|_| {
				println!("Error al leer el archivo index.html");
				0
			});
		content_type = "text/html".to_string();
	}
	else if request.contains(".css") {
		status = "200 OK";
		let requested_path: Vec<&str> = request.split_whitespace().collect();
		let path = format!(".{}", requested_path[1]);
		fs::File::open(path)
			.and_then(|mut file| file.read_to_end(&mut content))
			.unwrap_or_else(|_| {
				println!("Error al leer el archivo CSS");
				0
			});
		content_type = "text/css".to_string();
	}
	else if request.contains(".png HTTP/1.1")
		|| request.contains(".jpg HTTP/1.1")
		|| request.contains(".svg HTTP/1.1")
		|| request.contains(".webp HTTP/1.1")
	{
		status = "200 OK";
		let requested_path: Vec<&str> = request.split_whitespace().collect();
		let path = format!(".{}", requested_path[1]);
		fs::File::open(path)
			.and_then(|mut file| file.read_to_end(&mut content))
			.unwrap_or_else(|_| {
				println!("Error al leer el archivo de imagen");
				0
			});
		if request.contains(".png HTTP/1.1") {
			content_type = "image/png".to_string();
		} else if request.contains(".jpg HTTP/1.1") {
			content_type = "image/jpeg".to_string();
		} else if request.contains(".svg HTTP/1.1") {
			content_type = "image/svg+xml".to_string();
		} else if request.contains(".webp HTTP/1.1") {
			content_type = "image/webp".to_string();
		}
	}
	else {
		status = "404 Not Found";
		fs::File::open("res/html/404.html")
			.and_then(|mut file| file.read_to_end(&mut content))
			.unwrap_or_else(|_| {
				println!("Error al leer el archivo 404.html");
				0
			});
		content_type = "text/html".to_string();
	}

	(status.to_string(), content, content_type)
}


fn send_response(mut stream: TcpStream, status: String, content: Vec<u8>, content_type: String) {
	let response = format!(
		"HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
		status,
		content_type,
		content.len(),
		String::from_utf8_lossy(&content)
	);
	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
}


fn main() {
	let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

	let pool = ThreadPool::new(10);

	for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				pool.execute(|| {
					let (status, content, content_type) = handle_request(stream.try_clone().unwrap());
					send_response(stream, status, content, content_type);
				});
			}
			Err(err) => {
				println!("Error: {}", err);
			}
		}
	}
}
