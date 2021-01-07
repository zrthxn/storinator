use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn tcp() {
	let listener = TcpListener::bind("localhost:8080").unwrap();

	for stream in listener.incoming() {
		let stream = stream.unwrap();
		handler(stream)
	}
}

fn handler(mut stream: TcpStream) {
	let mut buffer = [0; 1024];

	stream.read(&mut buffer).unwrap();
	println!("{}", String::from_utf8_lossy(&buffer));

	let body = "BODY";
	let response = format!(
		"HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}\r\n",
		body.len(),
		body
	);

	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
}