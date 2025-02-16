use super::{MoveGenerator, SQ};

use crate::{bitboard::*, defs::{Pieces, Side, Sides, Square}, BitMove, Board, MoveList};

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

    pub fn generate_pawn_pushes(&self, board: &Board, list: &mut MoveList) {
        let side = board.game_state.side_to_move;
        let mut pawns = board.pieces[Pieces::PAWN][side];
        //println!("Initial pawns bitboard {:?}", w_pawns);
        let empty_bb: BitBoard = !board.occupancy(Sides::BOTH);
        while pawns.0 > 0 {
            let from = BitBoard::next(&mut pawns);
            let from_bb = from.to_bb();
            let mut to_bb: BitBoard = match side {
                Sides::WHITE => Self::w_pawn_single_push(from_bb, empty_bb) | Self::w_pawn_double_push(from_bb, empty_bb),
                Sides::BLACK => Self::b_pawn_single_push(from_bb, empty_bb) | Self::b_pawn_double_push(from_bb, empty_bb),
                _ => panic!()
            };
            while to_bb.0 > 0 {
                let to = BitBoard::next(&mut to_bb);
                //println!("Adding pawn move from {} to {}", from, to);
                self.add_move(&board, list, Pieces::PAWN, from.clone(), to)
            }
        }
    }

    pub fn generate_pawn_attacks(&self, board: &Board, list: &mut MoveList) {
        let side = board.game_state.side_to_move;
        let mut pawns = board.pieces[Pieces::PAWN][side];
        while pawns.0 > 0 {
            let from = BitBoard::next(&mut pawns);
            let targets =  self.get_pawn_attacks_from_square(side, &from);
            let captures = targets & board.occupancy(board.opponent());
            let ep_captures = match board.game_state.en_passant {
                Some(ep_square) => targets & Square(ep_square.into()).to_bb(),
                None => BitBoard(0)
            };
            let mut to_bb = captures | ep_captures;
            while to_bb.0 > 0 {
                let to = BitBoard::next(&mut to_bb);
                //println!("Adding pawn move from {} to {}", from, to);
                self.add_move(&board, list, Pieces::PAWN, from.clone(), to)
            }
        }
    }

    pub fn generate_pawn_moves(&self, board: &Board, list: &mut MoveList) {
        self.generate_pawn_pushes(board, list);
        self.generate_pawn_attacks(board, list);
    }

    pub fn get_pawn_attacks_from_square(&self, side: Side, square: &Square) -> BitBoard {
        self.pawns[side][square.0]
    }
}

