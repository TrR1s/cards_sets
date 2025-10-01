use crate::prelude::{FixMix,MixSet,PyramidSet64,FLUSHMASK64};
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
    
    
    
    
    todo!("exchange_comb_count");
    
    
    (0,false,0)
}