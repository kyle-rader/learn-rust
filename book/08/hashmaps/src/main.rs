use std::collections::HashMap;

fn main() {
    println!("Hello, HashMap!");

    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 6);

    // We could zip data together
    let teams = vec![String::from("blue"), String::from("red")];
    let init_scores = vec![10, 7];
    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(init_scores.into_iter()).collect();

    scores.insert(String::from("yellow"), 25);

    println!("Scoreboard:");
    for (team, score) in &scores {
        println!("{} has {} points", team, score);
    }

    // Ownership with HaspMaps
    let field_name = String::from("favorite_color");
    let field_value = String::from("blue");

    let mut map = HashMap::new();
    map.insert(field_name.clone(), field_value);
    // println!("{} = {}", field_name, field_value); // these were moved into the map and are not longer valid.
    if let Some(val) = &map.get(&field_name) {
        println!("{} = {}", field_name, val);
    }

    // Overwrite a value
    scores.insert("red".to_string(), 12);
    println!("{:?}", scores);

    {
        // Create new entry if not there already with Entry
        let green_score: &mut i32 = scores.entry(String::from("green")).or_insert(20);
        println!("green has {} points", green_score);
        *green_score += 8;
        println!("now green has {} points!", green_score);
    }

    scores.insert("green".to_string(), 34);
    if let Some(score) = &scores.get("green") {
        println!("now green has {} points!!", score);
    }

    println!("Fin.");

    // Word counting
    let text = "hello delightful world Isn't the world delightful and full of delight";
    let mut word_counts = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_counts.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", word_counts);
    println!("We can still use text ({})", text);
}
