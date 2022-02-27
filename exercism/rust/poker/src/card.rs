use std::str::FromStr;

use crate::suit::Suit;
use crate::rank::Rank;

#[derive(Debug, PartialEq)]
struct Card {
    rank: Rank,
    suit: Suit,
}

static ERR_INPUT_TOO_SHORT: &str = "Input string is too short!";
static ERR_INPUT_TOO_LONG: &str = "Input string is too long!";

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if (0..=1).contains(&s.len()) {
            return Err(String::from(ERR_INPUT_TOO_SHORT));
        }
        else if s.len() > 3 {
            return Err(String::from(ERR_INPUT_TOO_LONG));
        }

        let last_char = s.chars().last().unwrap(); // unwrap because we have asserted 

        let suit = match Suit::try_from(last_char) {
            Ok(s) => s,
            Err(msg) => return Err(msg),
        };

        Err(String::from("todo!"))

        // let rank = if s.len() == 3 {
        //     10
        // } else {
        //     let c = s.chars().nth(0).unwrap();
        //     let n = c as u8 - ('0' as u8);
        //     match (c, n) {
        //         (_, x @ 1..=9) => x,
        //         ('J', _) => 11,
        //         ('Q', _) => 12,
        //         ('K', _) => 13,
        //         ('A', _) => 14,
        //         _ => Err("Unknown rank '{c}'!"),
        //     }
        // };

        // Ok(Card { rank, suit })
    }
}

#[cfg(test)]
mod card_tests {
    use super::*;

    #[test]
    fn try_from_no_chars() {
        let subject  = "".parse::<Card>();
        let expected = Err(String::from(ERR_INPUT_TOO_SHORT));
        assert_eq!(subject, expected);
    }

    #[test]
    fn try_from_too_many_chars() {
        let subject = Card::from_str("4TOOLONG");
        let expected = Err(String::from(ERR_INPUT_TOO_LONG));
        assert_eq!(subject, expected);
    }

    #[test]
    fn try_from_bad_suit() {
        let subjects = vec![
            Card::from_str("3K"),
            Card::from_str("10Z"),
            Card::from_str("5Ā"),
        ];

        for subject in subjects.iter() {
            match subject {
                Ok(_) => panic!("Oops, none of these should parse!"),
                Err(msg) => assert!(msg.ends_with("not a suit!")),
            }
        }
    }
}
