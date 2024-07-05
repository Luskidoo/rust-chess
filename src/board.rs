mod fen;
mod game_state;

use game_state::GameState;

use crate::bitboard::*;
use crate::defs::*;

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

    // pub fn init_knights() -> [BitBoard; 64] {
    //     let attacks = [BitBoard(0); 64];
    //     for sq in 0..63 {
    //         let bb_square: BitBoard = BitBoard(1) << BitBoard(square);
    //         let bb_moves =
    //         (bb_square & !BB_RANKS[Ranks::R8] & !BB_RANKS[Ranks::R7] & !BB_FILES[Files::A]) << BitBoard(15)
    //         | (bb_square & !BB_RANKS[Ranks::R8] & !BB_RANKS[Ranks::R7] & !BB_FILES[Files::H]) << BitBoard(17)
    //         | (bb_square & !BB_FILES[Files::A] & !BB_FILES[Files::B] & !BB_RANKS[Ranks::R8]) << BitBoard(6)
    //         | (bb_square & !BB_FILES[Files::G] & !BB_FILES[Files::H] & !BB_RANKS[Ranks::R8]) << BitBoard(10)
    //         | (bb_square & !BB_RANKS[Ranks::R1] & !BB_RANKS[Ranks::R2] & !BB_FILES[Files::A]) >> BitBoard(17)
    //         | (bb_square & !BB_RANKS[Ranks::R1] & !BB_RANKS[Ranks::R2] & !BB_FILES[Files::H]) >> BitBoard(15)
    //         | (bb_square & !BB_FILES[Files::A] & !BB_FILES[Files::B] & !BB_RANKS[Ranks::R1]) >> BitBoard(10)
    //         | (bb_square & !BB_FILES[Files::G] & !BB_FILES[Files::H] & !BB_RANKS[Ranks::R1]) >> BitBoard(6);
    //         attacks[sq] = bb_moves;
    //     }
    //     attacks
    // }
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