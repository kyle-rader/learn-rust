fn format_amount(n: u32) -> String {
    match n {
        0 => String::from("No more"),
        _ => n.to_string(),
    }
}

fn plural(n: u32) -> &'static str {
    match n {
        1 => "bottle",
        _ => "bottles",
    }
}

pub fn verse(n: u32) -> String {
    let next_n = match n {
        0 => 99,
        _ => n - 1,
    };
    let bottles = plural(n);
    let next_bottles = plural(next_n);
    let count = format_amount(n);
    let count_lower = count.to_lowercase();
    let next_count = format_amount(next_n);
    let next_count_lower = next_count.to_lowercase();

    let part1 = format!("{count} {bottles} of beer on the wall, {count_lower} {bottles} of beer.");
    let part2_1 = match n {
        0 => format!("Go to the store and buy some more"),
        1 => format!("Take it down and pass it around"),
        _ => format!("Take one down and pass it around"),
    };
    let part2 = format!("{part2_1}, {next_count_lower} {next_bottles} of beer on the wall.");
    format!("{part1}\n{part2}\n")
}

pub fn sing(start: u32, end: u32) -> String {
    let mut result: Vec<String> = Vec::new();
    let ns = (end..=start).rev();
    for n in ns {
        result.push(verse(n));
    }
    result.join("\n")
}
