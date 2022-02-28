#[derive(Debug, PartialEq, Eq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl TryFrom<char> for Suit {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'H' | 'h' => Ok(Suit::Hearts),
            'D' | 'd' => Ok(Suit::Diamonds),
            'C' | 'c' => Ok(Suit::Clubs),
            'S' | 's' => Ok(Suit::Spades),
            _ => Err(format!("'{c}' is not a suit!")),
        }
    }
}

#[cfg(test)]
mod suit_tests {
    use super::*;

    #[test]
    fn suit_try_from_ok() {
        assert_eq!(Suit::try_from('C'), Ok(Suit::Clubs));
        assert_eq!(Suit::try_from('H'), Ok(Suit::Hearts));
        assert_eq!(Suit::try_from('D'), Ok(Suit::Diamonds));
        assert_eq!(Suit::try_from('S'), Ok(Suit::Spades));
        assert_eq!(Suit::try_from('c'), Ok(Suit::Clubs));
        assert_eq!(Suit::try_from('h'), Ok(Suit::Hearts));
        assert_eq!(Suit::try_from('d'), Ok(Suit::Diamonds));
        assert_eq!(Suit::try_from('s'), Ok(Suit::Spades));
    }

    #[test]
    fn suit_try_from_err() {
        let subject = Suit::try_from('A');
        let expected = Err(String::from("'A' is not a suit!"));
        assert_eq!(subject, expected);
    }
}
