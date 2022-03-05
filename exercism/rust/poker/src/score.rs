use crate::rank::Rank;

#[derive(Debug, PartialEq, Eq)]
pub enum Score {
    HighCard(Rank),
    Pair(Rank),
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}
