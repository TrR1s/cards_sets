use crate::prelude::{Card,Rank,Suit};
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

    pub fn flat_to_vec_cards(&self)->Vec<Card>{
        let mut card_vec:Vec<Card> = vec![];
        let mut flat_bin = self.flat_bin;
        let mut curr_pos = 0;
        while flat_bin !=0 {
            if 1== flat_bin & 1 {
                let suit_i :usize= curr_pos/16;
                let rank_i:usize = curr_pos % 16;
                let card = Card::new(Rank::rank_vec()[rank_i], Suit::suit_vec()[suit_i]);
                card_vec.push(card);
            }
            flat_bin >>=1;
            curr_pos +=1;
        }
        card_vec
    }
}