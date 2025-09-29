use crate::prelude::{FlatSet64,PyramidSet64,from_flat_64_to_pyr_64};
#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub struct MixSet{
    pub flat: FlatSet64,
    pub pyr: PyramidSet64,
}

impl MixSet {
    pub fn flat_to_pyr(&mut self) {
        self.pyr = from_flat_64_to_pyr_64(self.flat);

    }

    pub fn new_from_flat(flat:FlatSet64)->Self{
        let pyr = from_flat_64_to_pyr_64(flat);
        Self { flat, pyr }
    }

    
}