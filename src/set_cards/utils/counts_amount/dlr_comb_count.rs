use crate::prelude::{from_flat_64_to_pyr_64_wo_flush, pyr_in_pyr_flush_free, FixMix, FixSuitable, FlatSet64, MixSet, PyramidSet64,  LOWFLOW};
///
/// ### Count amount of dlr comb
/// count for unflush comb5, also give figures about fl_comb5
/// 
/// * dlr_fix_cards
/// * New struct DeckInfo
/// ** deck_flat_before
/// ** deck_pyr_before
/// ** virtual_cards_pyr
/// ** virtual_flush_except_flat
/// ** deck_pyr_after
/// 
/// * comb5_unfl_pyr
/// 
#[derive(Debug)]
pub struct DeckInfo{
    deck_before:MixSet,
    virt_cards_pyr:PyramidSet64,
    virt_flush_except_flat:FlatSet64,
    deck_pyr_after:PyramidSet64,


}

