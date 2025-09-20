
#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Suit {
    pub fn short_string(&self) -> &'static str {
        match *self {
            Suit::Spades => "s",
            Suit::Hearts => "h",
            Suit::Diamonds => "d",
            Suit::Clubs => "c",
        }
    }

    pub fn suit_vec() -> Vec<Suit>{

        vec![Suit::Spades,Suit::Hearts,Suit::Diamonds,Suit::Clubs]
    }

    pub fn suit_number(&self) -> u8{

        match *self {
            Suit::Hearts => 0,
            Suit::Clubs => 1,
            Suit::Diamonds => 2,
            Suit::Spades => 3,
        }
    }
}