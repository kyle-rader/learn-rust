fn main() {
    let fav_color: Option<&str> = None;
    let is_tues = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = fav_color {
        println!("fav color is in fact {}", color);
    }
    else if is_tues {
        println!("It's Tuesday!");
    }
    else if let Ok(age) = age {
        if age > 30 {
            println!("over 30!");
        } else {
            println!("30 or Under!");
        }
    } else {
        println!("Final if!");
    }

    let mut stack = Vec::new();
    for i in 0..5 {
        stack.push(i);
    }

    while let Some(top) = stack.pop() {
        println!("Got {} off the stack!", top);
    }

    let v = vec!['a', 'b', 'c', 'd'];
    for (i, val) in v.iter().enumerate() {
        println!("{} is at index {}", val, i);
    }

    let (x, y, z) = (1,9,100);
    println!("{}, {}, {}", x, y, z);

    // Pattern deconstruct does not work, since types don't match
    // let (x, y) = (3, 4, 5);
    
    foo(3);

    let p = (3, 5);
    coords(&p);
}

fn foo(x: i32) {
    println!("Got {}", x);
}

fn coords(&(x, y): &(i32, i32)) {
    println!("At {}, {}", x, y);
}
