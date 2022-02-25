#[derive(Debug, PartialEq, Eq)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

impl From<char> for Suit {
    fn from(c: char) -> Self {
        match c {
            'H' | 'h' => Suit::Heart,
            'D' | 'd' => Suit::Diamond,
            'C' | 'c' => Suit::Club,
            'S' | 's' => Suit::Spade,
            _ => panic!("{c} is an unknown suit!"),
        }
    }
}

#[cfg(test)]
mod suit_tests {
    use super::*;

    #[test]
    fn suit_from_char() {
        assert_eq!(Suit::from('C'), Suit::Club);
        assert_eq!(Suit::from('H'), Suit::Heart);
        assert_eq!(Suit::from('D'), Suit::Diamond);
        assert_eq!(Suit::from('S'), Suit::Spade);
        assert_eq!(Suit::from('c'), Suit::Club);
        assert_eq!(Suit::from('h'), Suit::Heart);
        assert_eq!(Suit::from('d'), Suit::Diamond);
        assert_eq!(Suit::from('s'), Suit::Spade);
    }

    #[test]
    #[should_panic]
    fn suit_panics() {
        let _ = Suit::from('a');
    }
}