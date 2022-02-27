#[derive(Debug, PartialEq, Eq)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

impl TryFrom<char> for Suit {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'H' | 'h' => Ok(Suit::Heart),
            'D' | 'd' => Ok(Suit::Diamond),
            'C' | 'c' => Ok(Suit::Club),
            'S' | 's' => Ok(Suit::Spade),
            _ => Err(format!("'{c}' is not a suit!")),
        }
    }
}

#[cfg(test)]
mod suit_tests {
    use super::*;

    #[test]
    fn suit_try_from_ok() {
        assert_eq!(Suit::try_from('C'), Ok(Suit::Club));
        assert_eq!(Suit::try_from('H'), Ok(Suit::Heart));
        assert_eq!(Suit::try_from('D'), Ok(Suit::Diamond));
        assert_eq!(Suit::try_from('S'), Ok(Suit::Spade));
        assert_eq!(Suit::try_from('c'), Ok(Suit::Club));
        assert_eq!(Suit::try_from('h'), Ok(Suit::Heart));
        assert_eq!(Suit::try_from('d'), Ok(Suit::Diamond));
        assert_eq!(Suit::try_from('s'), Ok(Suit::Spade));
    }

    #[test]
    fn suit_try_from_err() {
        let subject = Suit::try_from('A');
        let expected = Err(String::from("'A' is not a suit!"));
        assert_eq!(subject, expected);
    }
}