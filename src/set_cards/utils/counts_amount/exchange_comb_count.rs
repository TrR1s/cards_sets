use crate::prelude::{from_flat_64_to_pyr_64_wo_flush, pyr_in_pyr_flush_free, FixMix, FixSuitable, FlatSet64, MixSet, PyramidSet64,  LOWFLOW};
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
#[derive(Debug)]
pub struct FlushInfo{
    pub comb_fl_is:bool,
    pub amount:i64,
    pub fl_flat: FlatSet64,
}

pub fn exchange_comb_count(fix_cards:&FixMix,deck:&MixSet,comb5_unfl_pyr:PyramidSet64) -> (i64,FlushInfo){
    
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
        return (0 as i64,FlushInfo{
                                    comb_fl_is:comb5_fl_exist,
                                    amount:0,
                                    fl_flat:0
                                                }
                                            );
    };

    //2

    let diff = comb5_unfl_pyr & !fix_cards.mix_set.pyr;
    let pyr_diff = from_flat_64_to_pyr_64_wo_flush(diff);

    //3

    let comb5_unfl_amount = pyr_in_pyr_flush_free(pyr_diff, deck.pyr);

    // 4
    if !comb5_fl_exist || fix_cards.suitable_info == FixSuitable::NOT{

        return (comb5_unfl_amount,FlushInfo{
                                    comb_fl_is:comb5_fl_exist,
                                    amount:0,
                                    fl_flat:0
                                                });
        
    }

    // 5
    let check_flush= |flat64:FlatSet64,pyr_fl_64:PyramidSet64, suit_n:u8| -> (i64,FlatSet64) {
        
        let flush_n_64 = pyr_fl_64 << 16*suit_n;
        if flush_n_64 == flush_n_64 & flat64 {
            return (1,flush_n_64);
        }
        (0,0) 
    };


    let (comb5_fl_amount,fl_flat)  = match  fix_cards.suitable_info {
        FixSuitable::NOT => (0,0),
        FixSuitable::YES(suit_n) => check_flush(deck.flat,pyr_diff,suit_n),
        FixSuitable::ANY => {
                let mut fl_amount: i64 = 0;
                let mut fl_flat: PyramidSet64 = 0;
                for suit_r in [0,1,2,3]{
                    let (curr_fl_amount, curr_fl_flat) = check_flush(deck.flat,pyr_diff,suit_r);
                    fl_amount += curr_fl_amount;
                    fl_flat |= curr_fl_flat;

                }
                (fl_amount,fl_flat)

    }
        
        
    };


    (comb5_unfl_amount-comb5_fl_amount,FlushInfo{
                                    comb_fl_is:comb5_fl_exist,
                                    amount:comb5_fl_amount,
                                    fl_flat:fl_flat})
    
    
}