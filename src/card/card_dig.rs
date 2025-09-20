//! ### Simple card
//! 
//! Simple card now is
//! only u16 type named as CardDig16
//! 
//! 
//! where one 1 on rank position
//! 

///  + -------- + -------- +
/// 
///  | xxxbbbbb | bbbbbbbb |
/// 
///  b = bit turned on depending on rank of card 
/// 
///  xxx - later can be used for suit (001, 010, 011, 100)
/// 
///  # card16 - shot name
pub type CardDig16 = u16; 

pub mod tools;

pub use tools::{card_to_card16,rank_number_to_card16,card16_to_rank_nmb};

