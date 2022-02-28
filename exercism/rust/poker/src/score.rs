#[derive(Debug, PartialEq, Eq)]
pub enum Score {
    HighCard,
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

#[cfg(test)]
mod tests {
    use super::*;
}
