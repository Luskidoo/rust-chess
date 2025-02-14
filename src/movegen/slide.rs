use crate::{defs::{Piece, Pieces, Side, Sides, Square}, BitBoard, BitMove, Board, MoveList};

use super::{MoveGenerator, SQ};

impl MoveGenerator {
    pub fn generate_rook_moves(&self, board: &Board, list: &mut MoveList) {
        let side = board.game_state.side_to_move as usize;
        let mut rooks = board.pieces[Pieces::ROOK][side];
        let occupancy = board.black_occupied() | board.white_occupied();
        //println!("{}", occupancy);
        while rooks > BitBoard(0) {
            let from = BitBoard::next(&mut rooks);
            let index = self.rook_magics[from.0].get_index(occupancy);
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
                println!("Adding rook move from {} to {}", from, to);
                self.add_move(&board, list, Pieces::ROOK, from.clone(), to)
                //println!("Rook move from {} to {}", from, to);
            }
        }
    }

    pub fn generate_bishop_moves(&self, board: &Board, list: &mut MoveList) {
        let side = board.game_state.side_to_move as usize;
        let mut bishops = board.pieces[Pieces::BISHOP][side];
        let occupancy = board.black_occupied() | board.white_occupied();
        while bishops > BitBoard(0) {
            let from = BitBoard::next(&mut bishops);
            let index = self.bishop_magics[from.0].get_index(occupancy);
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
                println!("Adding bishop move from {} to {}", from, to);
                self.add_move(&board, list, Pieces::BISHOP, from.clone(), to)
            }
        }
    }


    pub fn generate_queen_moves(&self, board: &Board, list: &mut MoveList) {
        let side = board.game_state.side_to_move as usize;
        let mut queens = board.pieces[Pieces::QUEEN][side];
        let occupancy = board.black_occupied() | board.white_occupied();
        while queens > BitBoard(0) {
            let from = BitBoard::next(&mut queens);
            // Diagonal moves
            let mut index = self.bishop_magics[from.0].get_index(occupancy);
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
                println!("Adding queen move from {} to {}", from, to);
                self.add_move(&board, list, Pieces::QUEEN, from.clone(), to);
            }
            // Straight moves
            index = self.rook_magics[from.0].get_index(occupancy);
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
                println!("Adding queen move from {} to {}", from, to);
                self.add_move(&board, list, Pieces::QUEEN, from.clone(), to);
            }
        }
    }

    pub fn get_slider_attacks(&self, piece: Piece, square: &Square, occupancy: BitBoard) -> BitBoard {
        match piece {
            Pieces::ROOK => {
                let index = self.rook_magics[square.0].get_index(occupancy);
                self.rook[index]
            }
            Pieces::BISHOP => {
                let index = self.bishop_magics[square.0].get_index(occupancy);
                self.bishop[index]
            }
            Pieces::QUEEN => {
                let r_index = self.rook_magics[square.0].get_index(occupancy);
                let b_index = self.bishop_magics[square.0].get_index(occupancy);
                self.rook[r_index] ^ self.bishop[b_index]
            }
            _ => panic!("Not a sliding piece: {piece}"),
        }
    }


}