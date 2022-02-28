use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

static ERR_NOT_A_RANK: &str = "not a Rank!";

impl FromStr for Rank {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Rank::Ace),
            "2" => Ok(Rank::Two),
            "3" => Ok(Rank::Three),
            "4" => Ok(Rank::Four),
            "5" => Ok(Rank::Five),
            "6" => Ok(Rank::Six),
            "7" => Ok(Rank::Seven),
            "8" => Ok(Rank::Eight),
            "9" => Ok(Rank::Nine),
            "10" => Ok(Rank::Ten),
            "J" => Ok(Rank::Jack),
            "Q" => Ok(Rank::Queen),
            "K" => Ok(Rank::King),
            _ => Err(String::from(ERR_NOT_A_RANK)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case("A", Rank::Ace ; "Ace")]
    #[test_case("2", Rank::Two ; "Two")]
    #[test_case("3", Rank::Three ; "Three")]
    #[test_case("4", Rank::Four ; "Four")]
    #[test_case("5", Rank::Five ; "Five")]
    #[test_case("6", Rank::Six ; "Six")]
    #[test_case("7", Rank::Seven ; "Seven")]
    #[test_case("8", Rank::Eight ; "Eight")]
    #[test_case("9", Rank::Nine ; "Nine")]
    #[test_case("10", Rank::Ten ; "Ten")]
    #[test_case("J", Rank::Jack ; "Jack")]
    #[test_case("Q", Rank::Queen ; "Queen")]
    #[test_case("K", Rank::King ; "King")]
    fn from_str_ace(s: &str, r: Rank) {
        assert_eq!(s.parse::<Rank>(), Ok(r));
    }

    #[test]
    fn from_str_not_a_rank() {
        assert_eq!("42".parse::<Rank>(), Err(String::from(ERR_NOT_A_RANK)));
    }

    #[test]
    fn rank_is_sortable() {
        let expected = vec![
            Rank::Ace,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
        ];

        let mut subject = vec![
            Rank::King,
            Rank::Queen,
            Rank::Jack,
            Rank::Ten,
            Rank::Nine,
            Rank::Eight,
            Rank::Seven,
            Rank::Six,
            Rank::Five,
            Rank::Four,
            Rank::Three,
            Rank::Two,
            Rank::Ace,
        ];

        subject.sort();

        assert_eq!(subject, expected);
    }
}