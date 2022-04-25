use std::collections::{HashMap, HashSet};

use crate::{
    card::{Card, CardParsingError},
    rank::Rank,
    score::Score,
    suit::Suit,
};

// use thiserror::Error;

// #[derive(Debug, PartialEq, Error)]
// pub enum HandParsingError {
//     #[error("{n} is not the right number of cards. (A hand has 5 cards)")]
//     InvalidSize { n: usize },
//     #[error(transparent)]
//     CardError(#[from] CardParsingError),
// }
#[derive(Debug, PartialEq)]
pub enum HandParsingError {
    InvalidSize { n: usize },
    CardError(CardParsingError),
}

#[derive(Debug)]
pub struct Hand<'a> {
    pub hand: &'a str,
    pub cards: Vec<Card>,
    pub score: Score,
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl<'a> Eq for Hand<'a> {}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

const HAND_SIZE: usize = 5;

// RANK_LIST for windows of straights - excluding ace low/high straights.
const RANK_LIST: &[Rank; 12] = &[
    Rank::Two,
    Rank::Three,
    Rank::Four,
    Rank::Five,
    Rank::Six,
    Rank::Seven,
    Rank::Eight,
    Rank::Nine,
    Rank::Ten,
    Rank::Jack,
    Rank::Queen,
    Rank::King,
];

const STRAIGHT_ACE_LOW: &[Rank; 5] = &[Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five];
const STRAIGHT_ACE_HIGH: &[Rank; 5] = &[Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace];

impl<'a> TryFrom<&'a str> for Hand<'a> {
    type Error = HandParsingError;

    fn try_from(hand: &'a str) -> Result<Self, Self::Error> {
        let mut cards = match cards(hand) {
            Ok(cards) => cards,
            Err(card_parse_error) => return Err(HandParsingError::CardError(card_parse_error)),
        };

        if cards.len() != HAND_SIZE {
            return Err(HandParsingError::InvalidSize { n: cards.len() });
        }

        cards.sort(); // sort so we can easily get the high card
        let cards = cards; // no longer mut

        let score = calculate_score(&cards);

        Ok(Hand { hand, cards, score })
    }
}

fn cards(hand: &str) -> Result<Vec<Card>, CardParsingError> {
    hand.split_ascii_whitespace()
        .map(Card::try_from)
        .collect::<Result<Vec<Card>, CardParsingError>>()
}

fn calculate_score(cards: &Vec<Card>) -> Score {
    let mut ranks: HashMap<Rank, usize> = HashMap::new();
    let mut suits: HashMap<Suit, usize> = HashMap::new();
    let mut kicker: HashSet<Rank> = HashSet::new();

    for card in cards.iter() {
        let rank_count = ranks.entry(card.rank).or_insert(0);
        *rank_count += 1;
        let suit_count = suits.entry(card.suit).or_insert(0);
        *suit_count += 1;
        kicker.insert(card.rank);
    }

    let mut count_to_ranks: HashMap<usize, HashSet<Rank>> = HashMap::new();
    for (rank, count) in ranks.iter() {
        let count_rank = count_to_ranks.entry(*count).or_default();
        count_rank.insert(*rank);
    }
    // Rebind as immutable
    let count_to_ranks = count_to_ranks;

    // Other characteristics of the hand
    let high_rank = cards.iter().last().unwrap().rank; // default high rank, Aces are high.
    let is_flush = suits.len() == 1;
    let trios = count_to_ranks.get(&3);
    let pairs = count_to_ranks.get(&2);

    // Single Pair is ~42% of hands so check for that first
    if let Some(pairs) = pairs {
        let mut pair_ranks: Vec<Rank> = pairs.iter().copied().collect();
        pair_ranks.sort();
        for r in pair_ranks.iter() {
            kicker.remove(r);
        }

        if let Some(trios) = trios {
            // Full House
            let trio = *trios.iter().nth(0).unwrap();
            let pair = *pairs.iter().nth(0).unwrap();
            return Score::FullHouse { trio, pair };
        }

        return if pairs.len() == 2 {
            // Two Pair
            Score::TwoPair {
                low: pair_ranks[0],
                high: pair_ranks[1],
                kicker: kicker.into_iter().nth(0).unwrap(),
            }
        } else {
            // Single Pair
            let mut kickers: Vec<Rank> = kicker.into_iter().collect();
            kickers.sort_by(|a, b| b.cmp(a));
            Score::Pair {
                rank: pair_ranks[0],
                kickers,
            }
        };
    }

    // Three of a Kind
    if let Some(trios) = trios {
        let rank = *trios.iter().nth(0).unwrap();
        kicker.remove(&rank);
        let mut kickers: Vec<Rank> = kicker.into_iter().collect();
        kickers.sort_by(|a, b| b.cmp(a));
        return Score::ThreeOfAKind { rank, kickers };
    }

    // Straight stats
    let is_straight_ace_high = STRAIGHT_ACE_HIGH.iter().all(|r| ranks.contains_key(&r));
    let is_straight_ace_low = STRAIGHT_ACE_LOW.iter().all(|r| ranks.contains_key(&r));
    let is_straight = RANK_LIST
        .windows(HAND_SIZE)
        .any(|flush| flush.iter().all(|r| ranks.contains_key(r)))
        || is_straight_ace_high
        || is_straight_ace_low;
    let straight_rank = if is_straight_ace_low {
        Rank::Five
    } else {
        high_rank
    };

    // Royal Flush
    if is_straight_ace_high && is_flush {
        return Score::RoyalFlush;
    }

    // Straight Flush
    if is_straight && is_flush {
        return Score::StraightFlush {
            rank: straight_rank,
        };
    }

    // Four of a Kind
    if let Some(four_of_a_kind_rank) = count_to_ranks.get(&4) {
        let rank = *four_of_a_kind_rank.iter().nth(0).unwrap();
        kicker.remove(&rank);
        return Score::FourOfAKind {
            rank,
            kicker: kicker.into_iter().nth(0).unwrap(),
        };
    }

    // Flush
    if is_flush {
        let mut kickers: Vec<Rank> = kicker.into_iter().collect();
        kickers.sort_by(|a, b| b.cmp(a));
        return Score::Flush { kickers };
    }

    // Straight
    if is_straight {
        return Score::Straight {
            rank: straight_rank,
        };
    }

    // High Card remains
    let mut kickers: Vec<Rank> = kicker.into_iter().collect();
    kickers.sort_by(|a, b| b.cmp(a));
    Score::HighCard { kickers }
}

// #[cfg(test)]
// mod hand_tests {
//     use crate::{
//         rank::{Rank, RankParsingError},
//         score::Score,
//         suit::{Suit, SuitParsingError},
//     };

//     use super::*;
//     use test_case::test_case;

//     #[test]
//     fn can_make_a_hand() {
//         let original = "JC 4S 7H 5S 8D";

//         let expected = Ok(Hand {
//             hand: &original,
//             score: Score::HighCard {
//                 kickers: vec![Rank::Jack, Rank::Eight, Rank::Seven, Rank::Five, Rank::Four],
//             },
//             cards: vec![
//                 Card {
//                     rank: Rank::Four,
//                     suit: Suit::Spades,
//                 },
//                 Card {
//                     rank: Rank::Five,
//                     suit: Suit::Spades,
//                 },
//                 Card {
//                     rank: Rank::Seven,
//                     suit: Suit::Hearts,
//                 },
//                 Card {
//                     rank: Rank::Eight,
//                     suit: Suit::Diamonds,
//                 },
//                 Card {
//                     rank: Rank::Jack,
//                     suit: Suit::Clubs,
//                 },
//             ],
//         });

//         let hand = Hand::try_from(original);

//         assert_eq!(hand, expected);
//     }

//     #[test]
//     fn try_from_bad_suit_in_hand() {
//         let original = "ZZ 5S AH KC 3S";
//         let expected = HandParsingError::CardError(CardParsingError::InvalidSuit(
//             SuitParsingError::InvalidSuit { suit: 'Z' },
//         ));

//         assert_eq!(Hand::try_from(original).unwrap_err(), expected);
//         assert_eq!(format!("{expected}"), "'Z' is not a suit.");
//     }

//     #[test]
//     fn try_from_bad_rank_in_hand() {
//         let original = "4H 5S AH ZD 3S";
//         let expected = HandParsingError::CardError(CardParsingError::InvalidRank(
//             RankParsingError::InvalidRank {
//                 rank: String::from("Z"),
//             },
//         ));

//         assert_eq!(Hand::try_from(original).unwrap_err(), expected);
//         assert_eq!(format!("{expected}"), "'Z' is not a rank.");
//     }

//     #[test_case("AD 2D 3D 4D 5D KD", 6 ; "Too many cards")]
//     #[test_case("AD 2D 3D 4D", 4 ; "Too few cards")]
//     fn try_from_too_many_cards_is_error(input: &str, n: usize) {
//         let expected = HandParsingError::InvalidSize { n };

//         assert_eq!(Hand::try_from(input).unwrap_err(), expected);
//         assert_eq!(
//             format!("{expected}"),
//             format!("{n} is not the right number of cards. (A hand has 5 cards)")
//         );
//     }

//     #[test_case("10H JH QH KH AH" ; "Hearts")]
//     #[test_case("10S JS QS KS AS" ; "Spades")]
//     #[test_case("10C JC QC KC AC" ; "Clubs")]
//     #[test_case("10D JD QD KD AD" ; "Diamonds")]
//     fn hand_score_royal_flush(input: &str) {
//         let subject = Hand::try_from(input).unwrap();
//         assert_eq!(subject.score, Score::RoyalFlush);
//     }

//     #[test_case("AS 2S 3S 4S 5S", Rank::Five)]
//     #[test_case("2S 3S 4S 5S 6S", Rank::Six)]
//     #[test_case("3S 4S 5S 6S 7S", Rank::Seven)]
//     #[test_case("4S 5S 6S 7S 8S", Rank::Eight)]
//     #[test_case("5S 6S 7S 8S 9S", Rank::Nine)]
//     #[test_case("6S 7S 8S 9S 10S", Rank::Ten)]
//     #[test_case("7S 8S 9S 10S JS", Rank::Jack)]
//     #[test_case("8S 9S 10S JS QS", Rank::Queen)]
//     #[test_case("9S 10S JS QS KS", Rank::King)]
//     fn hand_score_straight_flush(input: &str, rank: Rank) {
//         let subject = Hand::try_from(input).unwrap();
//         assert_eq!(subject.score, Score::StraightFlush { rank });
//     }

//     #[test_case("4S 4C 4H 4D KS", Rank::Four, Rank::King)]
//     #[test_case("AS AC AH AD 3S", Rank::Ace, Rank::Three)]
//     fn hand_score_four_of_a_kind(input: &str, rank: Rank, kicker: Rank) {
//         let subject = Hand::try_from(input).unwrap();
//         assert_eq!(subject.score, Score::FourOfAKind { rank, kicker });
//     }

//     #[test_case("4S 4C KH KD KS", Rank::King, Rank::Four ; "In order")]
//     #[test_case("4C KH 4S KD KS", Rank::King, Rank::Four ; "Out of order")]
//     #[test_case("AC KH AS KD AS", Rank::Ace, Rank::King ; "With Aces")]
//     fn hand_score_full_house(input: &str, trio: Rank, pair: Rank) {
//         let subject = Hand::try_from(input).unwrap();
//         assert_eq!(subject.score, Score::FullHouse { trio, pair });
//     }

//     #[test_case("4S 5S 2S 9S KS" ; "Spades")]
//     #[test_case("4C 5C 2C 9C KC" ; "Clubs")]
//     #[test_case("4D 5D 2D 9D KD" ; "Diamonds")]
//     #[test_case("4H 5H 2H 9H KH" ; "Hearts")]
//     fn hand_score_flush(input: &str) {
//         let subject = Hand::try_from(input).unwrap();
//         assert!(matches!(subject.score, Score::Flush { .. }));
//     }

//     #[test]
//     fn hand_score_flush_with_kicker() {
//         let subject = Hand::try_from("4H 5H 2H 9H KH").unwrap();
//         assert_eq!(
//             subject.score,
//             Score::Flush {
//                 kickers: vec![Rank::King, Rank::Nine, Rank::Five, Rank::Four, Rank::Two,]
//             }
//         );
//     }

//     #[test]
//     fn hand_score_straight() {
//         let subject = Hand::try_from("4S 5C 6S 7D 8H").unwrap();
//         assert_eq!(subject.score, Score::Straight { rank: Rank::Eight });
//     }

//     #[test]
//     fn hand_score_straight_ace_low() {
//         let subject = Hand::try_from("4S AH 3S 2D 5H").unwrap();
//         assert_eq!(subject.score, Score::Straight { rank: Rank::Five });
//     }

//     #[test]
//     fn hand_score_three_of_a_kind() {
//         let subject = Hand::try_from("5S 5H 5D JS 2C").unwrap();
//         assert_eq!(
//             subject.score,
//             Score::ThreeOfAKind {
//                 rank: Rank::Five,
//                 kickers: vec![Rank::Jack, Rank::Two,],
//             }
//         );
//     }

//     #[test_case("6S 6H JD JS 2C", Rank::Jack, Rank::Six, Rank::Two ; "In order")]
//     #[test_case("6S JD JS 2C 6H", Rank::Jack, Rank::Six, Rank::Two ; "Out of order")]
//     #[test_case("KS JD 5C JS KH", Rank::King, Rank::Jack, Rank::Five ; "Higher than Jack")]
//     fn hand_score_two_pair(input: &str, high: Rank, low: Rank, kicker: Rank) {
//         let subject = Hand::try_from(input).unwrap();
//         assert_eq!(subject.score, Score::TwoPair { high, low, kicker });
//     }

//     #[test]
//     fn hand_score_pair() {
//         let subject = Hand::try_from("6S 3H JD JS 2C").unwrap();
//         assert_eq!(
//             subject.score,
//             Score::Pair {
//                 rank: Rank::Jack,
//                 kickers: vec![Rank::Six, Rank::Three, Rank::Two]
//             }
//         );
//     }

//     #[test]
//     fn hand_score_high_card() {
//         let subject = Hand::try_from("6S 3H 10D JS 2C").unwrap();
//         assert_eq!(
//             subject.score,
//             Score::HighCard {
//                 kickers: vec![Rank::Jack, Rank::Ten, Rank::Six, Rank::Three, Rank::Two]
//             }
//         );
//     }

//     #[test]
//     fn hand_eq_based_on_score() {
//         let hand1 = Hand::try_from("10S JS QS KS AS").unwrap();
//         let hand2 = Hand::try_from("10D JD QD KD AD").unwrap();
//         assert!(hand1 == hand2)
//     }

//     #[test]
//     fn hand_not_eq_based_on_score() {
//         let hand1 = Hand::try_from("5S JS QS KS AS").unwrap();
//         let hand2 = Hand::try_from("10D JD QD KD AD").unwrap();
//         assert!(hand1 != hand2)
//     }

//     #[test]
//     fn hands_are_sortable_by_score() {
//         let hands = ["3H 3S 3D 7D 7C", "10S JS QS KS AS"];

//         let mut hands = hands
//             .iter()
//             .filter_map(|v| Hand::try_from(*v).ok())
//             .collect::<Vec<Hand>>();

//         hands.sort();
//         hands.reverse();

//         assert!(matches!(
//             hands.iter().nth(0).unwrap().score,
//             Score::RoyalFlush
//         ));
//         assert!(matches!(
//             hands.iter().nth(1).unwrap().score,
//             Score::FullHouse {
//                 trio: Rank::Three,
//                 pair: Rank::Seven
//             }
//         ));
//     }
// }
