use crate::prelude::{FlatSet64,PyramidSet64,FLUSHMASK64};

pub fn from_flat_64_to_pyr_64(flat_64:FlatSet64) -> PyramidSet64{
let mut bin_freq:[u8;13]=[0;13];// amount cards by rank in the set
    let mut pyramid_cb: u64=0;
    let mut flat_copy = flat_64;
    while flat_copy != 0 {


        // for curr_rank in 0..13{
        //     if flat_copy & 1 == 1 {
        //         bin_freq[curr_rank] += 1; 
        //     }
        //     flat_copy >>=1
        // } 


        for rank_val in &mut bin_freq{
            if flat_copy & 1 == 1 {
                *rank_val += 1; 
            }
            flat_copy >>=1
        } 



        flat_copy >>= 3;
    }

    // for curr_rank in 0..13{
    //     if bin_freq[curr_rank] == 0 {
    //         continue;
    //     }
    //     let curr_rank_bin_mask: u64 = 1<<curr_rank;
    //     let mut curr_rank_bin: u64 = curr_rank_bin_mask;
    //     bin_freq[curr_rank] -=1;
    //     while bin_freq[curr_rank] != 0 {
    //         curr_rank_bin <<= 16;
    //         curr_rank_bin |= curr_rank_bin_mask;
    //         bin_freq[curr_rank] -= 1;
    //     }
    //     pyramid_cb |= curr_rank_bin;


    // }

    for (curr_rank,rank_val) in bin_freq.iter_mut().enumerate(){
        if *rank_val == 0 {
            continue;
        }
        let curr_rank_bin_mask: u64 = 1<<curr_rank;
        let mut curr_rank_bin: u64 = curr_rank_bin_mask;
        *rank_val -=1;
        while *rank_val != 0 {
            curr_rank_bin <<= 16;
            curr_rank_bin |= curr_rank_bin_mask;
            *rank_val -= 1;
        }
        pyramid_cb |= curr_rank_bin;


    }



    // check for flush
    let mut suit_mask: u64 = 0b0001_1111_1111_1111;
    let mut suit_is: bool = false;
    while suit_mask != 0  {
        if flat_64 == flat_64 & suit_mask{
            suit_is = true;
            break;
        }
        suit_mask <<=16;
    }


    if suit_is {
        pyramid_cb |= FLUSHMASK64;
    }

    pyramid_cb as PyramidSet64


}

