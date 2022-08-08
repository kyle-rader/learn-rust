use std::collections::{HashMap, HashSet};

type Key = HashMap<char, u8>;

pub fn solve(input: &str) -> Option<Key> {
    let key = HashMap::from([('I', 1), ('B', 9), ('L', 0)]);

    if test_key(&key, input) {
        Some(key)
    } else {
        None
    }
}

pub fn chars_we_care_about(input: &str) -> Vec<char> {
    let skip_chars: HashSet<char> = HashSet::from([' ', '+', '=']);
    input
        .chars()
        .filter(|c| !skip_chars.contains(&c))
        .collect::<HashSet<char>>()
        .iter()
        .cloned()
        .collect()
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
