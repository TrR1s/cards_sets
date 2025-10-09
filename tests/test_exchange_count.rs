use cards_sets::{prelude::*};

pub const DECK: u64  = 0x1fff1fff1fff1fff; // 0b0001_1111_1111_1111
#[test]
fn echange_count_1(){

    let  flat_deck = DECK;
    let deck_mix = MixSet::new_from_flat(flat_deck);
    println!("deck_mix: {}", deck_mix);

    let mut fix_card_vec:Vec<Card> = vec![];
    fix_card_vec.push(Card::new(Rank::Ace,Suit::Clubs));
    fix_card_vec.push(Card::new(Rank::King,Suit::Clubs));
    // fix_card_vec.push(Card::new(Rank::Queen,Suit::Clubs));
    
    let flat_set = FlatSet::new_from_card_vec(&fix_card_vec);
    let fix_cards = FixMix::new_from_flat(flat_set.flat_bin);
    // let fix_cards = FixMix::new_from_flat(0);


    println!("fix_cards {}",fix_cards);
    let mut comb_vec:Vec<CardDig16> = vec![];
    comb_vec.push(rank_number_to_card16(Rank::Ace.rank_number()));
    comb_vec.push(rank_number_to_card16(Rank::King.rank_number()));
    comb_vec.push(rank_number_to_card16(Rank::Ten.rank_number()));
    comb_vec.push(rank_number_to_card16(Rank::Two.rank_number()));
    comb_vec.push(rank_number_to_card16(Rank::Three.rank_number()));
    let pyr_comb = PyrSet::new_from_vec(&comb_vec, false);
    println!("pyr_comb {}",pyr_comb);
    let (comb5_unfl_amount,comb5_fl_info, virt_cards)  = exchange_comb_count(&fix_cards,&deck_mix,pyr_comb.pyr_bin);
    println!("******************************");
    println!("comb5_unfl_amount {}",comb5_unfl_amount);
    println!("comb5_fl_exist {:?}",comb5_fl_info);
    println!("virt_cards {:?}",virt_cards);
    






    
}