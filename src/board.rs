mod fen;

use crate::bitboard::*;
use crate::defs::*;

#[derive(Copy, Clone, PartialEq)]
pub struct GameState {
    pub halfmove_clock: u8,
    pub en_passant: Option<u8>,
    pub fullmove_number: u16,
    pub castling: BitBoard,
    pub side_to_move: u8,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            side_to_move: 0,
            castling: BitBoard(0),
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 0,
            //zobrist_key: 0,
            //psqt: [0; Sides::BOTH],
            //next_move: Move::new(0),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Board {
    pub pawns: [BitBoard; 3],
    pub bishops: [BitBoard; 3],
    pub knights: [BitBoard; 3],
    pub rooks: [BitBoard; 3],
    pub queens: [BitBoard; 3],
    pub king: [BitBoard; 3],
    pub game_state: GameState,
}

impl Board {
    pub fn new() -> Self { 
        Self {
            pawns: [BitBoard::empty; 3],
            bishops: [BitBoard::empty; 3],
            knights: [BitBoard::empty; 3],
            rooks: [BitBoard::empty; 3],
            queens: [BitBoard::empty; 3],
            king: [BitBoard::empty; 3],
            game_state: GameState::new(),
        }
    }

    pub fn empty_squares(self) -> BitBoard {
        !(self.pawns[Sides::BOTH] | self.bishops[Sides::BOTH] | self.knights[Sides::BOTH] | self.rooks[Sides::BOTH] | self.queens[Sides::BOTH] | self.king[Sides::BOTH])
    }

    pub fn white_occupied(self) -> BitBoard {
        self.pawns[Sides::WHITE] | self.bishops[Sides::WHITE] | self.knights[Sides::WHITE] | self.rooks[Sides::WHITE] | self.queens[Sides::WHITE] | self.king[Sides::WHITE]
    }

    pub fn black_occupied(self) -> BitBoard {
        self.pawns[Sides::BLACK] | self.bishops[Sides::BLACK] | self.knights[Sides::BLACK] | self.rooks[Sides::BLACK] | self.queens[Sides::BLACK] | self.king[Sides::BLACK]
    }

    pub fn white_empty(self) -> BitBoard {
        !(self.pawns[Sides::WHITE] | self.bishops[Sides::WHITE] | self.knights[Sides::WHITE] | self.rooks[Sides::WHITE] | self.queens[Sides::WHITE] | self.king[Sides::WHITE])
    }

    pub fn black_empty(self) -> BitBoard {
        !(self.pawns[Sides::BLACK] | self.bishops[Sides::BLACK] | self.knights[Sides::BLACK] | self.rooks[Sides::BLACK] | self.queens[Sides::BLACK] | self.king[Sides::BLACK])
    }
}

impl Board {
    pub fn reset(&mut self) {
        self.pawns = [BitBoard::empty; 3];
        self.bishops =  [BitBoard::empty; 3];
        self.knights = [BitBoard::empty; 3];
        self.rooks = [BitBoard::empty; 3];
        self.queens = [BitBoard::empty; 3];
        self.king = [BitBoard::empty; 3];
        self.game_state = GameState::new();
        //self.history.clear();
        //self.piece_list = [Pieces::NONE; NrOf::SQUARES];
    }
}