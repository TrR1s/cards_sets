use cards_sets::prelude::*;



#[test]
fn it_pyr_frm_vec(){
    let mut sb16c_vec:Vec<CardDig16> = vec![];
    sb16c_vec.push(rank_number_to_card16(1));
    sb16c_vec.push(rank_number_to_card16(1));
    sb16c_vec.push(rank_number_to_card16(2));
    sb16c_vec.push(rank_number_to_card16(12));
    sb16c_vec.push(rank_number_to_card16(12));
    sb16c_vec.push(rank_number_to_card16(1));
    let suit_is = false;
    let pyr_comb = PyrSet::new_from_vec(&sb16c_vec, suit_is);
    println!("pyr_comb {:?}",pyr_comb);
}

#[test]
fn it_pyr_view(){
    let mut sb16c_vec:Vec<CardDig16> = vec![];
    sb16c_vec.push(rank_number_to_card16(1));
    sb16c_vec.push(rank_number_to_card16(1));
    sb16c_vec.push(rank_number_to_card16(2));
    sb16c_vec.push(rank_number_to_card16(12));
    sb16c_vec.push(rank_number_to_card16(12));
    sb16c_vec.push(rank_number_to_card16(8));
    sb16c_vec.push(rank_number_to_card16(8));
    sb16c_vec.push(rank_number_to_card16(12));
    sb16c_vec.push(rank_number_to_card16(7));
    let suit_is = true;
    let pyr_comb = PyrSet::new_from_vec(&sb16c_vec, suit_is);
    println!("pyr_comb {:?}",pyr_comb);
    let pyr_view = PyrVew::do_frm_pyrset(&pyr_comb.pyr_bin);
    println!("pyr_view {}",pyr_view);
    println!("pyr_comb {}",pyr_comb);
    

}