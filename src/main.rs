pub mod data;
mod board;
mod validate;

use crate::data::*;
use crate::board::*;
use ndarray::*;
use once_cell::sync::Lazy;
use fen::*;

pub fn main() {
    
    let undo = Undo {
        m: 0,
        castlePerm: 0,
        enPas: NO_SQ,
    };

    let mut pos = Board::default();

    let mut list = board::MoveList {
        moves: [board::Move {m: 0, score: 0}; data::MAX_POSITION_MOVES],
        count: 0
    };
    unsafe {
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        let board = fen::BoardState::from_fen(fen).unwrap();
        let piece = board.pieces[0]

        match piece {
            // The division was valid
            Some(x) => println!("Result: {x}"),
            // The division was invalid
            None    => println!("Cannot divide by 0"),
        }
        //println!("{}", piece.Some(x))
        //board::init_hash();
        //board::init_board();
        //board::init_sq120_to_sq64();
        //board::init_files_ranks_board();
        //print_board();
        //board::generate_all_moves(&mut pos, &mut list);
    }
}

// unsafe fn print_board()
// {
// 	let i: usize;
	
// 	print!("\n8");
// 	for i in 0..64 {
//         print!(" {}", piece_char[board::piece[i] as usize]);
// 		if (i + 1) % 8 == 0 && i != 63 {
//             print!("\n{0}", data::rank_index(i));
//     }
    
// 	}
// 	print!("\n  a b c d e f g h\n\n");
// }