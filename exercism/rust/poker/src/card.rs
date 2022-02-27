use std::str::FromStr;

use crate::rank::Rank;
use crate::suit::Suit;

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
        } else if s.len() > 3 {
            return Err(String::from(ERR_INPUT_TOO_LONG));
        }

        // unwrap the last char because we have asserted above there *is* a last char.
        let last_char = s.chars().last().unwrap();

        let suit = match Suit::try_from(last_char) {
            Ok(s) => s,
            Err(msg) => return Err(msg),
        };

        let rank = match Rank::from_str(&s[0..(s.len()-1)]) {
            Ok(r) => r,
            Err(msg) => return Err(msg),
        };

        Ok(Card { rank, suit })
    }
}

#[cfg(test)]
mod card_tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn from_str_no_chars() {
        let subject = "".parse::<Card>();
        let expected = Err(String::from(ERR_INPUT_TOO_SHORT));
        assert_eq!(subject, expected);
    }

    #[test]
    fn from_str_too_many_chars() {
        let subject = Card::from_str("4TOOLONG");
        let expected = Err(String::from(ERR_INPUT_TOO_LONG));
        assert_eq!(subject, expected);
    }

    #[test]
    fn from_str_bad_suit() {
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

    #[test_case("AH", Rank::Ace, Suit::Heart ; "Ace of Hearts")]
    #[test_case("2H", Rank::Two, Suit::Heart ; "Two of Spades")]
    #[test_case("3S", Rank::Three, Suit::Spade ; "Three of Spades")]
    #[test_case("10C", Rank::Ten, Suit::Club ; "Ten of Clubs")]
    #[test_case("JD", Rank::Jack, Suit::Diamond ; "Jack of Diamonds")]
    fn from_str_parses_a_card(subject: &str, rank: Rank, suit: Suit) {
        let expected = Ok(Card {
            rank,
            suit,
        });
        assert_eq!(subject.parse(), expected);
    }
}
