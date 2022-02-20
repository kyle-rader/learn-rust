use core::fmt;
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
const GET_CSS: &[u8] = b"GET /index.css HTTP/1.1\r\n";

enum Status {
    Ok,
    NotFound,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Ok => write!(f, "200 OK"),
            Status::NotFound =>write!(f, "404 NOT FOUND"),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let status: Status;
    let content: String;

    if buffer.starts_with(GET_REQUEST) {
        status = Status::Ok;
        content = fs::read_to_string("hello.html").unwrap();
    }
    else if buffer.starts_with(GET_CSS) {
        status = Status::Ok;
        content = fs::read_to_string("index.css").unwrap();
    } else {
        status = Status::NotFound;
        content = fs::read_to_string("404.html").unwrap();
    }

    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Length: {}\r\n\r\n{content}",
        content.len()
    );

    println!("Repsonse:--------\n{}\n--------", response);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
