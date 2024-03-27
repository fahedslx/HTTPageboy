// En /tests/test1.rs

#[cfg(test)]
mod tests {
	use std::process::Command;
	use std::thread;
	use std::time::Duration;
	use std::net::TcpStream;
	use std::io::{Read, Write};

	#[test]
	fn test_server() {
		// Inicia tu servidor en un hilo separado
		let server = thread::spawn(|| {
			let output = Command::new("cargo")
				.arg("run")
				.arg("--bin")
				.arg("serve")
				.output()
				.expect("Error al ejecutar el binario serve");

			assert!(output.status.success());

			let stdout = String::from_utf8_lossy(&output.stdout);
			let stderr = String::from_utf8_lossy(&output.stderr);
			assert!(stdout.contains("expected output"));
			assert!(stderr.is_empty());
		});

		// Espera un momento para que el servidor se inicie
		thread::sleep(Duration::from_secs(1));

		// Crea una conexión al servidor
		let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
		println!("paso 1");
		// Escribe una solicitud HTTP en el stream
		stream.write_all(b"GET / HTTP/1.1\r\n\r\n").unwrap();

		// Lee la respuesta del servidor
		let mut buffer = Vec::new();
		stream.read_to_end(&mut buffer).unwrap();

		// Verifica que la respuesta es correcta
		assert!(buffer.starts_with(b"HTTP/1.1 200 OK\r\n"));

		// Asegúrate de unirte al hilo del servidor para que no se cierre prematuramente
		server.join().unwrap();
	}
}
