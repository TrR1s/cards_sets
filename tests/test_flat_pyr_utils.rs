use cards_sets::prelude::*;

#[test]
fn flat_to_pyr(){
    let mut card_vec:Vec<Card> = vec![];
    card_vec.push(Card::new(Rank::Ace,Suit::Hearts));
    card_vec.push(Card::new(Rank::King,Suit::Diamonds));
    card_vec.push(Card::new(Rank::Queen,Suit::Hearts));
    card_vec.push(Card::new(Rank::Jack,Suit::Hearts));
    card_vec.push(Card::new(Rank::Jack,Suit::Diamonds));
    card_vec.push(Card::new(Rank::Two,Suit::Diamonds));
    card_vec.push(Card::new(Rank::Ten,Suit::Spades));
    card_vec.push(Card::new(Rank::Ten,Suit::Clubs));
    card_vec.push(Card::new(Rank::Ten,Suit::Hearts));

    let flat_set = FlatSet::new_from_card_vec(&card_vec);
    print!("flat_set: {}", flat_set);

    let pyr_64 = from_flat_64_to_pyr_64(flat_set.flat_bin);
    let pyr_set = PyrSet{pyr_bin:pyr_64};
    print!("pyr: {}", pyr_set);


}