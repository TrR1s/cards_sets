use cards_sets::{prelude::*};


#[test]
fn mix_set_new(){
    let mut card_vec:Vec<Card> = vec![];
    card_vec.push(Card::new(Rank::Ace,Suit::Clubs));
    card_vec.push(Card::new(Rank::King,Suit::Diamonds));
    card_vec.push(Card::new(Rank::Queen,Suit::Hearts));
    card_vec.push(Card::new(Rank::Jack,Suit::Spades));
    card_vec.push(Card::new(Rank::Ten,Suit::Hearts));
    
    println!("{:?}",card_vec[0].short_string());
    
    let flat_set = FlatSet::new_from_card_vec(&card_vec);
    
    let flat_vec_card = flat_set.flat_to_vec_cards();
    println!("{:?}",flat_vec_card);

    println!("flat_set: {}", flat_set);
    let mix_s = MixSet::new_from_flat(flat_set.flat_bin);
    println!("mix_s {}",mix_s);
    println!("flat_set: {}", flat_set);

}