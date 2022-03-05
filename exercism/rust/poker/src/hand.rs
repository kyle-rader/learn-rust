use crate::{
    card::{Card, CardParsingError},
    rank::Rank,
    score::Score,
    suit::Suit,
};

use counter::Counter;
use enum_iterator::IntoEnumIterator;
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
    pub score: Score,
}

const HAND_SIZE: usize = 5;

impl<'a> TryFrom<&'a str> for Hand<'a> {
    type Error = HandParsingError;

    fn try_from(hand: &'a str) -> Result<Self, Self::Error> {
        let mut cards = hand
            .split_ascii_whitespace()
            .map(Card::try_from)
            .collect::<Result<Vec<Card>, CardParsingError>>()?;

        if cards.len() != HAND_SIZE {
            return Err(HandParsingError::InvalidSize { n: cards.len() });
        }

        cards.sort(); // sort so we can easily get the high card
        let cards = cards; // no longer mut

        let score = calculate_score(&cards);

        Ok(Hand { hand, cards, score })
    }
}

const ROYAL_FLUSH: &[Rank; 5] = &[Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace];

fn calculate_score(cards: &Vec<Card>) -> Score {
    let ranks: Counter<Rank> = cards.iter().map(|c| c.rank).collect();
    let suits: Counter<Suit> = cards.iter().map(|c| c.suit).collect();

    // Royal Flush
    if suits.len() == 1 && ROYAL_FLUSH.into_iter().all(|r| ranks.contains_key(&r)) {
        return Score::RoyalFlush;
    }

    // Straight Flush
    let mut all_ranks = Vec::new();
    for r in Rank::into_enum_iter() {
        all_ranks.push(r);
    }

    let is_straight = all_ranks
        .windows(HAND_SIZE)
        .any(|flush| flush.iter().all(|r| ranks.contains_key(r)));

    let is_flush = suits.len() == 1;

    if is_straight && is_flush {
        return Score::StraightFlush;
    }

    // Four of a Kind
    if ranks.values().any(|v| *v == 4) {
        return Score::FourOfAKind;
    }

    // Full House
    let is_three_of_kind = ranks.values().any(|v| *v == 3);
    if is_three_of_kind && ranks.values().any(|v| *v == 2) {
        return Score::FullHouse;
    }

    // Flush
    if is_flush {
        return Score::Flush;
    }

    // Straight
    if is_straight {
        return Score::Straight;
    }

    // Three of a Kind
    if is_three_of_kind {
        return Score::ThreeOfAKind;
    }

    // Two Pair
    let pairs_count = ranks.values().fold(0, |acc, rank_count| {
        acc + if *rank_count == 2 { 1 } else { 0 }
    });

    match pairs_count {
        2 => Score::TwoPair,
        1 => Score::Pair,
        _ => Score::HighCard,
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Sub;

    use crate::{
        rank::{Rank, RankParsingError},
        score::Score,
        suit::{Suit, SuitParsingError},
    };

    use super::*;
    use test_case::test_case;

    #[test]
    fn can_make_a_hand() {
        let original = "4S 5S 7H 8D JC";
        let expected = Ok(Hand {
            hand: &original,
            score: Score::HighCard,
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

    #[test]
    fn hand_score_royal_flush() {
        let subject = Hand::try_from("10H JH QH KH AH").unwrap();
        assert_eq!(subject.score, Score::RoyalFlush);
    }

    #[test]
    fn hand_score_straight_flush() {
        let subject = Hand::try_from("4S 5S 6S 7S 8S").unwrap();
        assert_eq!(subject.score, Score::StraightFlush);
    }

    #[test]
    fn hand_score_four_of_a_kind() {
        let subject = Hand::try_from("4S 4C 4H 4D KS").unwrap();
        assert_eq!(subject.score, Score::FourOfAKind);
    }

    #[test]
    fn hand_score_full_house() {
        let subject = Hand::try_from("4S 4C KH KD KS").unwrap();
        assert_eq!(subject.score, Score::FullHouse);
    }

    #[test]
    fn hand_score_flush() {
        let subject = Hand::try_from("4S 5S 2S 9S KS").unwrap();
        assert_eq!(subject.score, Score::Flush);
    }

    #[test]
    fn hand_score_straight() {
        let subject = Hand::try_from("4S 5C 6S 7D 8H").unwrap();
        assert_eq!(subject.score, Score::Straight);
    }

    #[test]
    fn hand_score_three_of_a_kind() {
        let subject = Hand::try_from("6S 6H 6D JS 2C").unwrap();
        assert_eq!(subject.score, Score::ThreeOfAKind);
    }

    #[test]
    fn hand_score_two_pair() {
        let subject = Hand::try_from("6S 6H JD JS 2C").unwrap();
        assert_eq!(subject.score, Score::TwoPair);
    }

    #[test]
    fn hand_score_pair() {
        let subject = Hand::try_from("6S 3H JD JS 2C").unwrap();
        assert_eq!(subject.score, Score::Pair);
    }

    #[test]
    fn hand_score_high_card() {
        let subject = Hand::try_from("6S 3H 10D JS 2C").unwrap();
        assert_eq!(subject.score, Score::HighCard);
    }
}
