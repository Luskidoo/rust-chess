mod data;
mod board;
use crate::data::*;
use crate::board::*;

fn main () {
    let mut pos = Board {
        pieces: test_pieces2,
    };
    let mut list = MoveList {
        moves: [Move {m: 0, score: 0}; MAX_POSITION_MOVES],
        count: 0, 
    };
    generate_moves(&mut pos, &mut list);
    print_board(pos);
}

fn print_board (pos: Board) {
    for i in 0..64 {
        if i % 8 == 0 {
            print!("\n")
        }
        print!("{} ", piece_char[pos.pieces[i] as usize]);
    };
}