use crate::{BitBoard, BitMove, Board, MoveList};

use super::{MoveGenerator, SQ};

impl MoveGenerator {
    pub fn generate_king_moves(&self, board: Board, list: &mut MoveList) {
        let side = board.game_state.side_to_move as usize;
        let mut kings = board.king[side];
        //println!("{}", occupancy);
        while kings > BitBoard(0) {
            let from = BitBoard::next(&mut kings);
            let mut to_bb = self.king_attacks[from as usize];
            // if white
            if side == 0 {
                to_bb &= !board.white_occupied()
            }
            else {
                to_bb &= !board.black_occupied()
            }           
            println!("{}", to_bb);
            while to_bb > BitBoard(0) {
            let to = BitBoard::next(&mut to_bb);
                list.push(BitMove::new(0, SQ(from as u8), SQ(to as u8)));
                println!("King move from {} to {}", from, to);
            }
        }
    }
}