use thiserror::Error;

use crate::rank::{Rank, RankParsingError};
use crate::suit::{Suit, SuitParsingError};

#[derive(Debug, PartialEq, Error)]
pub enum CardParsingError {
    #[error("Error: Input string is too short!")]
    TooShort,
    #[error("Error: Input string is too long!")]
    TooLong,
    #[error(transparent)]
    InvalidRank(#[from] RankParsingError),
    #[error(transparent)]
    InvalidSuit(#[from] SuitParsingError),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl TryFrom<&str> for Card {
    type Error = CardParsingError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if (0..=1).contains(&s.len()) {
            return Err(CardParsingError::TooShort);
        } else if s.len() > 3 {
            return Err(CardParsingError::TooLong);
        }

        // unwrap the last char because we have asserted above there *is* a last char.
        let last_char = s.chars().last().unwrap();
        let suit = Suit::try_from(last_char)?;

        let rank = s.chars().take(s.chars().count() - 1).collect::<String>();
        let rank = Rank::try_from(&rank[..])?;

        Ok(Card { rank, suit })
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.rank.partial_cmp(&other.rank)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank.cmp(&other.rank)
    }
}

#[cfg(test)]
mod card_tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn from_str_no_chars() {
        let subject = Card::try_from("").unwrap_err();
        let expected = CardParsingError::TooShort;
        assert_eq!(subject, expected);
    }

    #[test]
    fn from_str_too_many_chars() {
        let subject = Card::try_from("4TOOLONG").unwrap_err();
        let expected = CardParsingError::TooLong;
        assert_eq!(subject, expected);
    }

    #[test_case("3K" ; "King is not a suit")]
    #[test_case("10Z" ; "Z is not a suit")]
    #[test_case("5Ā" ; "Other unicode chars are not a suit")]
    fn from_str_bad_suit(subject: &str) {
        let subject = Card::try_from(subject);

        match subject {
            Err(err) => {
                assert!(err.to_string().contains("is not a suit."));
            }
            _ => panic!("should be suit parsing error!"),
        }
    }

    #[test_case("AH", Rank::Ace, Suit::Hearts ; "Ace of Hearts")]
    #[test_case("2H", Rank::Two, Suit::Hearts ; "Two of Spades")]
    #[test_case("3S", Rank::Three, Suit::Spades ; "Three of Spades")]
    #[test_case("10C", Rank::Ten, Suit::Clubs ; "Ten of Clubs")]
    #[test_case("JD", Rank::Jack, Suit::Diamonds ; "Jack of Diamonds")]
    fn from_str_parses_a_card(subject: &str, rank: Rank, suit: Suit) {
        let expected = Ok(Card { rank, suit });
        assert_eq!(Card::try_from(subject), expected);
    }

    #[test]
    fn card_can_sort() {
        let expected = vec![
            Card {
                rank: Rank::Four,
                suit: Suit::Spades,
            },
            Card {
                rank: Rank::Nine,
                suit: Suit::Clubs,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Hearts,
            },
            Card {
                rank: Rank::Ace,
                suit: Suit::Clubs,
            },
        ];

        let mut subject = vec![
            Card {
                rank: Rank::Four,
                suit: Suit::Spades,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Hearts,
            },
            Card {
                rank: Rank::Nine,
                suit: Suit::Clubs,
            },
            Card {
                rank: Rank::Ace,
                suit: Suit::Clubs,
            },
        ];

        subject.sort();
        assert_eq!(subject, expected);
    }
}
