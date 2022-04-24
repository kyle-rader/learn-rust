use crate::rank::Rank;

pub type Kickers = Vec<Rank>;

#[derive(Debug, PartialEq, Eq, PartialOrd)]
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

#[cfg(test)]
mod score_tests {
    use super::*;
    use test_case::test_case;

    fn high_card_seven() -> Score {
        Score::HighCard {
            kickers: vec![Rank::Two, Rank::Three, Rank::Five, Rank::Six, Rank::Seven],
        }
    }

    fn high_card_ten() -> Score {
        Score::HighCard {
            kickers: vec![Rank::Two, Rank::Three, Rank::Five, Rank::Six, Rank::Ten],
        }
    }

    fn high_card_ten_low() -> Score {
        Score::HighCard {
            kickers: vec![Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Ten],
        }
    }

    fn pair_fives_jack_high() -> Score {
        Score::Pair {
            rank: Rank::Five,
            kickers: vec![Rank::Two, Rank::Three, Rank::Jack],
        }
    }

    fn pair_fives_king_high() -> Score {
        Score::Pair {
            rank: Rank::Five,
            kickers: vec![Rank::Two, Rank::Three, Rank::King],
        }
    }

    #[test_case(high_card_seven(), high_card_ten())]
    #[test_case(high_card_ten_low(), high_card_ten())]
    #[test_case(high_card_ten(), pair_fives_jack_high())]
    #[test_case(pair_fives_jack_high(), pair_fives_king_high())]
    #[test_case(Score::StraightFlush { rank: Rank::King }, Score::RoyalFlush)]
    #[test_case(Score::FourOfAKind { rank: Rank::Ten, kicker: Rank::Five }, Score::FourOfAKind { rank: Rank::King, kicker: Rank::Seven })]
    fn first_is_less_than_second(first: Score, second: Score) {
        assert!(first < second)
    }
}
