use cards_sets::prelude::*;

#[test]
fn it_card16_cardview(){

    let card1=Card::new(Rank::Two,Suit::Clubs);
    println!("{card1}");
    let card16_card = card_to_card16(&card1);
    println!("card16_card {card16_card}");
}

#[test]
fn it_card16_frm_rank_nmb(){
    let card16_card = rank_number_to_card16(12);
    println!("card16_card {card16_card}");
}