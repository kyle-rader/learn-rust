use std::io::*;
use std::net::{TcpListener, TcpStream};
use std::{fs, process};

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

const GET_REQUEST: &[u8] = b"GET / HTTP/1.1\r\n";

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let content:String;

    if buffer.starts_with(GET_REQUEST) {
        content = fs::read_to_string("hello.html").unwrap();
    }
    else {
        content = fs::read_to_string("404.html").unwrap();
    }

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    );

    println!("Repsonse:--------\n{}\n--------", response);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
