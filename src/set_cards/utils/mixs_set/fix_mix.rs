pub use crate::prelude::{MixSet,FlatSet64,flat_check_flush};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum FixSuitable {
    NOT,
    YES(u8)
    }

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
pub struct FixMix{
    mix_set:MixSet,
    suitable_info: FixSuitable,
}

impl FixMix {
    pub fn new_from_flat(flat:FlatSet64) -> Self{
        let mix_set = MixSet::new_from_flat(flat);
        // let suit_res = flat_check_flush(flat);
        let suitable_info = match flat_check_flush(flat) {
            Some(suit_n) => FixSuitable::YES(suit_n),
            None => FixSuitable::NOT,
            
        };

        FixMix{mix_set,suitable_info} 

    }
    
}
