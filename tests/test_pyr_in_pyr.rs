use cards_sets::prelude::*;

#[test]
fn pyr_in_pyr_test(){

    let mut pyr_big_vec:Vec<CardDig16> = vec![];
    pyr_big_vec.push(rank_number_to_card16(1));
    pyr_big_vec.push(rank_number_to_card16(1));
    pyr_big_vec.push(rank_number_to_card16(2));
    pyr_big_vec.push(rank_number_to_card16(12));
    pyr_big_vec.push(rank_number_to_card16(12));
    pyr_big_vec.push(rank_number_to_card16(8));
    pyr_big_vec.push(rank_number_to_card16(8));
    pyr_big_vec.push(rank_number_to_card16(8));
    pyr_big_vec.push(rank_number_to_card16(8));
    pyr_big_vec.push(rank_number_to_card16(12));
    pyr_big_vec.push(rank_number_to_card16(7));
    pyr_big_vec.push(rank_number_to_card16(7));
    let suit_is = true;
    let pyr_big = PyrSet::new_from_vec(&pyr_big_vec, suit_is);
    println!("pyr_comb {}",pyr_big);


    let mut pyr_small_vec:Vec<CardDig16> = vec![];
    pyr_small_vec.push(rank_number_to_card16(1));
    // pyr_small_vec.push(rank_number_to_card16(1));
    // pyr_small_vec.push(rank_number_to_card16(2));
    pyr_small_vec.push(rank_number_to_card16(12));
    pyr_small_vec.push(rank_number_to_card16(12));
    // pyr_small_vec.push(rank_number_to_card16(8));
    pyr_small_vec.push(rank_number_to_card16(8));
    pyr_small_vec.push(rank_number_to_card16(8));
    // pyr_small_vec.push(rank_number_to_card16(8));
    pyr_small_vec.push(rank_number_to_card16(12));
    // pyr_small_vec.push(rank_number_to_card16(7));
    pyr_small_vec.push(rank_number_to_card16(7));
    let suit_is = false;
    let pyr_small = PyrSet::new_from_vec(&pyr_small_vec, suit_is);
    println!("pyr_small {}",pyr_small);

    let amount= pyr_in_pyr_flush_free(pyr_small.pyr_bin,pyr_big.pyr_bin);
    println!("amount {}",amount );





}