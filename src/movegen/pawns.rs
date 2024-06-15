use super::{MoveGenerator, SQ};

use crate::{bitboard::*, defs::Sides, BitMove, Board, MoveList};

impl MoveGenerator {
    fn w_pawn_single_push(bb: BitBoard, empty: BitBoard) -> BitBoard {
        bb.north_one() & empty
    }
    
    fn w_pawn_double_push(bb: BitBoard, empty: BitBoard) -> BitBoard {
        let single_pushes = Self::w_pawn_single_push(bb, empty);
        single_pushes.north_one() & empty & BitBoard::rank4
    }
    
    fn b_pawn_single_push(bb: BitBoard, empty: BitBoard) -> BitBoard {
        bb.south_one() & empty
    }
    
    fn b_pawn_double_push(bb: BitBoard, empty: BitBoard) -> BitBoard {
        let single_pushes = Self::b_pawn_single_push(bb, empty);
        single_pushes.south_one() & empty & BitBoard::rank5
    }

    pub fn generate_w_pawn_moves(board: Board, list: &mut MoveList) {
        let mut w_pawns = board.pawns[Sides::WHITE];
        //println!("Initial pawns bitboard {:?}", w_pawns);
        let empty_bb: BitBoard = board.empty_squares();
        while w_pawns > BitBoard(0) {
            let from = BitBoard::next(&mut w_pawns);
            let from_bb: BitBoard = BitBoard(1) << BitBoard(from);
            let mut to_bb: BitBoard = Self::w_pawn_single_push(from_bb, empty_bb) | Self::w_pawn_double_push(from_bb, empty_bb);
            while to_bb > BitBoard(0) {
                let to = BitBoard::next(&mut to_bb);
                list.push(BitMove::new(0, SQ(from as u8), SQ(to as u8)));
                //println!("Pawn move from {} to {}", from, to);
            }
        }
    }
    
    pub fn generate_b_pawn_moves(board: Board) {
        let mut b_pawns = board.pawns[Sides::WHITE];
        let empty_bb: BitBoard = board.empty_squares();
        while b_pawns > BitBoard(0) {
            let from = BitBoard::next(&mut b_pawns);
            let from_bb: BitBoard = BitBoard(1) << BitBoard(from);
            let mut to_bb: BitBoard = Self::b_pawn_single_push(from_bb, empty_bb) | Self::b_pawn_double_push(from_bb, empty_bb);
            while to_bb > BitBoard(0) {
                let to = BitBoard::next(&mut to_bb);
                //println!("Move from {} to {}", from, to);
            }
        }
    }
}

