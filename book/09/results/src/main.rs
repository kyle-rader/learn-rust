use std::fs::{self, File};
use std::io::{Error, ErrorKind, Read};

fn read_username_from_file1() -> Result<String, Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::with_capacity(24);
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, Error> {
    fs::read_to_string("hello.txt")
}

fn main() {
    println!("Hello, Results!");

    let hello_filename = "hello.txt";

    // Will be an Error, but isn't going to panic.
    let f: Result<File, Error> = File::open(hello_filename);

    {
        let f = match File::open(hello_filename) {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create(hello_filename) {
                    Ok(new_file) => {
                        println!("Had to create {}", hello_filename);
                        new_file
                    }
                    Err(e) => panic!("Failed to create {}:{:?}", hello_filename, e),
                },
                _ => {
                    panic!("Problem opening the file {:?}", error)
                }
            },
        };
    }

    {
        let f = match File::open(hello_filename) {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => File::create(hello_filename).expect("Failed to create file"),
                _ => panic!("Problem opening the file {:?}", error),
            },
        };
    }

    {
        let username = read_username_from_file3();
        if let Ok(username) = username {
            if username.is_empty() {
                println!("username is empty!");
            } else {
                println!("Got username: '{}'", username);
            }
        } else if let Err(e) = username {
            println!("Couldn't get username ({})", e);
        }
    }
}
