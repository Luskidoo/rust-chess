//use crate::macros::fr2sq;
use crate::data::*;

macro_rules! fr2sq {
    ($file:expr, $rank:expr) => {
        (21 + $file) + ($rank * 10)
    };
}



fn init_sq120_to_sq64() {
    let mut sq120_to_sq64: [i32; BRD_SQ_NUM] = [0; BRD_SQ_NUM];
    let mut sq64_to_sq120: [i32; 64] = [0; 64];
	let mut sq: i32 = 0;
    let mut sq64: i32 = 0;
    for i in 0..BRD_SQ_NUM {
		sq120_to_sq64[i] = 65;
	}

	for i in 0..64 {
		sq64_to_sq120[i] = 120;
	}

	for rank in RANK_1..RANK_8 {
		for file in FILE_A..FILE_H {
            sq = fr2sq!(file,rank);
            sq64_to_sq120[sq64 as usize] = sq;
            sq120_to_sq64[sq as usize] = sq64;
            sq64 += 1;
		}
	}
}

pub fn init_all() {
    init_sq120_to_sq64();
}