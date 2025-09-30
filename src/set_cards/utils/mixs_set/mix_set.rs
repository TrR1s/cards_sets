use std::{fmt}; 
use crate::prelude::{FlatSet64,PyramidSet64,from_flat_64_to_pyr_64_wo_flush,PyrSet,FlatSet};


#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub struct MixSet{
    pub flat: FlatSet64,
    pub pyr: PyramidSet64,
}

impl MixSet {
    pub fn flat_to_pyr(&mut self) {
        self.pyr = from_flat_64_to_pyr_64_wo_flush(self.flat);

    }

    pub fn new_from_flat(flat:FlatSet64)->Self{
        let pyr = from_flat_64_to_pyr_64_wo_flush(flat);
        Self { flat, pyr }
    }

    
}

impl fmt::Display for MixSet {
    fn fmt(&self, f:&mut fmt::Formatter)->fmt::Result{
        let pyr_show = format!("{}",PyrSet{  pyr_bin:self.pyr});
        let flat_show = format!("{}",FlatSet{  flat_bin:self.flat});

        write!(f,"\nPyr_mix: {pyr_show}\nFlat_mix:{flat_show}")
    }    
}