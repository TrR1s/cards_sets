use std::{fmt}; 
// use crate::prelude::{};
use super::{FlatSet,};

impl fmt::Display for FlatSet {
    fn fmt(&self, f:&mut fmt::Formatter)->fmt::Result{
        
        // let mut one_suit_vec:Vec<String> = vec![String::from("__");13];
        let mut card_arr_str:Vec<Vec<String>> = vec![vec![String::from("__");13];4];

        // for i in 0..4{ card_arr_str[i][10]=String::from("___");}
        for vec_suit in card_arr_str.iter_mut(){vec_suit[10]=String::from("___");}

        let vec_card = self.flat_to_vec_cards();
        for curr_card in vec_card {
            // card_arr[curr_card.suit.suit_number()][curr_card.rank.rank_number()] = format!("{}",curr_card).to_owned();
            let val_str = curr_card.short_string();
            card_arr_str[curr_card.suit.suit_number() as usize][curr_card.rank.rank_number()as usize]= val_str; 
        }
        let mut res_str=String::from("\n");
        // for i in 0..4{
        //     let res_str_curr = card_arr_str[i].join(",");
        //     res_str.push_str(res_str_curr.as_str());
        //     res_str.push('\n');
        // }
        for vec_suit in card_arr_str.iter(){
            let res_str_curr = vec_suit.join(",");
            res_str.push_str(res_str_curr.as_str());
            res_str.push('\n');
        }

       write!(f,"{}",res_str) 
    }}