struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Cord(i32, i32);

fn new_use(email: &str, username: &str) -> User {
    User {
        email: String::from(email),
        username: String::from(username),
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    println!("Hello structs");

    let user1 = User {
        email: String::from("joe@foobar.com"),
        username: String::from("job_foobar"),
        active: true,
        sign_in_count: 2,
    };

    println!("We have user1 {}", user1.username);

    let user2 = new_use("jane@doe.com", "jane_doe");
    println!("And user2 {}", user2.username);

    // struct update syntax
    let user3 = User {
        email: String::from("bob@gmail.com"),
        username: String::from("bobo"),
        ..user1
    };

    let a = Cord(1, 4);
    let center = Cord(0,0);
}
