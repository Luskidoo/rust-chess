use crate::{defs::{Pieces, Square}, BitBoard, BitMove, Board, MoveList};

use super::{MoveGenerator, SQ};

impl MoveGenerator {
    pub fn w_knight_moves(&self, board: &Board, mut bb: BitBoard, w_empty: BitBoard, list: &mut MoveList) {
        //println!("Initial bitboard {:?}", bb);
        while bb > BitBoard(0) {
            let from = BitBoard::next(&mut bb);
            //println!("From {}", from);
            //println!("From bb {:?}", from_bb);
            let mut to_bb: BitBoard = self.knight_moves_array[from.0] & w_empty;
            //println!("To bb {:?}", to_bb);
            while to_bb > BitBoard(0) {
                let to = BitBoard::next(&mut to_bb);
                self.add_move(board, list, Pieces::KNIGHT, from.clone(), to)
                //println!("Knight move from {} to {}", from, to);
            }
        }
        //bb.knight_attacks() & w_empty 
    }

    // Return knight attacks for the given square.
    pub fn get_knight_attacks(&self, square: &Square) -> BitBoard {
        self.knight_moves_array[square.0]
    }
}