use std::collections::HashMap;

type Key = HashMap<char, u8>;

pub fn solve(input: &str) -> Option<Key> {
    let solution: &[(char, u8)] = &[('I', 1u8), ('B', 9u8), ('L', 0u8)];
    let key: Key = solution.iter().cloned().collect();

    if test_key(&key, input) {
        Some(key)
    } else {
        None
    }
}

fn test_key(key: &Key, input: &str) -> bool {
    // First, translate the input according to the given key
    let translated: String = input
        .chars()
        .map(|c| key.get(&c).map(|num| *num as char).unwrap_or(c))
        .collect();

    // Now parse and eval the expression.
    let translated: Vec<&str> = translated.split("==").collect();
    match translated.len() {
        2 => eval(translated[0]) == eval(translated[1]),
        _ => panic!("We need both a left and right expression to eval!"),
    }
}

fn eval(input: &str) -> usize {
    input
        .split(' ')
        .fold(0, |acc, token| acc + token.parse().unwrap_or(0))
}
