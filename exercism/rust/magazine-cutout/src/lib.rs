// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut vocab: HashMap<&str, u32> = HashMap::new();
    for w in magazine {
        let entry = vocab.entry(w).or_default();
        *entry += 1;
    }

    for w in note {
        let entry = vocab.entry(w).or_default();
        if *entry <= 0 {
            return false;
        } else {
            *entry -= 1;
        }
    }
    true
}
