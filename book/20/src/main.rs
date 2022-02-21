use core::fmt;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, process};
use std::{io::*, thread};

use final_chapter::ThreadPool;

fn main() {
    println!("🚀 Starting Server");
    let mut pool = match ThreadPool::new(4) {
        Ok(pool) => pool,
        Err(msg) => {
            eprintln!("{msg}");
            process::exit(1);
        }
    };

    let host = "127.0.0.1:7878";
    println!("🔗 Binding to '{host}'");
    let listener = match TcpListener::bind(host) {
        Ok(bound_port) => bound_port,
        Err(msg) => {
            eprintln!("Binding to port 7878 failed!\n{}", msg);
            process::exit(1);
        }
    };

    println!("📭 Listening for requests.");
    for stream in listener.incoming().take(4) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting Down");
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
    if let Some(route) = req.split("\r\n").nth(0) {
        println!("Request: {route}");
    }

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

    let content_type = match filename.split('.').last() {
        Some("html") => "text/html",
        Some("css") => "text/css",
        Some(suffix) => {
            eprintln!("Warning: file suffix '{suffix}' has no matching content-type. Defaulting to text/plain.");
            "text/plain"
        }
        None => {
            eprintln!("Warning: no file suffix. Defaulting to text/plain.");
            "text/plain"
        }
    };

    let content = fs::read_to_string(filename).unwrap();

    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Length: {}\r\nContent-Type: {content_type}\r\n\r\n{content}",
        content.len()
    );

    println!("Repsonse: {status}\n");
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
