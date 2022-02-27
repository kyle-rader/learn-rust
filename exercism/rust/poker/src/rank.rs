use int_enum::IntEnum;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, IntEnum, Copy, Clone)]
enum Rank {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rank_from_int() {
        let jack = Rank::from_int(11);
        if let Ok(r) = jack {
            assert_eq!(r, Rank::Jack);
        }
    }

    #[test]
    #[should_panic]
    fn rank_from_not_a_rank() {
        Rank::from_int(0).unwrap();
    }

    #[test]
    fn rank_can_sort() {
        if let Ok(subject) = vec![1, 5, 13, 11, 8]
            .into_iter()
            .map(|n| Rank::from_int(n))
            .collect()
        {
            assert_eq!(subject, &[
                Rank::Ace,
                Rank::Five,
                Rank::Eight,
                Rank::Jack,
                Rank::King,
            ]);
        }
    }
}
