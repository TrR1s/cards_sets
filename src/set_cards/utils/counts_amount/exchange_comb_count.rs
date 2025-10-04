use crate::prelude::{from_flat_64_to_pyr_64_wo_flush, pyr_in_pyr_flush_free, FixMix, FixSuitable, FlatSet64, MixSet, PyramidSet64, LOWFLOW};
///
/// # Arg
/// * fix_cards : FixMix, cards which plr left on the hand. 
/// can be 0 (empty, exchanging 5 cards)
/// * deck:MixSet rest of deck
/// * comb5_unfl_pyr:PyramidSet64 unflush comb. 
/// # Out
///  (i64,bool,i64)
///  let
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
    let comb5_fl_exist = LOWFLOW == comb5_unfl_pyr | LOWFLOW;

    if comb5_unfl_pyr != comb5_unfl_pyr | fix_cards.mix_set.pyr {
        return (0 as i64,comb5_fl_exist,0 as i64);
    };

    //2

    let diff = comb5_unfl_pyr & fix_cards.mix_set.pyr;
    let pyr_diff = from_flat_64_to_pyr_64_wo_flush(diff);

    //3

    let comb5_unfl_amount = pyr_in_pyr_flush_free(pyr_diff, deck.pyr);

    // 4
    if !comb5_fl_exist || fix_cards.suitable_info == FixSuitable::NOT{
        return (comb5_unfl_amount,comb5_fl_exist,0 as i64);
    }

    // 5
    let check_flush= |flat64:FlatSet64,pyr_fl_64:PyramidSet64, suit_n:u8| -> i64 {
        
        let flush_n_64 = pyr_fl_64 << 16*suit_n;
        if flush_n_64 == flush_n_64 & flat64 {
            return 1;
        }
        0 
    };

    let suist_nn: [u8; 4] = [0,1,2,3];

    let comb5_fl_amount  = match  fix_cards.suitable_info {
        FixSuitable::NOT => 0,
        FixSuitable::YES(suit_n) => check_flush(deck.flat,pyr_diff,suit_n),
        FixSuitable::ANY => suist_nn.iter().fold(0, |acc, x| acc + check_flush(deck.flat,pyr_diff,*x)),

        
    };


    (comb5_unfl_amount-comb5_fl_amount,comb5_fl_exist,comb5_fl_amount)
    
    
}