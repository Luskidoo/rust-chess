pub mod data;
mod board;
mod validate;

use crate::data::*;
use crate::board::*;
use ndarray::*;
use once_cell::sync::Lazy;

pub fn main() {
    
    let undo = Undo {
        m: 0,
        castlePerm: 0,
        enPas: NO_SQ,
    };

    let pos = Board {
        pieces: [OFFBOARD; BRD_SQ_NUM],
        pawns: [0; 3],
        side: BOTH,
        enPas: NO_SQ,
        ply: 0,
        hisPly: 0,
        castlePerm: 0,
        pceNum: [0; 13],
        bigPce: [0; 2],
        majPce: [0; 2],
        minPce: [0; 2],
        history: [undo; MAX_GAME_MOVES],
        pList: Lazy::new(|| Array2::<i32>::zeros((13, 10))),
    };
    let mut list = board::MoveList {
        moves: [board::Move {m: 0, score: 0}; data::MAX_POSITION_MOVES],
        count: 0
    };
    unsafe {
        board::init_hash();
        board::init_board();
        //board::init_sq120_to_sq64();
        //board::init_files_ranks_board();
        print_board();
        board::gen(&mut list);
    }
}

unsafe fn print_board()
{
	let i: usize;
	
	print!("\n8");
	for i in 0..64 {
        print!(" {}", piece_char[board::piece[i] as usize]);
		if (i + 1) % 8 == 0 && i != 63 {
            print!("\n{0}", data::rank_index(i));
    }
    
	}
	print!("\n  a b c d e f g h\n\n");
}