use crate::{defs::{Pieces, Square}, BitBoard, BitMove, Board, MoveList};

use super::{MoveGenerator, SQ};

impl MoveGenerator {
    pub fn generate_knight_moves(&self, board: &Board, list: &mut MoveList) {
        let side = board.game_state.side_to_move as usize;
        let mut knights = board.pieces[side][Pieces::KNIGHT];
        //println!("Initial bitboard {:?}", bb);
        while knights.0 > 0 {
            let from = BitBoard::next(&mut knights);
            //println!("From {}", from);
            //println!("From bb {:?}", from_bb);
            let mut to_bb: BitBoard = self.knight_moves_array[from.0];
            // if white
            if side == 0 {
                to_bb &= !board.white_occupied()
            }
            else {
                to_bb &= !board.black_occupied()
            } 
            //println!("To bb {:?}", to_bb);
            while to_bb.0 > 0 {
                let to = BitBoard::next(&mut to_bb);
                //println!("Adding knight move from {} to {}", from, to);
                self.add_move(board, list, Pieces::KNIGHT, from.clone(), to);
            }
        }
        //bb.knight_attacks() & w_empty 
    }

    // Return knight attacks for the given square.
    pub fn get_knight_attacks(&self, square: &Square) -> BitBoard {
        self.knight_moves_array[square.0]
    }
}