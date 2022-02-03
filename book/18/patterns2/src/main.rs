fn main() {
    println!("Hello, patterns!");

    let x = 5;
    match x {
        1 => println!("one"),
        5 => println!("five"),
        _ => println!("other"),
    }

    let y = Some(5);
    match y {
        Some(10) => println!("Got 10"),
        Some(y) => println!("Got {}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    let p = Point { x: 5, y: 8 };
    let Point { x, y } = p;
    println!("x: {}, y: {}", x, y);

    match p {
        Point { x, y: 0 } => println!("on x at {}, y=0", x),
        Point { x: 0, y } => println!("on y at {}, x=0", y),
        Point { x, y } => println!("On x,y ({}, {})", x, y),
    }

    let msgs = vec![
        Message::ChangeColor(0, 160, 255),
        Message::Write(String::from("3 points!")),
        Message::ChangeColor(23, 56, 89),
        Message::Move { x: 5, y: 8 },
        Message::Quit,
    ];

    println!("Matching enums!");
    for msg in msgs.iter() {
        match msg {
            Message::Quit => println!("Quit!"),
            Message::Move { x, y } => {
                println!("Move x {} and y {}", x, y);
            }
            Message::Write(s) => println!("Writing {:}", s),
            Message::ChangeColor(r, g, b) => {
                println!("({}, {}, {})", r, g, b);
            }
        }
    }

    println!("Match guards in patterns!");
    for msg in msgs.iter() {
        match msg {
            Message::Move { x, y } if x * y > 5 => {
                println!("Move x: {}, y: {}", x, y);
            }
            _ => println!("Not a move."),
        }
    }

    match wisdom() {
        0 => println!("I haven't celebrated my first birthday yet"),
        // Could `match` 1 ..= 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 ..= 12. Now the age can be reported.
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound. Return the result.
        n => println!("I'm an old person of age {:?}", n),
    }
}

fn wisdom() -> u32 {
    42
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct Point {
    x: i32,
    y: i32,
}
