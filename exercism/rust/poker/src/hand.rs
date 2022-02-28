use std::str::FromStr;

use crate::card::{Card, CardParsingError};

use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum HandParsingError {
    #[error("{msg:?}")]
    Error { msg: String },
    #[error(transparent)]
    CardError(#[from] CardParsingError),
}
#[derive(Debug, PartialEq)]
pub struct Hand<'a> {
    hand: &'a str,
    cards: Vec<Card>,
}

impl<'a> TryFrom<&'a str> for Hand<'a> {
    type Error = HandParsingError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let cards = s
            .split_ascii_whitespace()
            .map(Card::from_str)
            .collect::<Result<Vec<Card>, CardParsingError>>()?;

        Ok(Hand { hand: s, cards })
    }
}

#[cfg(test)]
mod tests {
    use crate::{rank::Rank, suit::Suit};

    use super::*;

    #[test]
    fn can_make_a_hand() {
        let original = "4S 5S 7H 8D JC";
        let expected = Ok(Hand {
            hand: &original,
            cards: vec![
                Card {
                    rank: Rank::Four,
                    suit: Suit::Spade,
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Spade,
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Heart,
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Club,
                },
            ],
        });

        let hand = Hand::try_from(original);

        assert_eq!(hand, expected);
    }

    #[test]
    fn bad_cards_in_hand() {
        let original = "ZZ 5S AH KC 3S";
        let expected = HandParsingError::CardError(CardParsingError::SubError {
            msg: String::from("not a Rank!\n'Z' is not a suit!"),
        });

        assert_eq!(Hand::try_from(original).unwrap_err(), expected);
        assert_eq!(
            format!("{expected}"),
            "Sub error: not a Rank!\n'Z' is not a suit!"
        );
    }
}
