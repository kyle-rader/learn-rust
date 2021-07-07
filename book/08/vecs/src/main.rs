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

    let print_vec = |v: &Vec<i32>| {
        let as_str: String = v.iter()
            .map(|&i| i.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        println!("{}", as_str);
    };

    print_at(&odds, 2);
    print_at(&evens, 5);

    // for x in &evens { // must borrow here if we don't want to move ownership
    //     println!("{}", x);
    // }
    print_vec(&evens);

    evens.push(6);

    println!("Let's mutate odds and push to evens");
    for x in &mut odds {
        *x += 10;
        evens.push(*x-1);
    }

    println!("Now odds has:");
    print_vec(&odds);
    println!("Now evens has:");
    print_vec(&evens);

    // Holding different types of items
    #[derive(Debug)]
    enum Cell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        Cell::Int(4),
        Cell::Text(String::from("blue")),
        Cell::Float(3.14159),
    ];

    println!("row has:");
    for i in &row {
        println!("{:?}", i);
    }
}
