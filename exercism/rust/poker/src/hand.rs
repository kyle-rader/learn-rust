use crate::card::{Card, CardParsingError};

use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum HandParsingError {
    #[error("{n} is not the right number of cards. (A hand has 5 cards)")]
    InvalidSize { n: usize },
    #[error(transparent)]
    CardError(#[from] CardParsingError),
}

#[derive(Debug, PartialEq)]
pub struct Hand<'a> {
    pub hand: &'a str,
    pub cards: Vec<Card>,
    // pub score: Score,
}

impl<'a> TryFrom<&'a str> for Hand<'a> {
    type Error = HandParsingError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut cards = s
            .split_ascii_whitespace()
            .map(Card::try_from)
            .collect::<Result<Vec<Card>, CardParsingError>>()?;

        if cards.len() != 5 {
            return Err(HandParsingError::InvalidSize { n: cards.len() });
        }

        cards.sort();

        Ok(Hand {
            hand: s,
            cards,
            // score,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        rank::{Rank, RankParsingError},
        suit::{Suit, SuitParsingError},
    };

    use super::*;
    use test_case::test_case;

    #[test]
    fn can_make_a_hand() {
        let original = "4S 5S 7H 8D JC";
        let expected = Ok(Hand {
            hand: &original,
            cards: vec![
                Card {
                    rank: Rank::Four,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Clubs,
                },
            ],
        });

        let hand = Hand::try_from(original);

        assert_eq!(hand, expected);
    }

    #[test]
    fn try_from_bad_suit_in_hand() {
        let original = "ZZ 5S AH KC 3S";
        let expected = HandParsingError::CardError(CardParsingError::InvalidSuit(
            SuitParsingError::InvalidSuit { suit: 'Z' },
        ));

        assert_eq!(Hand::try_from(original).unwrap_err(), expected);
        assert_eq!(format!("{expected}"), "'Z' is not a suit.");
    }

    #[test]
    fn try_from_bad_rank_in_hand() {
        let original = "4H 5S AH ZD 3S";
        let expected = HandParsingError::CardError(CardParsingError::InvalidRank(
            RankParsingError::InvalidRank {
                rank: String::from("Z"),
            },
        ));

        assert_eq!(Hand::try_from(original).unwrap_err(), expected);
        assert_eq!(format!("{expected}"), "'Z' is not a rank.");
    }

    #[test_case("AD 2D 3D 4D 5D KD", 6 ; "Too many cards")]
    #[test_case("AD 2D 3D 4D", 4 ; "Too few cards")]
    fn try_from_too_many_cards_is_error(input: &str, n: usize) {
        let expected = HandParsingError::InvalidSize { n };

        assert_eq!(Hand::try_from(input).unwrap_err(), expected);
        assert_eq!(
            format!("{expected}"),
            format!("{n} is not the right number of cards. (A hand has 5 cards)")
        );
    }
}
