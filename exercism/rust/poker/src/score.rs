use crate::rank::Rank;

pub type Kickers = Vec<Rank>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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
    use std::cmp::Ordering;

    use super::*;
    use test_case::test_case;

    fn high_card_seven() -> Score {
        Score::HighCard {
            kickers: vec![Rank::Seven, Rank::Six, Rank::Five, Rank::Three, Rank::Two],
        }
    }

    fn high_card_ten() -> Score {
        Score::HighCard {
            kickers: vec![Rank::Ten, Rank::Six, Rank::Five, Rank::Three, Rank::Two],
        }
    }

    fn high_card_ten_low() -> Score {
        Score::HighCard {
            kickers: vec![Rank::Ten, Rank::Five, Rank::Four, Rank::Three, Rank::Two],
        }
    }

    fn pair_fives_jack_high() -> Score {
        Score::Pair {
            rank: Rank::Five,
            kickers: vec![Rank::Jack, Rank::Eight, Rank::Seven],
        }
    }

    fn pair_fives_king_three_high() -> Score {
        Score::Pair {
            rank: Rank::Five,
            kickers: vec![Rank::King, Rank::Three, Rank::Two],
        }
    }

    fn pair_fives_king_seven_high() -> Score {
        Score::Pair {
            rank: Rank::Five,
            kickers: vec![Rank::King, Rank::Seven, Rank::Three],
        }
    }

    fn four_of_kind_king_seven_high() -> Score {
        Score::FourOfAKind {
            rank: Rank::King,
            kicker: Rank::Seven,
        }
    }

    fn four_of_a_kind_ten_seven_high() -> Score {
        Score::FourOfAKind {
            rank: Rank::Ten,
            kicker: Rank::Seven,
        }
    }

    fn four_of_a_kind_ten_jack_high() -> Score {
        Score::FourOfAKind {
            rank: Rank::Ten,
            kicker: Rank::Jack,
        }
    }

    #[test_case(high_card_seven(), high_card_ten())]
    #[test_case(high_card_ten_low(), high_card_ten())]
    #[test_case(high_card_ten(), pair_fives_jack_high())]
    #[test_case(pair_fives_jack_high(), pair_fives_king_three_high())]
    #[test_case(pair_fives_king_three_high(), pair_fives_king_seven_high())]
    #[test_case(Score::StraightFlush { rank: Rank::King }, Score::RoyalFlush)]
    #[test_case(four_of_a_kind_ten_seven_high(), four_of_kind_king_seven_high())]
    #[test_case(four_of_a_kind_ten_seven_high(), four_of_a_kind_ten_jack_high())]
    fn first_is_less_than_second(first: Score, second: Score) {
        assert!(first.cmp(&second) == Ordering::Less);
    }
}
