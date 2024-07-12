use crate::{movegen::bit_move::Move, BitBoard};

use super::Side;

#[derive(Copy, Clone, PartialEq)]
pub struct GameState {
    pub halfmove_clock: u8,
    pub en_passant: Option<u8>,
    pub fullmove_number: u16,
    pub castling: BitBoard,
    pub side_to_move: Side,
    pub next_move: Move,
    pub zobrist_key: u64,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            side_to_move: 0,
            castling: 0,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 0,
            //psqt: [0; Sides::BOTH],
            next_move: Move::new(0),
            zobrist_key: 0,
        }
    }
}