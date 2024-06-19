use crate::bitboard::*;
use crate::board::*;
use crate::bitmove::*;
use crate::movelist::*;
use crate::sq::*;
use crate::defs::{Sides};

mod pawns;
mod knights;
mod magics;

pub(crate) struct MoveGenerator {
    pub knight_moves_array: [BitBoard; 64],
    pub white_pawn_attacks: [BitBoard; 64],
    pub black_pawn_attacks: [BitBoard; 64]
}

impl MoveGenerator {
    pub fn new () -> Self {
        Self {
            knight_moves_array: Self::init_knight_moves(),
            white_pawn_attacks: Self::init_white_pawn_attacks(),
            black_pawn_attacks: Self::init_black_pawn_attacks()
        }
        
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
        let east_attacks = BitBoard(sq << 9u64) & BitBoard::not_a_file;
        let west_attacks = BitBoard(sq << 7u64) & BitBoard::not_h_file;
        east_attacks | west_attacks
    }

    fn black_pawn_attacks(sq: u64) -> BitBoard {
        let east_attacks = BitBoard(sq >> 7u64) & BitBoard::not_a_file;
        let west_attacks = BitBoard(sq >> 9u64) & BitBoard::not_h_file;
        east_attacks | west_attacks
    }
    
    fn init_knight_moves() -> [BitBoard; 64] {
        let mut moves = [BitBoard(0); 64];
        for sq in 0..64 {
            moves[sq] = Self::knight_moves((1u64 << sq) as u64);
        }
        moves
    }

    fn init_white_pawn_attacks() -> [BitBoard; 64] {
        let mut moves = [BitBoard(0); 64];
        for sq in 0..64 {
            moves[sq] = Self::white_pawn_attacks((1u64 << sq) as u64);
        }
        moves
    }

    fn init_black_pawn_attacks() -> [BitBoard; 64] {
        let mut moves = [BitBoard(0); 64];
        for sq in 0..64 {
            moves[sq] = Self::black_pawn_attacks((1u64 << sq) as u64);
        }
        moves
    }

    fn generate_knight_moves(&self, board: Board, list: &mut MoveList) {
        let w_knights = board.knights[Sides::WHITE];
        let w_empty = board.white_empty();
        Self::w_knight_moves(&self, w_knights, w_empty, list);
    }
    
    pub fn generate_all_moves(self, board: Board, list: &mut MoveList) {
        Self::generate_w_pawn_pushes(board, list);
        Self::generate_w_pawn_attacks(&self, board, list);
        Self::generate_knight_moves(&self, board, list);
    }
}




