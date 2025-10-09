use crate::prelude::{from_flat_64_to_pyr_64_wo_flush, pyr_in_pyr_flush_free, FlushInfo,FixMix, FixSuitable, FlatSet64, MixSet, PyramidSet64,  VirtCards,LOWFLOW};
///
/// ### Count amount of dlr comb
/// count for unflush comb5, also give figures about fl_comb5
/// 
/// * dlr_fix_cards
/// * New struct DeckInfo
/// ** deck MixSet
/// ** virt_cards
/// ** deck_pyr_after
/// 
/// * comb5_unfl_pyr
/// 
#[derive(Debug)]
pub struct DeckInfo{
    deck_before:MixSet,
    virt_cards:VirtCards,
    pyr_after:PyramidSet64

}

pub fn dlr_comb_count(fix_cards:&FixMix,deck_info:&DeckInfo,comb5_unfl_pyr:PyramidSet64)->(f64,FlushInfo,VirtCards ) {
/*

*/
    let comb5_fl_exist = LOWFLOW == comb5_unfl_pyr | LOWFLOW;

    //  no fix cards in comb5
    // return zeroes
    if comb5_unfl_pyr != comb5_unfl_pyr | fix_cards.mix_set.pyr {
        return (0. as f64,FlushInfo{
                                    comb_fl_is:comb5_fl_exist,
                                    amount:0,
                                    fl_flat:0
                                                }
                                                , VirtCards{virt_pyr:0,virt_flush_except_flat:0,fl_except_amount:0}
                                            );
    };


    //2
    //fix cards in comb5

    let diff = comb5_unfl_pyr & !fix_cards.mix_set.pyr;
    let pyr_diff = from_flat_64_to_pyr_64_wo_flush(diff);

    //3
    // count total amount comb5 minus ex ???? стоит ли вычитать?
    let comb5_unfl_amount = pyr_in_pyr_flush_free(pyr_diff, deck_info.pyr_after) - deck_info.virt_cards.fl_except_amount ;

    // 4
    // если флеш не возможен. 
    // либо с данными фиксировными картами.
    // либо у comb5 нет comb5_fl
    if !comb5_fl_exist || fix_cards.suitable_info == FixSuitable::NOT{

        return (comb5_unfl_amount as f64 ,FlushInfo{
                                    comb_fl_is:comb5_fl_exist,
                                    amount:0,
                                    fl_flat:0
                                                }, VirtCards{virt_pyr:pyr_diff,virt_flush_except_flat:0,fl_except_amount:0});
        
    }







    return (0.,FlushInfo{
                                    comb_fl_is:false,
                                    amount:0,
                                    fl_flat:0
                                                }
                                                , VirtCards{virt_pyr:0,virt_flush_except_flat:0,fl_except_amount:0}
                                            );
}


