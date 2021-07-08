use std::fs::File;
use std::io::{Error, ErrorKind};

fn main() {
    println!("Hello, Results!");

    let hello_filename = "hello.txt";

    // Will be an Error, but isn't going to panic.
    let f: Result<File, Error> = File::open(hello_filename);

    let f = match File::open(hello_filename) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(hello_filename) {
                Ok(new_file) => {
                    println!("Had to create {}", hello_filename);
                    new_file
                },
                Err(e) => panic!("Failed to create {}:{:?}", hello_filename, e),
            },
            _ => {
                panic!("Problem opening the file {:?}", error)
            }
        },
    };
}
