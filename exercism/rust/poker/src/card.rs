use std::str::FromStr;
use thiserror::Error;

use crate::rank::Rank;
use crate::suit::Suit;

#[derive(Debug, PartialEq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

#[derive(Debug, PartialEq, Error)]
pub enum CardParsingError {
    #[error("Error: Input string is too short!")]
    TooShort,
    #[error("Error: Input string is too long!")]
    TooLong,
    #[error("Sub error: {msg:?}")]
    SubError { msg: String },
}

impl FromStr for Card {
    type Err = CardParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if (0..=1).contains(&s.len()) {
            return Err(CardParsingError::TooShort);
        } else if s.len() > 3 {
            return Err(CardParsingError::TooLong);
        }

        // unwrap the last char because we have asserted above there *is* a last char.
        let last_char = s.chars().last().unwrap();

        let suit = Suit::try_from(last_char);
        let rank = s.chars().take(s.chars().count() - 1).collect::<String>();
        let rank = Rank::from_str(&rank[..]);

        match (rank, suit) {
            (Ok(rank), Ok(suit)) => Ok(Card { rank, suit }),
            (Ok(_), Err(msg)) => Err(CardParsingError::SubError { msg }),
            (Err(msg), Ok(_)) => Err(CardParsingError::SubError { msg }),
            (Err(msg1), Err(msg2)) => Err(CardParsingError::SubError {
                msg: format!("{msg1}\n{msg2}"),
            }),
        }
    }
}

#[cfg(test)]
mod card_tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn from_str_no_chars() {
        let subject = "".parse::<Card>().unwrap_err();
        let expected = CardParsingError::TooShort;
        assert_eq!(subject, expected);
    }

    #[test]
    fn from_str_too_many_chars() {
        let subject = Card::from_str("4TOOLONG").unwrap_err();
        let expected = CardParsingError::TooLong;
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
                Err(err) => match err {
                    CardParsingError::SubError { msg } => assert!(msg.ends_with("not a suit!")),
                    _ => panic!("Wrong kind of parsing error!"),
                },
            }
        }
    }

    #[test_case("AH", Rank::Ace, Suit::Heart ; "Ace of Hearts")]
    #[test_case("2H", Rank::Two, Suit::Heart ; "Two of Spades")]
    #[test_case("3S", Rank::Three, Suit::Spade ; "Three of Spades")]
    #[test_case("10C", Rank::Ten, Suit::Club ; "Ten of Clubs")]
    #[test_case("JD", Rank::Jack, Suit::Diamond ; "Jack of Diamonds")]
    fn from_str_parses_a_card(subject: &str, rank: Rank, suit: Suit) {
        let expected = Ok(Card { rank, suit });
        assert_eq!(subject.parse(), expected);
    }
}
