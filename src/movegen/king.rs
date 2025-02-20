use crate::{defs::{Pieces, Square}, BitBoard, BitMove, Board, MoveList};

use super::{MoveGenerator, SQ};

impl MoveGenerator {
    pub fn generate_king_moves(&self, board: &Board, list: &mut MoveList) {
        let side = board.game_state.side_to_move as usize;
        let mut kings = board.pieces[side][Pieces::KING];
        //println!("{}", occupancy);
        while kings.0 > 0 {
            let from = BitBoard::next(&mut kings);
            let mut to_bb = self.king_attacks[from.0];
            // if white
            if side == 0 {
                to_bb &= !board.white_occupied()
            }
            else {
                to_bb &= !board.black_occupied()
            }           
            //println!("{}", to_bb);
            while to_bb.0 > 0 {
            let to = BitBoard::next(&mut to_bb);
                //println!("Adding king move from {} to {}", from, to);
                self.add_move(&board, list, Pieces::KING, from.clone(), to.clone());
            }
        }
    }

    // Return king attacks for the given square.
    pub fn get_king_attacks(&self, square: &Square) -> BitBoard {
        self.king_attacks[square.0]
    }
}