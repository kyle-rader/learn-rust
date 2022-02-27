#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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

    #[test]
    fn score_can_sort() {
        let mut subject = vec![
            Score::Pair,
            Score::Flush,
            Score::RoyalFlush,
            Score::Straight
        ];

        subject.sort();
        assert_eq!(subject.iter().nth(3).unwrap(), &Score::RoyalFlush);
    }
}