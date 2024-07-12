use crate::{defs::{Pieces, Square}, bitboard::*, BitMove, Board, MoveList};

use super::{MoveGenerator, SQ};

impl MoveGenerator {
    pub fn generate_king_moves(&self, board: &Board, list: &mut MoveList) {
        let side = board.game_state.side_to_move as usize;
        let mut kings = board.pieces[Pieces::KING][side];
        //println!("{}", occupancy);
        while kings > 0 {
            let from = next(&mut kings);
            let mut to_bb = self.king_attacks[from.0];
            // if white
            if side == 0 {
                to_bb &= !board.white_occupied()
            }
            else {
                to_bb &= !board.black_occupied()
            }           
            //println!("{}", to_bb);
            while to_bb > 0 {
            let to = next(&mut to_bb);
                self.add_move(&board, list, Pieces::KING, from.clone(), to.clone());
                //println!("King move from {} to {}", from, to);
            }
        }
    }

    // Return knight attacks for the given square.
    pub fn get_king_attacks(&self, square: &Square) -> BitBoard {
        self.king_attacks[square.0]
    }
}