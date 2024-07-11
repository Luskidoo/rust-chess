mod fen;
mod game_state;
mod zobrist;
mod utils;

use game_state::GameState;

use crate::bitboard::*;
use crate::defs::*;

#[derive(Copy, Clone, PartialEq)]
pub struct Board {
    pub pieces: [[BitBoard; Sides::BOTH + 1]; NrOf::PIECE_TYPES],
    pub piece_list: [Piece; NrOf::SQUARES],
    pub game_state: GameState,
}

impl Board {
    pub fn new() -> Self { 
        Self {
            pieces: [[BitBoard(0); Sides::BOTH + 1]; NrOf::PIECE_TYPES],
            piece_list: [Pieces::NONE; NrOf::SQUARES],
            game_state: GameState::new(),
        }
    }

    pub fn occupancy(self, side: Side) -> BitBoard {
        match side {
            Sides::WHITE => self.white_occupied(),
            Sides::BLACK => self.black_occupied(),
            Sides::BOTH => self.white_occupied() | self.black_occupied(),
            _ => panic!("Invalid side")
        }
    }

    pub fn white_occupied(self) -> BitBoard {
        self.pieces[Pieces::PAWN][Sides::WHITE] | self.pieces[Pieces::BISHOP][Sides::WHITE] | self.pieces[Pieces::KNIGHT][Sides::WHITE] | self.pieces[Pieces::ROOK][Sides::WHITE] | self.pieces[Pieces::QUEEN][Sides::WHITE] | self.pieces[Pieces::KING][Sides::WHITE]
    }

    pub fn black_occupied(self) -> BitBoard {
        self.pieces[Pieces::PAWN][Sides::BLACK] | self.pieces[Pieces::BISHOP][Sides::BLACK] | self.pieces[Pieces::KNIGHT][Sides::BLACK] | self.pieces[Pieces::ROOK][Sides::BLACK] | self.pieces[Pieces::QUEEN][Sides::BLACK] | self.pieces[Pieces::KING][Sides::BLACK]
    }
    
    // Initialize the piece list. This list is used to quickly determine
    // which piece type (rook, knight...) is on a square without having to
    // loop through the piece bitboards.
    fn init_piece_list(&self) -> [Piece; NrOf::SQUARES] {
        let bb_w = self.pieces[Sides::WHITE]; // White piece bitboards
        let bb_b = self.pieces[Sides::BLACK]; // Black piece bitboards
        let mut piece_list: [Piece; NrOf::SQUARES] = [Pieces::NONE; NrOf::SQUARES];

        // piece_type is enumerated, from 0 to 6.
        // 0 = KING, 1 = QUEEN, and so on, as defined in board::defs.
        for (piece_type, (w, b)) in bb_w.iter().zip(bb_b.iter()).enumerate() {
            let mut white_pieces = *w; // White pieces of type "piece_type"
            let mut black_pieces = *b; // Black pieces of type "piece_type"

            // Put white pieces into the piece list.
            while white_pieces > BitBoard(0) {
                let square = BitBoard::next(&mut white_pieces);
                piece_list[square.0] = piece_type;
            }

            // Put black pieces into the piece list.
            while black_pieces > BitBoard(0) {
                let square = BitBoard::next(&mut black_pieces);
                piece_list[square.0] = piece_type;
            }
        }

        piece_list
    }
}

impl Board {
    pub fn reset(&mut self) {
        self.pieces = [[BitBoard(0); Sides::BOTH + 1]; NrOf::PIECE_TYPES];
        self.game_state = GameState::new();
        self.piece_list = [Pieces::NONE; NrOf::SQUARES];
        //self.history.clear();
        //self.piece_list = [Pieces::NONE; NrOf::SQUARES];
    }

    // Main initialization function. This is used to initialize the "other"
    // bit-boards that are not set up by the FEN-reader function.
    fn init(&mut self) {
        self.piece_list = self.init_piece_list();
    }
}