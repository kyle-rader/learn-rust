use std::collections::{HashMap, HashSet};

use crate::{
    card::{Card, CardParsingError},
    rank::Rank,
    score::Score,
    suit::Suit,
};

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
const ROYAL_FLUSH: &[Rank; 5] = &[Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace];

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

fn calculate_score(cards: &Vec<Card>) -> Score {
    let mut ranks: HashMap<Rank, usize> = HashMap::new();
    let mut suits: HashMap<Suit, usize> = HashMap::new();

    for card in cards.iter() {
        let rank_count = ranks.entry(card.rank).or_insert(0);
        *rank_count += 1;
        let suit_count = suits.entry(card.suit).or_insert(0);
        *suit_count += 1;
    }

    let mut count_to_ranks: HashMap<usize, HashSet<Rank>> = HashMap::new();
    for (rank, count) in ranks.iter() {
        let count_rank = count_to_ranks.entry(*count).or_default();
        count_rank.insert(*rank);
    }

    // Ensure that each count does have a HashSet so that we do not have to borrow as mutable multiple times later.
    for i in 0..=HAND_SIZE {
        let _ = count_to_ranks.entry(i).or_default();
    }

    // Rebind as immutable
    let count_to_ranks = count_to_ranks;

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
    if count_to_ranks.get(&4).unwrap().len() == 1 {
        return Score::FourOfAKind;
    }

    // Full House
    let trios = count_to_ranks.get(&3).unwrap();
    let pairs = count_to_ranks.get(&2).unwrap();

    if trios.len() == 1 && pairs.len() == 1 {
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
    if trios.len() == 1 {
        return Score::ThreeOfAKind;
    }

    // Two Pair
    match pairs.len() {
        2 => Score::TwoPair,
        1 => Score::Pair,
        _ => Score::HighCard,
    }
}

#[cfg(test)]
mod tests {
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

    #[test_case("10H JH QH KH AH" ; "Hearts")]
    #[test_case("10S JS QS KS AS" ; "Spades")]
    #[test_case("10C JC QC KC AC" ; "Clubs")]
    #[test_case("10D JD QD KD AD" ; "Diamonds")]
    fn hand_score_royal_flush(input: &str) {
        let subject = Hand::try_from(input).unwrap();
        assert_eq!(subject.score, Score::RoyalFlush);
    }

    #[test_case("AS 2S 3S 4S 5S")]
    #[test_case("2S 3S 4S 5S 6S")]
    #[test_case("3S 4S 5S 6S 7S")]
    #[test_case("4S 5S 6S 7S 8S")]
    #[test_case("5S 6S 7S 8S 9S")]
    #[test_case("6S 7S 8S 9S 10S")]
    #[test_case("7S 8S 9S 10S JS")]
    #[test_case("8S 9S 10S JS QS")]
    #[test_case("9S 10S JS QS KS")]
    fn hand_score_straight_flush(input: &str) {
        let subject = Hand::try_from(input).unwrap();
        assert_eq!(subject.score, Score::StraightFlush);
    }

    #[test_case("4S 4C 4H 4D KS")]
    #[test_case("AS AC AH AD 3S")]
    fn hand_score_four_of_a_kind(input: &str) {
        let subject = Hand::try_from(input).unwrap();
        assert_eq!(subject.score, Score::FourOfAKind);
    }

    #[test_case("4S 4C KH KD KS" ; "In order")]
    #[test_case("4C KH 4S KD KS" ; "Out of order")]
    #[test_case("AC KH AS KD AS" ; "With Aces")]
    fn hand_score_full_house(input: &str) {
        let subject = Hand::try_from(input).unwrap();
        assert_eq!(subject.score, Score::FullHouse);
    }

    #[test_case("4S 5S 2S 9S KS" ; "Spades")]
    #[test_case("4C 5C 2C 9C KC" ; "Clubs")]
    #[test_case("4D 5D 2D 9D KD" ; "Diamonds")]
    #[test_case("4H 5H 2H 9H KH" ; "Hearts")]
    fn hand_score_flush(input: &str) {
        let subject = Hand::try_from(input).unwrap();
        assert_eq!(subject.score, Score::Flush);
    }

    #[test]
    fn hand_score_straight() {
        let subject = Hand::try_from("4S 5C 6S 7D 8H").unwrap();
        assert_eq!(subject.score, Score::Straight);
    }

    #[test]
    fn hand_score_three_of_a_kind() {
        let subject = Hand::try_from("5S 5H 5D JS 2C").unwrap();
        assert_eq!(subject.score, Score::ThreeOfAKind);
    }

    #[test_case("6S 6H JD JS 2C" ; "In order")]
    #[test_case("6S JD JS 2C 6H" ; "Out of order")]
    #[test_case("KS JD 2C JS KH" ; "Higher than Jack")]
    fn hand_score_two_pair(input: &str) {
        let subject = Hand::try_from(input).unwrap();
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
