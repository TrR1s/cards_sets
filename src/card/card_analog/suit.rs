
#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Suit {
    Hearts,
    Clubs,
    Diamonds,
    Spades,
}

impl Suit {
    pub fn short_string(&self) -> &'static str {
        match *self {
            Suit::Hearts => "h",
            Suit::Clubs => "c",
            Suit::Diamonds => "d",
            Suit::Spades => "s",
        }
    }

    pub fn suit_vec() -> Vec<Suit>{

        vec![Suit::Hearts,Suit::Clubs,Suit::Diamonds,Suit::Spades]
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