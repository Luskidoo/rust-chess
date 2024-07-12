use super::{MoveGenerator, SQ};

use crate::{bitboard::*, defs::{Pieces, Side, Sides, Square}, BitMove, Board, MoveList};

impl MoveGenerator {
    fn w_pawn_single_push(bb: BitBoard, empty: &BitBoard) -> BitBoard {
        north_one(bb) & empty
    }
    
    fn w_pawn_double_push(bb: BitBoard, empty: &BitBoard) -> BitBoard {
        let single_pushes = Self::w_pawn_single_push(bb, empty);
        north_one(single_pushes) & empty & RANK4
    }
    
    fn b_pawn_single_push(bb: BitBoard, empty: &BitBoard) -> BitBoard {
        south_one(bb) & empty
    }
    
    fn b_pawn_double_push(bb: BitBoard, empty: &BitBoard) -> BitBoard {
        let single_pushes = Self::b_pawn_single_push(bb, empty);
        south_one(single_pushes) & empty & RANK5
    }

    pub fn generate_w_pawn_pushes(&self, board: &Board, list: &mut MoveList) {
        let mut w_pawns = board.pieces[Pieces::PAWN][Sides::WHITE];
        //println!("Initial pawns bitboard {:?}", w_pawns);
        let empty_bb: &BitBoard = &!board.occupancy(Sides::BOTH);
        while w_pawns > 0 {
            let from = next(&mut w_pawns);
            let from_bb: BitBoard = 1 << from.0;
            let mut to_bb: BitBoard = Self::w_pawn_single_push(from_bb, empty_bb) | Self::w_pawn_double_push(from_bb, empty_bb);
            while to_bb > 0 {
                let to = next(&mut to_bb);
                self.add_move(&board, list, Pieces::PAWN, from.clone(), to)
                //println!("Pawn move from {} to {}", from, to);
            }
        }
    }

    pub fn generate_w_pawn_attacks(&self, board: &Board, list: &mut MoveList) {
        let mut w_pawns = board.pieces[Pieces::PAWN][Sides::WHITE];
        while w_pawns > 0 {
            let from = next(&mut w_pawns);
            let mut to_bb: BitBoard = self.pawns[Sides::WHITE][from.0] & board.black_occupied();
            while to_bb > 0 {
                let to = next(&mut to_bb);
                self.add_move(&board, list, Pieces::PAWN, from.clone(), to)
                //println!("Pawn move from {} to {}", from, to);
            }

        }
    }
    
    pub fn generate_b_pawn_pushes(&self, board: &Board, list: &mut MoveList) {
        let mut b_pawns = board.pieces[Pieces::PAWN][Sides::BLACK];
        //println!("Initial pawns bitboard {:?}", w_pawns);
        let empty_bb: BitBoard = !board.occupancy(Sides::BOTH);
        while b_pawns > 0 {
            let from = next(&mut b_pawns);
            let from_bb: BitBoard = 1 << from.0;
            let mut to_bb: BitBoard = Self::w_pawn_single_push(from_bb, &empty_bb) | Self::w_pawn_double_push(from_bb, &empty_bb);
            while to_bb > 0 {
                let to = next(&mut to_bb);
                self.add_move(&board, list, Pieces::PAWN, from.clone(), to)
                //println!("Pawn move from {} to {}", from, to);
            }
        }
    }

    pub fn generate_b_pawn_attacks(&self, board: &Board, list: &mut MoveList) {
        let mut b_pawns = board.pieces[Pieces::PAWN][Sides::BLACK];
        while b_pawns > 0 {
            let from = next(&mut b_pawns);
            let mut to_bb: BitBoard = self.pawns[Sides::BLACK][from.0] & board.white_occupied();
            while to_bb > 0 {
                let to = next(&mut to_bb);
                self.add_move(&board, list, Pieces::PAWN, from.clone(), to)
                //println!("Pawn move from {} to {}", from, to);
            }

        }
    }

    pub fn generate_pawn_moves(&self, board: &Board, list: &mut MoveList) {
        self.generate_w_pawn_pushes(board, list);
        self.generate_b_pawn_pushes(board, list);
        self.generate_w_pawn_attacks(board, list);
        self.generate_b_pawn_attacks(board, list);
    }

    pub fn get_pawn_attacks_from_square(&self, side: Side, square: &Square) -> BitBoard {
        self.pawns[side][square.0]
    }
}

