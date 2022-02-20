use core::fmt;
use std::{io::*, thread};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, process};

use final_chapter::ThreadPool;

fn main() {
    println!("Hello, servers!");
    let mut pool = match ThreadPool::new(4) {
        Ok(pool) => pool,
        Err(msg) => {
            eprintln!("{msg}");
            process::exit(1);
        }
    };

    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(bound_port) => bound_port,
        Err(msg) => {
            eprintln!("Binding to port 7878 failed!\n{}", msg);
            process::exit(1);
        }
    };

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

const GET_REQUEST: &[u8] = b"GET / HTTP/1.1\r\n";
const GET_CSS: &[u8] = b"GET /index.css HTTP/1.1\r\n";
const GET_SLEEP: &[u8] = b"GET /sleep HTTP/1.1\r\n";

enum Status {
    Ok,
    NotFound,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Ok => write!(f, "200 OK"),
            Status::NotFound => write!(f, "404 NOT FOUND"),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let req = String::from_utf8_lossy(&buffer[..]);
    let parts: Vec<&str> = req.split("\r\n").collect();


    println!("Request: {}", parts[0]);

    let (status, filename) = if buffer.starts_with(GET_REQUEST) {
        (Status::Ok, "hello.html")
    } else if buffer.starts_with(GET_CSS) {
        (Status::Ok, "index.css")
    } else if buffer.starts_with(GET_SLEEP) {
        thread::sleep(Duration::from_secs(5));
        (Status::Ok, "hello.html")
    } else {
        (Status::NotFound, "404.html")
    };

    let content = fs::read_to_string(filename).unwrap();

    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Length: {}\r\n\r\n{content}",
        content.len()
    );

    println!("Repsonse: {status}\n");
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
