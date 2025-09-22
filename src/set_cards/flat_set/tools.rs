use crate::prelude::{Card};
use super::{FlatSet,FlatSet64};

impl FlatSet {



    pub fn new_from_card_vec(vec_c: &Vec<Card>) -> Self{
        let mut flat_bin: FlatSet64 = 0;

        for curr_card in vec_c{
            flat_bin |=Self::convert_card_analog_to_flat_bin(curr_card);
        }


        Self{flat_bin}
    }
    
    pub fn convert_card_analog_to_flat_bin(&card:&Card) -> FlatSet64{
        let mut flat_bin: FlatSet64 = 1;
        flat_bin <<= (card.suit.suit_number()*16)+card.rank.rank_number();
        flat_bin
    }
}