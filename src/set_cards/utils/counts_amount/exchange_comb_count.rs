use crate::prelude::{FixMix,MixSet,PyramidSet64,FLUSHMASK64,pyr_in_pyr_flush_free,LOWFLOW,from_flat_64_to_pyr_64_wo_flush};
///
/// # Arg
/// * fix_cards : FixMix, cards which plr left on the hand. 
/// can be 0 (empty, exchanging 5 cards)
/// * deck:MixSet rest of deck
/// * comb5_unfl_pyr:PyramidSet64 unflush comb. 
/// # Out
///  (i64,bool,i64)
///  (comb5_unfl_amount,comb5_fl_exist,comb5_fl_amount )
/// 
/// fn count amount of possible comb5_unfl_pyr with fix card,
/// with deck
/// 
/// 
/// 
/// 
/// 
pub fn exchange_comb_count(fix_cards:&FixMix,deck:&MixSet,comb5_unfl_pyr:PyramidSet64) -> (i64,bool,i64){
    
    /*
    1. проверяем есть ли fix_cards в comb5_unfl_pyr, если нет, то проверяшь comb5_fl_exist и возращаем 0 0
    2. вычитаем fix_cards из comb5_unfl_pyr
    3. считаем pyr_in_pyr - разницу fix_cards из comb5_unfl_pyr в deck
    4. проверяем возможен ли флеш.
    5. считаем кол-во флешей, если да.
    
     */
    
    // 1

    let mut comb5_unfl_amount: i64 =0;
    let mut comb5_fl_amount: i64 =0;
    let comb5_fl_exist = LOWFLOW == comb5_unfl_pyr | LOWFLOW;

    if comb5_unfl_pyr != comb5_unfl_pyr | fix_cards.mix_set.pyr {
        return (0 as i64,comb5_fl_exist,0 as i64);
    };

    //2

    let diff = comb5_unfl_pyr & fix_cards.mix_set.pyr;
    let pyr_diff = from_flat_64_to_pyr_64_wo_flush(diff);

    //3

    let rest_am = pyr_in_pyr_flush_free(pyr_diff, deck.pyr);

    if !comb5_fl_exist{
        return (rest_am,comb5_fl_exist,0 as i64);
    }


    todo!("count flush amount");
    
    
}