pub mod suit;
use crate::suit::Suit;

#[derive(Debug, PartialEq)]
struct Card {
    rank: u8,
    suit: Suit,
}

impl From<&str> for Card {
    fn from(s: &str) -> Self {
        let rank = if s.len() == 3 {
            10
        } else {
            let c = s.chars().nth(0).unwrap();
            let n = c as u8 - ('0' as u8);
            match (c, n) {
                (_, x @ 1..=9) => x,
                ('J', _) => 11,
                ('Q', _) => 12,
                ('K', _) => 13,
                ('A', _) => 14,
                _ => panic!("Unknown rank '{c}'!"),
            }
        };
        let suit = Suit::from(s.chars().last().unwrap());
        Card { rank, suit }
    }
}

#[cfg(test)]
mod card_tests {
    use super::*;

    #[test]
    fn card_from_string() {
        let card = Card::from("4S");
        assert_eq!(
            card,
            Card {
                rank: 4,
                suit: Suit::Spade
            }
        );
    }

    #[test]
    fn card_with_rank_ten() {
        let card = Card::from("10S");
        assert_eq!(card, Card { rank: 10, suit: Suit::Spade });
    }

    #[test]
    fn cards_above_ten() {
        let jack = Card::from("JH");
        assert_eq!(jack, Card { rank: 11, suit: Suit::Heart});
    }

    #[test]
    fn cards_are_not_equal() {
        let three_of_spades = Card::from("3S");
        let nine_of_hearts = Card::from("9H");
        assert_ne!(three_of_spades, nine_of_hearts);
    }

    #[test]
    fn cards_can_sort() {
        let initial: Vec<Card> = vec!["KS", "3D", "8C", "5H", "6H"]
            .iter()
            .map(|c| Card::from(*c))
            .collect();
    }
}
