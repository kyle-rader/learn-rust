use crate::suit::Suit;
use crate::rank::Rank;
use crate::card::Card;

#[derive(Debug)]
pub struct Hand<'a> {
    hand: &'a str,
    // cards: Vec<Card>,
}

impl Hand<'_> {
    fn new<'a>(hand: &'a str) -> Hand {
        Hand { hand }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_make_a_hand() {
        let original = "4S 5S 7H 8D JC";
        let hand = Hand::new(original);
        assert_eq!(original, hand.hand);
    }
}
