/* Examples
"Portable Network Graphics" -> "PNG"
"Ruby on Rails" -> "ROR"
"HyperText Markup Language" -> "HTML"
"First In, First Out" -> "FIFO"
"GNU Image Manipulation Program"),
"PHP: Hypertext Preprocessor" -> "PHP"
"Complementary metal-oxide semiconductor" -> "CMOS"
"Something - I made up from thin air" ->
"Halley's Comet" -> "HC"
"The Road _Not_ Taken" -> "TRNT"
 */

use std::collections::HashSet;

pub fn abbreviate(phrase: &str) -> String {
    let separators = HashSet::from([' ', '-', '_']);
    let mut letters: Vec<char> = Vec::new();
    let mut last: Option<char> = None;

    for c in phrase.chars() {
        let should_add = match last {
            None => true,
            Some(l) => {
                (!separators.contains(&c) && separators.contains(&l))
                    || (l.is_ascii_lowercase() && c.is_ascii_uppercase())
            }
        };

        if should_add {
            letters.push(c);
        }
        last = Some(c);
    }

    letters
        .into_iter()
        .map(|x| x.to_ascii_uppercase())
        .collect::<String>()
}
