use std::{fmt}; 
pub use crate::prelude::{MixSet,FlatSet64,flat_check_flush};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum FixSuitable {
    NOT,
    YES(u8),
    ANY
    }

impl fmt::Display for FixSuitable {
    fn fmt(&self, f:&mut fmt::Formatter)->fmt::Result{
        let suit_show = match  self {
            FixSuitable::YES(suit_n) => format!("Cards suitable, suit n: {suit_n}"),
            FixSuitable::NOT => format!("Cards unsuitable"),
            FixSuitable::ANY => format!("ANY(no fix cards)"),

            
        };
        
        write!(f,"\n{suit_show}")
    }    
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
pub struct FixMix{
    pub mix_set:MixSet,
    pub suitable_info: FixSuitable,
}

impl FixMix {
    pub fn new_from_flat(flat:FlatSet64) -> Self{
        let mix_set = MixSet::new_from_flat(flat);
        // let suit_res = flat_check_flush(flat);
        let suitable_info = match flat_check_flush(flat) {
            Some(suit_n) => { if suit_n == 100 
                                        {FixSuitable::ANY} 
                                    
                                    else {
                                        
                                        FixSuitable::YES(suit_n)}
            }

            None => FixSuitable::NOT,
            
        };

        FixMix{mix_set,suitable_info} 

    }
    
}

impl fmt::Display for FixMix {
    fn fmt(&self, f:&mut fmt::Formatter)->fmt::Result{
        let mix_show = format!("{}",self.mix_set);
        
        write!(f,"\nFix_mix: \n{} {}",self.suitable_info,mix_show)
    }    
}
