use bit_move::Move;
use bit_move::Shift;
use magics::Magic;

use crate::bitboard::*;
use crate::board::*;
use crate::bitmove::*;
use crate::defs::Piece;
use crate::defs::Pieces;
use crate::defs::Side;
use crate::defs::Square;
use crate::movelist::*;
use crate::sq::*;
use crate::defs::{Sides};

mod pawns;
mod knights;
mod magics;
mod init;
mod slide;
mod king;
pub mod bit_move;
mod castle;

// This is a list of all pieces a pawn can promote to.
const PROMOTION_PIECES: [usize; 4] = [Pieces::QUEEN, Pieces::ROOK, Pieces::BISHOP, Pieces::KNIGHT];

pub(crate) struct MoveGenerator {
    pub knight_moves_array: [BitBoard; 64],
    pub pawns: [[BitBoard; 64]; 2],
    pub king_attacks: [BitBoard; 64],
    pub rook: Vec<BitBoard>,
    pub bishop: Vec<BitBoard>,
    pub rook_magics: [Magic; 64],
    pub bishop_magics: [Magic; 64],
}

impl MoveGenerator {
    pub fn new () -> Self {
        let mut mg = Self {
            knight_moves_array: Self::init_knight_moves(),
            pawns: [[BitBoard(0); 64]; 2],
            king_attacks: Self::init_king_moves(),
            rook_magics: [Magic::new(); 64],
            bishop_magics: [Magic::new(); 64],
            rook: vec![BitBoard(0); 102400],
            bishop: vec![BitBoard(0); 5248],
        };
        mg.init_pawn_attacks();
        mg.init_magics(true);
        mg.init_magics(false);
        mg
        
    }

    fn knight_moves(sq: u64) -> BitBoard {
        let l1: BitBoard = BitBoard(sq >> 1) & BitBoard(0x7f7f7f7f7f7f7f7f);
        let l2: BitBoard = BitBoard(sq >> 2) & BitBoard(0x3f3f3f3f3f3f3f3f);
        let r1: BitBoard = BitBoard(sq << 1) & BitBoard(0xfefefefefefefefe);
        let r2: BitBoard = BitBoard(sq << 2) & BitBoard(0xfcfcfcfcfcfcfcfc);
        let h1: BitBoard = l1 | r1;
        let h2: BitBoard = l2 | r2;
        BitBoard((h1.0 << 16) | (h1.0 >> 16) | (h2.0 << 8) | (h2.0 >> 8))
    }

    fn white_pawn_attacks(sq: u64) -> BitBoard {
        let east_attacks = BitBoard(sq << 9u64) & BitBoard::NOT_A_FILE;
        let west_attacks = BitBoard(sq << 7u64) & BitBoard::NOT_H_FILE;
        east_attacks | west_attacks
    }

    fn black_pawn_attacks(sq: u64) -> BitBoard {
        let east_attacks = BitBoard(sq >> 7u64) & BitBoard::NOT_A_FILE;
        let west_attacks = BitBoard(sq >> 9u64) & BitBoard::NOT_H_FILE;
        east_attacks | west_attacks
    }
    
    fn generate_knight_moves(&self, board: Board, list: &mut MoveList) {
        let w_knights = board.knights[Sides::WHITE];
        let w_empty = board.white_empty();
        Self::w_knight_moves(&self, w_knights, w_empty, list);
    }

    fn king_moves(kings: BitBoard) -> BitBoard {
        let mut attacks = kings.east_one() | kings.west_one();
        let intermediate = kings | attacks;
        attacks |= intermediate.north_one() | intermediate.south_one();
        attacks
    }
    
    pub fn generate_all_moves(self, board: Board, list: &mut MoveList) {
        Self::generate_w_pawn_pushes(board, list);
        Self::generate_w_pawn_attacks(&self, board, list);
        Self::generate_knight_moves(&self, board, list);
        Self::generate_rook_moves(&self, board, list);
        Self::generate_bishop_moves(&self, board, list);
        Self::generate_queen_moves(&self, board, list);
        Self::generate_king_moves(&self, board, list);
    }

    pub fn square_attacked(&self, board: &Board, attacker: Side, square: &Square) -> bool {
        // Use the super-piece method: get the moves for each piece,
        // starting from the given square. This provides the sqaures where
        // a piece has to be, to be able to reach the given square.
        let occupancy = board.occupancy(board.game_state.side_to_move);
        let bb_king = self.get_knight_attacks(square);
        let bb_rook = self.get_slider_attacks(Pieces::ROOK, square, occupancy);
        let bb_bishop = self.get_slider_attacks(Pieces::BISHOP, square, occupancy);
        let bb_knight = self.get_king_attacks(square);
        let bb_pawns = self.get_pawn_attacks_from_square(attacker ^ 1, square);
        let bb_queen = bb_rook | bb_bishop;

        // Then determine if such a piece is actually there: see if a rook
        // is on one of the squares a rook has to be to reach the given
        // square. Same for the queen, knight, etc... As soon as one is
        // found, the square is attacked.
        (bb_king & board.pieces[Pieces::KING][attacker] > BitBoard(0))
            || (bb_rook & board.pieces[Pieces::ROOK][attacker] > BitBoard(0))
            || (bb_queen & board.pieces[Pieces::QUEEN][attacker] > BitBoard(0))
            || (bb_bishop & board.pieces[Pieces::BISHOP][attacker] > BitBoard(0))
            || (bb_knight & board.pieces[Pieces::KNIGHT][attacker] > BitBoard(0))
            || (bb_pawns & board.pieces[Pieces::PAWN][attacker] > BitBoard(0))
    }

    pub fn add_move(&self, board: Board, list: &mut MoveList, piece: Piece, from: Square, to: Square) {
            let is_pawn = piece == Pieces::PAWN;
            let promotion_rank = Board::promotion_rank(board.game_state.side_to_move);
            let promotion = is_pawn && Board::square_on_rank(&to, Square(promotion_rank));
            let capture = board.piece_list[to.0];
            let castling = (piece == Pieces::KING) && ((to.0 as i8 - from.0 as i8).abs() == 2);
            let ep_capture = match board.game_state.en_passant {
                Some(square) => is_pawn && (square as usize == to.0),
                None => false,
            };
            let double_step = is_pawn && ((to.0 as i8 - from.0 as i8).abs() == 16);
            let mut move_data = (piece)
                | from.0 << Shift::FROM_SQ
                | to.0 << Shift::TO_SQ
                | capture << Shift::CAPTURE
                | (ep_capture as usize) << Shift::EN_PASSANT
                | (double_step as usize) << Shift::DOUBLE_STEP
                | (castling as usize) << Shift::CASTLING;

            // Push the move to the piece list...
            if !promotion {
                move_data |= Pieces::NONE << Shift::PROMOTION;
                list.push(Move::new(move_data));
            } else {
                // ...or push four promotion moves.
                PROMOTION_PIECES.iter().for_each(|piece| {
                    let promotion_piece = *piece << Shift::PROMOTION;
                    list.push(Move::new(move_data | promotion_piece))
                });
            }
        }
}




