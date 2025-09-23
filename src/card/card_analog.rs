
use std::fmt::{self}; 
pub mod suit;
pub mod rank;
pub use self::suit::{Suit};
pub use self::rank::{Rank};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card{ rank, suit }
    }

    pub fn short_string(&self) -> String{
        format!("{}{}",self.rank.short_string(),self.suit.short_string())

        
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank.short_string(), self.suit.short_string())
    }
}
