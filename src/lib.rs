pub mod card;

pub mod prelude {
    pub use crate::card::card_dig::tools::{card_to_card16,rank_number_to_card16,card16_to_rank_nmb};
    pub use crate::card::card_dig::CardDig16;
    pub  use crate::card::card_analog::{Card,Suit,Rank};
}