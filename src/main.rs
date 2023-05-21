pub mod data;
mod board;
mod validate;

use crate::data::*;


pub fn main() {
    let mut list = board::MoveList {
        moves: [board::Move {m: 0, score: 0}; data::MAX_POSITION_MOVES],
        count: 0
    };
    unsafe {
        board::init_hash();
        board::init_board();
        board::init_sq120_to_sq64();
        board::init_files_ranks_board();
        print_board();
        board::gen(&mut list);
    }
}

unsafe fn print_board()
{
	let i: usize;
	
	print!("\n1");
	for i in 0..64 {
        print!(" {}", piece_char[board::piece[i] as usize]);
		if (i + 1) % 8 == 0 && i != 63 {
            print!("\n{0}", 2 + data::ROW(i));
    }
    
	}
	print!("\n  a b c d e f g h\n\n");
}