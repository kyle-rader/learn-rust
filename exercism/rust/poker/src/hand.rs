use std::error::Error;
use std::str::FromStr;

use crate::card::{Card, CardParsingError};
use crate::rank::Rank;
use crate::suit::Suit;

use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum HandParsingError {
    #[error("{msg:?}")]
    Error { msg: String },
}

#[derive(Debug, PartialEq)]
pub struct Hand<'a> {
    hand: &'a str,
    cards: Vec<Card>,
}

impl<'a> FromStr for Hand<'a> {
    type Err = HandParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Err(HandParsingError::Error {
            msg: String::from("todo!"),
        })

        // let cards = s
        //     .split_ascii_whitespace()
        //     .map(Card::from_str)
        //     .collect::<Result<Vec<Card>, CardParsingError>>();

        // match cards {
        //     Ok(c) => Ok(Hand { hand: s, cards: c }),
        //     Err(msg) => Err(HandParsingError::Error {
        //         msg: msg.to_string(),
        //     }),
        // }
    }
}

#[cfg(test)]
mod tests {
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

        let hand = original.parse::<Hand>();

        assert_eq!(hand, expected);
    }
}
