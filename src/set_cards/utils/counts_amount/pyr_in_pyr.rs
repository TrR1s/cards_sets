use crate::prelude::{PyramidSet64,FLUSHMASK64};

// pub const FIRSTRANKSMASK: u64 = 0b0000_0000_0000_0001_0000_0000_0000_0001_0000_0000_0000_0001_0000_0000_0000_0001;
pub const FIRSTRANKSMASK: u64 = 0x0001000100010001;
pub const LOWFLOW: u64  = 0x1fff; // 0b0001_1111_1111_1111

fn c_n_m_by_rank_mask(diff_res:u64)-> i64{
    match diff_res{
        0 =>1, // (n frm n)
        0x0001000100010000 => 4, // 1110 (1 frm 4)
        0x0001000100000000 => 6, // 1100 (2 frm 4)
        0x0001000000000000 => 4, // 1000 (3 frm 4)
        0x0000000100010000 => 3, // 0110 (1 frm 3)
        0x0000000100000000 => 3, // 0100 (2 frm 3)
        0x0000000000010000 => 2, // 0010 (1 frm 2)
        _ => {
            println!("diff_res {diff_res}");
            panic!("wrong diff_res")},
    }
}

pub fn pyr_in_pyr_flush_free(small_pyr:PyramidSet64, big_pyr: PyramidSet64) -> i64{
    let mut small_pyr_fl_free = !FLUSHMASK64 & small_pyr;
    let mut big_pyr_fl_free = !FLUSHMASK64 & big_pyr;
    if big_pyr_fl_free != big_pyr_fl_free | small_pyr_fl_free { return  0;}
    let mut res_am: i64 = 1;
    let mut low_flow_small = small_pyr_fl_free & LOWFLOW;
    while  low_flow_small != 0 {
        
        if 1 == small_pyr_fl_free & 1 {
               let diff = !small_pyr_fl_free & big_pyr_fl_free & FIRSTRANKSMASK;
               res_am *= c_n_m_by_rank_mask(diff);

            }
        small_pyr_fl_free >>=1;
        big_pyr_fl_free >>=1;
        low_flow_small >>=1;

        
    }

    
    res_am
}
