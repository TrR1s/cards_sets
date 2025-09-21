use  crate::card::card_analog::{Rank};
use super::PyramidSet;
use crate::set_cards::FLUSHMASK64;

#[derive(Debug)]
pub struct  PyrVew{
    flush_is: bool,
    pyr_arr: [[Option<Rank>;13];4]

}

impl PyrVew {
    pub fn do_frm_pyrset(&pyr_s: &PyramidSet) ->PyrVew{
        let mut pyr_arr: [[Option<Rank>;13];4] = [[None;13];4];
        let flush_is = (FLUSHMASK64 & pyr_s)== FLUSHMASK64;

        let mut pyr_flush_free = !FLUSHMASK64 & pyr_s;
        let mut curr_pos = 0;
        // let rank_vec= Rank::rank_vec();
        while pyr_flush_free !=0 {
            // println!("{curr_pos}");
            if 1 == pyr_flush_free & 1 {
                let  x :usize= curr_pos/16 ;
                let y:usize = curr_pos % 16;
                println!("{x}, {y} ");
                pyr_arr[x][y] = Some(Rank::rank_vec()[y]);
            }
            pyr_flush_free >>=1;
            curr_pos +=1;
            
        }


        PyrVew{flush_is,pyr_arr}


        
        

    }
    
}