pub mod data;
mod board;
mod validate;

use crate::data::*;
use crate::board::*;
use ndarray::*;
use once_cell::sync::Lazy;
use fen::*;

fn piece_string(piece: &Option<Piece>) -> String {
    match piece {
        Some(piece) => piece.to_string(),
        None    => String::from("."),
    }
}

// fn get_piece_type(piece: &Option<Piece>) -> PieceKind {
//     match piece {
//         Some(piece) => piece.kind,
//         None        => piece.kind,
//     }
// }

fn piece_index(piece_string: String) -> usize {
    let index = piece_char.iter().position(|&x| x == piece_string.chars().next().unwrap());
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
    
    let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
    let board = fen::BoardState::from_fen(fen).unwrap();
    print_board(board);
        
        
    //println!("{}", piece.Some(x))
    //board::init_hash();
    //board::init_board();
    //board::init_sq120_to_sq64();
    //board::init_files_ranks_board();
    //print_board();
    //board::generate_all_moves(&mut pos, &mut list);
}

// fn print_board(board: BoardState)
// {

// 	print!("\n8");
// 	for (sq, piece) in board.pieces.iter().enumerate().rev() {
//         print!(" {}", sq);
// 		if (sq + 1) % 8 == 0 && sq != 63 {
//             print!("\n{0}", 8 - data::rank_index(sq));
//     }
    
// 	}
// 	print!("\n  a b c d e f g h\n\n");
// }

macro_rules! fr2sq {
    ($file:expr, $rank:expr) => {
        (21 + $file) + ($rank * 10)
    };
}

fn print_board(board: BoardState)
{
	//print!("\n8");
	for rank in (0..=7).rev() {
        println!("{} ", char::from_digit(rank + 1, 10).unwrap());
        for file in 0..=7 {
            print!("{} ", mailbox[fr2sq!(file, rank) as usize])
        }
		// if (sq + 1) % 8 == 0 && sq != 63 {
        //     print!("\n{0}", 8 - data::rank_index(sq));
        //}   
	}
	print!("\n  a b c d e f g h\n\n");
}