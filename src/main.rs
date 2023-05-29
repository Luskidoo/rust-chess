mod data;
mod board;
use crate::data::*;
use crate::board::*;

fn main () {
    let board = Board {
        pieces: init_pieces,
    };
    print_board(board);
}

fn print_board (pos: Board) {
    for i in 0..64 {
        if i % 8 == 0 {
            print!("\n")
        }
        print!("{} ", piece_char[pos.pieces[i] as usize]);
    };
}