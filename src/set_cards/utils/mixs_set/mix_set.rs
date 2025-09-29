use crate::prelude::{FlatSet64,PyramidSet64,from_flat_64_to_pyr_64_wo_flush};


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