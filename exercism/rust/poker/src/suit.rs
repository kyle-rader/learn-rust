// use thiserror::Error;

// #[derive(Debug, PartialEq, Error)]
// pub enum SuitParsingError {
//     #[error("'{suit}' is not a suit.")]
//     InvalidSuit { suit: char },
// }
#[derive(Debug, PartialEq)]
pub enum SuitParsingError {
    InvalidSuit { suit: char },
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl TryFrom<char> for Suit {
    type Error = SuitParsingError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'H' | 'h' => Ok(Suit::Hearts),
            'D' | 'd' => Ok(Suit::Diamonds),
            'C' | 'c' => Ok(Suit::Clubs),
            'S' | 's' => Ok(Suit::Spades),
            _ => Err(SuitParsingError::InvalidSuit { suit: c }),
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
        let expected = Err(SuitParsingError::InvalidSuit { suit: 'A' });
        assert_eq!(subject, expected);
    }
}
