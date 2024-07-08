use crate::{defs::{Castling, Pieces, Sides, Square}, BitBoard, Board, MoveList};

use super::MoveGenerator;

impl MoveGenerator {
    pub fn castling(&self, board: &Board, list: &mut MoveList) {
        // Create shorthand variables.
        let side_to_move = board.game_state.side_to_move;
        let opponent = side_to_move ^ 1;
        let castle_perms_white = (board.game_state.castling & (Castling::WK | Castling::WQ)) > BitBoard(0);
        let castle_perms_black = (board.game_state.castling & (Castling::BK | Castling::BQ)) > BitBoard(0);
        let bb_occupancy = board.occupancy(Sides::BOTH);
        let mut bb_king = board.pieces[Pieces::KING][board.game_state.side_to_move];
        let from = BitBoard::next(&mut bb_king);
    
        // Generate castling moves for white.
        if side_to_move == Sides::WHITE && castle_perms_white {
            // Kingside
            if board.game_state.castling & Castling::WK > BitBoard(0) {
                let bb_kingside_blockers = Square::F1.to_bb() | Square::G1.to_bb();
                let is_kingside_blocked = (bb_occupancy & bb_kingside_blockers) > BitBoard(0);
    
                if !is_kingside_blocked
                    && !self.square_attacked(board, opponent, &Square::E1)
                    && !self.square_attacked(board, opponent, &Square::F1)
                {
                    let to = Square(from.try_into().unwrap()).to_bb() << BitBoard(2);
                    self.add_move(board, Pieces::KING, from, to, list);
                }
            }
    
            if board.game_state.castling & Castling::WQ > BitBoard(0) {
                // Queenside
                let bb_queenside_blockers =
                    Square::B1.to_bb() | Square::C1.to_bb() | Square::D1.to_bb();
                let is_queenside_blocked = (bb_occupancy & bb_queenside_blockers) > BitBoard(0);
    
                if !is_queenside_blocked
                    && !self.square_attacked(board, opponent, &Square::E1)
                    && !self.square_attacked(board, opponent, &Square::D1)
                {
                    let to = Square(from.try_into().unwrap()).to_bb() >> BitBoard(2);
                    self.add_move(board, Pieces::KING, from, to, list);
                }
            }
        }
    
        // Generate castling moves for black.
        if side_to_move == Sides::BLACK && castle_perms_black {
            // Kingside
            if board.game_state.castling & Castling::BK > BitBoard(0) {
                let bb_kingside_blockers = Square::F8.to_bb() | Square::G8.to_bb();
                let is_kingside_blocked = (bb_occupancy & bb_kingside_blockers) > BitBoard(0);
    
                if !is_kingside_blocked
                    && !self.square_attacked(board, opponent, &Square::E8)
                    && !self.square_attacked(board, opponent, &Square::F8)
                {
                    let to = Square(from.try_into().unwrap()).to_bb() << BitBoard(2);
                    self.add_move(board, Pieces::KING, from, to, list);
                }
            }
    
            // Queenside
            if board.game_state.castling & Castling::BQ > BitBoard(0) {
                let bb_queenside_blockers =
                    Square::B8.to_bb() | Square::C8.to_bb() | Square::D8.to_bb();
                let is_queenside_blocked = (bb_occupancy & bb_queenside_blockers) > BitBoard(0);
    
                if !is_queenside_blocked
                    && !self.square_attacked(board, opponent, &Square::E8)
                    && !self.square_attacked(board, opponent, &Square::D8)
                {
                    let to = Square(from.try_into().unwrap()).to_bb() >> BitBoard(2);
                    self.add_move(board, Pieces::KING, from, to, list);
                }
            }
        }
    }
}

