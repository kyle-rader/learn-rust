use std::net::{TcpListener, TcpStream};
use std::process;
use std::io::*;

fn main() {
    println!("Hello, servers!");

    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(bound_port) => bound_port,
        Err(msg) => {
            eprintln!("Binding to port 7878 failed!\n{}", msg);
            process::exit(1);
        }
    };

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    println!("Repsonse: {}", response);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
