/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    unimplemented!("Out of {:?}, which hand wins?", hands)
}

#[derive(Debug)]
struct Hand<'a> {
    hand: &'a str,
    // cards: Vec<Card>,
}

impl Hand<'_> {
    fn new<'a>(hand: &'a str) -> Hand {
        Hand { hand }
    }
}

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

#[derive(Debug, PartialEq)]
enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

impl From<char> for Suit {
    fn from(c: char) -> Self {
        match c {
            'H' | 'h' => Suit::Heart,
            'D' | 'd' => Suit::Diamond,
            'C' | 'c' => Suit::Club,
            'S' | 's' => Suit::Spade,
            _ => panic!("{c} is an unknown suit!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suit_from_char() {
        assert_eq!(Suit::from('C'), Suit::Club);
        assert_eq!(Suit::from('H'), Suit::Heart);
        assert_eq!(Suit::from('D'), Suit::Diamond);
        assert_eq!(Suit::from('S'), Suit::Spade);
        assert_eq!(Suit::from('c'), Suit::Club);
        assert_eq!(Suit::from('h'), Suit::Heart);
        assert_eq!(Suit::from('d'), Suit::Diamond);
        assert_eq!(Suit::from('s'), Suit::Spade);
    }

    #[test]
    #[should_panic]
    fn suit_panics() {
        let _ = Suit::from('a');
    }

    #[test]
    fn test_can_make_a_hand() {
        let original = "4S 5S 7H 8D JC";
        let hand = Hand::new(original);
        assert_eq!(original, hand.hand);
    }

    #[test]
    fn test_card_from_string() {
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
    fn test_card_with_rank_ten() {
        let card = Card::from("10S");
        assert_eq!(card, Card { rank: 10, suit: Suit::Spade });
    }

    #[test]
    fn test_cards_above_ten() {
        let jack = Card::from("JH");
        assert_eq!(jack, Card { rank: 11, suit: Suit::Heart});
    }

    #[test]
    fn test_cards_are_not_equal() {
        let three_of_spades = Card::from("3S");
        let nine_of_hearts = Card::from("9H");
        assert_ne!(three_of_spades, nine_of_hearts);
    }
}
