use cards_sets::prelude::*;


#[test]
fn card_to_flat(){
    let card1=Card::new(Rank::Ace,Suit::Hearts);
    println!("{card1}");
    let flat_bin = FlatSet::convert_card_analog_to_flat_bin(&card1);
    print!("flat_bin: {flat_bin}")
}

#[test]
fn vec_cards_to_flat(){
    let mut card_vec:Vec<Card> = vec![];
    card_vec.push(Card::new(Rank::Ace,Suit::Hearts));
    card_vec.push(Card::new(Rank::King,Suit::Diamonds));
    card_vec.push(Card::new(Rank::Queen,Suit::Hearts));
    card_vec.push(Card::new(Rank::Jack,Suit::Hearts));
    card_vec.push(Card::new(Rank::Ten,Suit::Hearts));

    let flat_set = FlatSet::new_from_card_vec(&card_vec);
    print!("flat_set: {:?}", flat_set)
}
