use cards_sets::{prelude::*};


#[test]
fn fix_mix_set_new(){
    let mut card_vec:Vec<Card> = vec![];
    card_vec.push(Card::new(Rank::Ace,Suit::Clubs));
    card_vec.push(Card::new(Rank::King,Suit::Clubs));
    card_vec.push(Card::new(Rank::Queen,Suit::Clubs));
    card_vec.push(Card::new(Rank::Jack,Suit::Clubs));
    card_vec.push(Card::new(Rank::Ten,Suit::Clubs));
    
    println!("{:?}",card_vec[0].short_string());
    
    let flat_set = FlatSet::new_from_card_vec(&card_vec);
    
    let flat_vec_card = flat_set.flat_to_vec_cards();
    println!("{:?}",flat_vec_card);

    println!("flat_set: {}", flat_set);
    let fix_mix = FixMix::new_from_flat(flat_set.flat_bin);
    println!("fix_mix {}",fix_mix);
    // println!("flat_set: {}", flat_set);

}