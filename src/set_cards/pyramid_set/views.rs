use std::{fmt}; 
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
                // println!("{x}, {y} ");
                pyr_arr[x][y] = Some(Rank::rank_vec()[y]);
            }
            pyr_flush_free >>=1;
            curr_pos +=1;
            
        }


        PyrVew{flush_is,pyr_arr}
    }
    
}

impl fmt::Display for PyrVew {
    fn fmt(&self, f:&mut fmt::Formatter)->fmt::Result{
        let flush_set:String = if self.flush_is {"One Suit".to_string()} else {"UnSuit".to_string()} ;
        

        let mut str_vec : Vec<String> = vec![];
        for curr_level in self.pyr_arr{
            let mut ind =2;
            let mut pyr_str = String::from("\n");
            for curr_el in curr_level{
                let curr_str = match curr_el {
                    Some(rank) => rank.short_string(),
                    None => if ind == 10{"__"} else {"_"}
                };
                ind +=1;

                pyr_str.push_str(curr_str);
                pyr_str.push('_');
            }
            str_vec.push(pyr_str);

        }
        let mut res_str = String::new();
        for i in (0..=3).rev(){
            res_str.push_str(str_vec.get(i).unwrap());
        }
        let res_str =  format!("***{flush_set}***, {res_str}");
        write!(f,"{res_str}")
    }
    
}