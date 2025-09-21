use crate::prelude::{CardDig16,card16_to_rank_nmb,FLUSHMASK64};
use super::PyrSet;

impl PyrSet{
    pub fn new_from_vec(vec_u:&Vec<CardDig16>, suit_is:bool) -> Self{
        let mut bin_freq:[u8;13]=[0;13];// amount cards by rank in the set
        let mut pyr_bin: u64=0;
        for curr_rank_nmb in vec_u{
            let rank_nmb = card16_to_rank_nmb(*curr_rank_nmb);
            bin_freq[rank_nmb as usize] +=1;
        }

        for (curr_rank,curr_freq) in bin_freq.iter_mut().enumerate(){
            if *curr_freq == 0 {
                continue;
            }
            let curr_rank_bin_mask: u64 = 1<<curr_rank;
            let mut curr_rank_bin: u64 = curr_rank_bin_mask;
            *curr_freq -=1;
            while *curr_freq != 0 {
                curr_rank_bin <<= 16;
                curr_rank_bin |= curr_rank_bin_mask;
                *curr_freq -= 1;
            }
            pyr_bin |= curr_rank_bin;
    
            
        }

        // suit check
        if suit_is {
            pyr_bin |= FLUSHMASK64;
        }
    

        Self {pyr_bin} 


    }


}