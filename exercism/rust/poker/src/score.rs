use crate::rank::Rank;

#[derive(Debug, PartialEq, Eq)]
pub enum Score {
    HighCard(Rank),
    Pair(Rank),
    TwoPair(Rank, Rank),
    ThreeOfAKind(Rank),
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}
