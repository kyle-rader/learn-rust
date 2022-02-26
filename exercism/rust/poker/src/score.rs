#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Score {
    HighCard = 1,
    Pair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    Straight = 5,
    Flush = 6,
    FullHouse = 7,
    FourOfAKind = 8,
    StraightFlush = 9,
    RoyalFlush = 10,
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