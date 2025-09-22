pub mod card;
pub mod set_cards;

pub mod prelude {
    pub use crate::card::card_dig::tools::{card_to_card16,rank_number_to_card16,card16_to_rank_nmb};
    pub use crate::card::card_dig::CardDig16;
    pub  use crate::card::card_analog::{Card,Suit,Rank};
    pub use crate::set_cards::{FLUSHMASK64};
    pub use  crate::set_cards::pyramid_set::{PyrSet};
    pub use  crate::set_cards::pyramid_set::views::{PyrVew};
    pub  use  crate::set_cards::flat_set::{FlatSet};
}
