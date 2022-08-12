use std::{collections::{HashMap, HashSet}, num::ParseIntError};

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum MyParseError {
    #[error("not a digit: {0}")]
    ParseInt(#[from] ParseIntError),

    #[error("cannot have leading zeros")]
    LeadingZero,
}

type Key = HashMap<char, u8>;

pub fn solve(input: &str) -> Option<Key> {
    let mut letters_to_use: Vec<char> = chars_we_care_about(input).into_iter().collect();
    let mut numbers_in_use: HashSet<u8> = HashSet::new();
    let mut solution: Key = Key::new();
    test_all_keys(input, &mut letters_to_use, &mut numbers_in_use, &mut solution)
}

fn test_all_keys(input: &str, letters_to_use: &mut Vec<char>, numbers_in_use: &mut HashSet<u8>, solution: &mut Key) -> Option<Key> {
    if let Some(letter) = letters_to_use.pop() {
        // we a letter to iterate on
        for i in 0..=9 {
            if numbers_in_use.insert(i) {
                solution.insert(letter, i);
                match test_all_keys(input, letters_to_use, numbers_in_use, solution) {
                    Some(solution) => return Some(solution),
                    None => {
                        numbers_in_use.remove(&i);
                    }
                }
            }
        }
        letters_to_use.push(letter);
        None
    }
    else {
        // base case, the solution is full and we can test it.
        match test_key(input, solution) {
            Ok(test_result) => {
                if test_result {
                    Some(solution.clone())
                }
                else {
                    None
                }
            }
            Err(_) => None
        }
    }
}

pub fn test_key(input: &str, key: &Key) -> Result<bool, MyParseError> {
    // First, translate the input according to the given key
    let translated: String = input
        .chars()
        .map(|c| key.get(&c).map(|num| (num + b'0') as char).unwrap_or(c))
        .collect();

    // Now parse and eval the expression.
    let translated: Vec<&str> = translated.split("==").collect();
    match translated.len() {
        2 => Ok(eval(translated[0])? == eval(translated[1])?),
        _ => panic!("We need both a left and right expression to eval!"),
    }
}

pub fn eval(input: &str) -> Result<usize, MyParseError> {
    let tokens = input.split_whitespace().filter(|s| *s != "+");
    let mut result: usize = 0;
    for token in tokens {
        if token.starts_with('0') {
            return Err(MyParseError::LeadingZero);
        }
        result += token.parse::<usize>()?;
    }
    Ok(result)
}

pub fn chars_we_care_about(input: &str) -> HashSet<char> {
    let skip_chars: HashSet<char> = HashSet::from([' ', '+', '=']);
    input
        .chars()
        .filter(|c| !skip_chars.contains(c))
        .collect()
}
