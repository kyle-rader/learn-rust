use std::collections::{HashMap, HashSet};

type CharMap = HashMap<char, u32>;

fn char_map(word: &str) -> CharMap {
    let mut map: HashMap<char, u32> = HashMap::new();
    for c in word.chars() {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }
    map
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&'a str> = HashSet::new();
    let word_lower = word.to_lowercase();
    let key = char_map(word_lower.as_str());

    for w in possible_anagrams {
        if w.len() != word.len() {
            continue;
        }

        let w_lower = w.to_lowercase();
        if w_lower == word_lower {
            continue;
        }

        let w_map = char_map(w_lower.as_str());
        if w_map == key {
            result.insert(*w);
        }
    }
    result
}
