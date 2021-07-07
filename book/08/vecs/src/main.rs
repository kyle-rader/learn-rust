fn main() {
    println!("Hello, vecs!");

    // Now that we are pushing i32's below, we don't need this type annotation.
    // let mut evens: Vec<i32> = Vec::new();
    let mut evens: Vec<i32> = Vec::new();
    let mut odds = vec![1, 3, 5];

    evens.push(0);
    evens.push(2);
    evens.push(4);

    odds.push(7);

    // Get and read elements in a Vec
    let five: &i32 = &odds[2];
    println!("&odd[2] gives us {}", five);

    let print_at = |v: &Vec<i32>, i: usize| match v.get(i) {
        Some(val) => println!("v[{}] = {}", i, val),
        None => println!("v[{}] does not exist!", i),
    };

    print_at(&odds, 2);
    print_at(&evens, 5);

    for x in &evens { // must borrow here if we don't want to move ownership
        println!("{}", x);
    }

    evens.push(6);

    println!("Let's mutate odds and push to evens");
    for x in &mut odds {
        *x += 10;
        evens.push(*x-1);
    }

    println!("Now odds has:");
    for i in &odds {
        println!("{}", i);
    }
    println!("Now evens has:");
    for i in &evens {
        println!("{}", i);
    }
}
