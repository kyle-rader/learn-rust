// use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone)]
pub enum Rank {
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
    Ace,
}

// #[derive(Debug, PartialEq, Error)]
// pub enum RankParsingError {
//     #[error("'{rank}' is not a rank.")]
//     InvalidRank { rank: String },
// }
#[derive(Debug, PartialEq)]
pub enum RankParsingError {
    InvalidRank { rank: String },
}

impl TryFrom<&str> for Rank {
    type Error = RankParsingError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
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
            "A" => Ok(Rank::Ace),
            _ => Err(RankParsingError::InvalidRank {
                rank: String::from(s),
            }),
        }
    }
}

// #[cfg(test)]
// mod rank_tests {
//     use super::*;
//     use test_case::test_case;

//     #[test_case("A", Rank::Ace ; "Ace")]
//     #[test_case("2", Rank::Two ; "Two")]
//     #[test_case("3", Rank::Three ; "Three")]
//     #[test_case("4", Rank::Four ; "Four")]
//     #[test_case("5", Rank::Five ; "Five")]
//     #[test_case("6", Rank::Six ; "Six")]
//     #[test_case("7", Rank::Seven ; "Seven")]
//     #[test_case("8", Rank::Eight ; "Eight")]
//     #[test_case("9", Rank::Nine ; "Nine")]
//     #[test_case("10", Rank::Ten ; "Ten")]
//     #[test_case("J", Rank::Jack ; "Jack")]
//     #[test_case("Q", Rank::Queen ; "Queen")]
//     #[test_case("K", Rank::King ; "King")]
//     fn try_from_valid_rank(s: &str, r: Rank) {
//         assert_eq!(Rank::try_from(s), Ok(r));
//     }

//     #[test]
//     fn from_str_not_a_rank() {
//         assert_eq!(
//             Rank::try_from("42"),
//             Err(RankParsingError::InvalidRank {
//                 rank: String::from("42"),
//             })
//         );
//     }

//     #[test]
//     fn rank_is_sortable() {
//         let expected = vec![
//             Rank::Two,
//             Rank::Three,
//             Rank::Four,
//             Rank::Five,
//             Rank::Six,
//             Rank::Seven,
//             Rank::Eight,
//             Rank::Nine,
//             Rank::Ten,
//             Rank::Jack,
//             Rank::Queen,
//             Rank::King,
//             Rank::Ace,
//         ];

//         let mut subject = vec![
//             Rank::King,
//             Rank::Queen,
//             Rank::Jack,
//             Rank::Ten,
//             Rank::Nine,
//             Rank::Eight,
//             Rank::Seven,
//             Rank::Six,
//             Rank::Five,
//             Rank::Four,
//             Rank::Ace,
//             Rank::Three,
//             Rank::Two,
//         ];

//         subject.sort();

//         assert_eq!(subject, expected);
//     }

//     #[test]
//     fn rank_is_sortable_in_reverse() {
//         let expected = vec![Rank::King, Rank::Jack, Rank::Three];
//         let mut subject = vec![Rank::Jack, Rank::Three, Rank::King];
//         subject.sort_by(|a, b| b.cmp(a));
//         assert_eq!(expected, subject);
//     }

//     #[test_case(vec![Rank::Queen], vec![Rank::King])]
//     #[test_case(vec![Rank::King, Rank::Ten], vec![Rank::King, Rank::Jack])]
//     #[test_case(vec![Rank::King, Rank::Ten, Rank::Seven], vec![Rank::King, Rank::Ten, Rank::Eight])]
//     fn vec_of_rank_is_comparable(a: Vec<Rank>, b: Vec<Rank>) {
//         assert!(a < b);
//     }
// }
