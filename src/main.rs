pub mod data;
mod board;
mod validate;

use crate::data::*;
use crate::board::*;
use ndarray::*;
use once_cell::sync::Lazy;
use fen::*;

fn piece_string(piece: &Option<Piece>, sq: usize) -> String {
    match piece {
        Some(piece) => piece.to_string(),
        None    => String::from("."),
    }
}

fn piece_index(piece_string: String) -> usize {
    let index = piece_char.iter().position(|&x| x == piece_string);
    match index {
        Some(index) => index,
        None => 0,
    }

}

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
        for (sq, piece) in board.pieces.iter().enumerate() {
            println!("{}", piece_index(piece_string(piece, sq)))
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