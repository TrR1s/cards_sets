use super::super::card_analog::{Card, Rank, Suit};
use super::CardDig16;
///
/// Do  CardDig16
/// from Card
pub fn card_to_card16(card: &Card) -> CardDig16{
    let rank : u16  = match card.rank {
        Rank::Two => 0,
        Rank::Three => 1,
        Rank::Four => 2,
        Rank::Five => 3,
        Rank::Six => 4,
        Rank::Seven => 5,
        Rank::Eight => 6,
        Rank::Nine => 7,
        Rank::Ten => 8,
        Rank::Jack => 9,
        Rank::Queen => 10,
        Rank::King => 11,
        Rank::Ace => 12,
    };
    let bits : u16 = 1 << rank;
    bits
}


pub fn rank_number_to_card16(rank_number:u8) -> CardDig16{
    match rank_number {
        0..13 => {
                    let bits : u16 = 1 << rank_number;
                    return bits as CardDig16;
                     },
        _ => panic!("Unexpected rank value! '{}'", rank_number)
    }
    

}




pub fn card16_to_rank_nmb(card16:CardDig16) -> u8{
    match card16 {
        0b0000_0000_0000_0001 => 0,    
        0b0000_0000_0000_0010 => 1,    
        0b0000_0000_0000_0100 => 2,    
        0b0000_0000_0000_1000 => 3,    
        0b0000_0000_0001_0000 => 4,    
        0b0000_0000_0010_0000 => 5,    
        0b0000_0000_0100_0000 => 6,    
        0b0000_0000_1000_0000 => 7,    
        0b0000_0001_0000_0000 => 8,    
        0b0000_0010_0000_0000 => 9,    
        0b0000_0100_0000_0000 => 10,    
        0b0000_1000_0000_0000 => 11,    
        0b0001_0000_0000_0000 => 12,    
       
        _ => panic!("Unexpected card16 value! '{}'", card16)
    }
    

}