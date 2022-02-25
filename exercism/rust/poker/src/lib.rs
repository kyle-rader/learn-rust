pub mod suit;
pub mod card;

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
