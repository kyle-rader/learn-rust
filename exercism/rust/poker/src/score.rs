use crate::rank::Rank;

pub type Kickers = Vec<Rank>;

#[derive(Debug, PartialEq, Eq)]
pub enum Score {
    HighCard { kickers: Kickers },
    Pair { rank: Rank, kickers: Kickers },
    TwoPair { high: Rank, low: Rank, kicker: Rank },
    ThreeOfAKind { rank: Rank, kickers: Kickers },
    Straight { rank: Rank },
    Flush { kickers: Kickers },
    FullHouse { trio: Rank, pair: Rank },
    FourOfAKind { rank: Rank, kicker: Rank },
    StraightFlush { rank: Rank },
    RoyalFlush,
}
