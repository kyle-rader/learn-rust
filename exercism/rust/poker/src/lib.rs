use hand::Hand;

pub mod card;
pub mod hand;
pub mod rank;
pub mod score;
pub mod suit;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut hands: Vec<Hand> = hands
        .iter()
        .filter_map(|h| Hand::try_from(*h).ok())
        .collect();

    hands.sort();
    hands.reverse();
    let hands = hands;

    let mut top_hands: Vec<&'a str> = Vec::new();
    if let Some(top_hand) = hands.iter().nth(0) {
        for h in hands.iter() {
            if h.score == top_hand.score {
                top_hands.push(h.hand);
            } else {
                break;
            }
        }
    }
    top_hands
}
