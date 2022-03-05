use crate::rank::Rank;

#[derive(Debug, PartialEq, Eq)]
pub enum Score {
    HighCard(Rank),
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}
