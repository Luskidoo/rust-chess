use crate::{defs::{Side, Sides}, BitBoard, BitMove, Board, MoveList};

use super::{MoveGenerator, SQ};

impl MoveGenerator {
    pub fn generate_rook_moves(&self, board: Board, list: &mut MoveList) {
        let side = board.game_state.side_to_move as usize;
        let mut rooks = board.rooks[side];
        let occupancy = board.black_occupied() | board.white_occupied();
        //println!("{}", occupancy);
        while rooks > BitBoard(0) {
            let from = BitBoard::next(&mut rooks);
            let index = self.rook_magics[from as usize].get_index(occupancy);
            let mut to_bb = self.rook[index];
            // if white
            if side == 0 {
                to_bb &= !board.white_occupied()
            }
            else {
                to_bb &= !board.black_occupied()
            }
            
            //println!("{}", to_bb);
            while to_bb > BitBoard(0) {
            let to = BitBoard::next(&mut to_bb);
                list.push(BitMove::new(0, SQ(from as u8), SQ(to as u8)));
                //println!("Rook move from {} to {}", from, to);
            }
        }
    }

    pub fn generate_bishop_moves(&self, board: Board, list: &mut MoveList) {
        let side = board.game_state.side_to_move as usize;
        let mut bishops = board.bishops[side];
        let occupancy = board.black_occupied() | board.white_occupied();
        while bishops > BitBoard(0) {
            let from = BitBoard::next(&mut bishops);
            let index = self.rook_magics[from as usize].get_index(occupancy);
            let mut to_bb = self.rook[index];
            // if white
            if side == 0 {
                to_bb &= !board.white_occupied()
            }
            else {
                to_bb &= !board.black_occupied()
            }
            while to_bb > BitBoard(0) {
            let to = BitBoard::next(&mut to_bb);
                list.push(BitMove::new(0, SQ(from as u8), SQ(to as u8)));
            }
        }
    }


    pub fn generate_queen_moves(&self, board: Board, list: &mut MoveList) {
        let side = board.game_state.side_to_move as usize;
        let mut queens = board.queens[side];
        let occupancy = board.black_occupied() | board.white_occupied();
        while queens > BitBoard(0) {
            let from = BitBoard::next(&mut queens);
            // Diagonal moves
            let mut index = self.bishop_magics[from as usize].get_index(occupancy);
            let mut to_bb = self.bishop[index];
            // if white
            if side == 0 {
                to_bb &= !board.white_occupied()
            }
            else {
                to_bb &= !board.black_occupied()
            }
            while to_bb > BitBoard(0) {
                let to = BitBoard::next(&mut to_bb);
                    list.push(BitMove::new(0, SQ(from as u8), SQ(to as u8)));
                }
            // Straight moves
            index = self.rook_magics[from as usize].get_index(occupancy);
            to_bb = self.rook[index];
            // if white
            if side == 0 {
                to_bb &= !board.white_occupied()
            }
            else {
                to_bb &= !board.black_occupied()
            }
            while to_bb > BitBoard(0) {
                let to = BitBoard::next(&mut to_bb);
                    list.push(BitMove::new(0, SQ(from as u8), SQ(to as u8)));
                }
        }
    }

}