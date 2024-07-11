mod fen;
mod game_state;
mod zobrist;
mod utils;
mod history;

use game_state::GameState;
use history::History;

use crate::bitboard::*;
use crate::defs::*;

#[derive(Clone, Copy)]
pub struct Board {
    pub pieces: [[BitBoard; Sides::BOTH + 1]; NrOf::PIECE_TYPES],
    pub piece_list: [Piece; NrOf::SQUARES],
    pub game_state: GameState,
    pub history: History
}

impl Board {
    pub fn new() -> Self { 
        Self {
            pieces: [[BitBoard(0); Sides::BOTH + 1]; NrOf::PIECE_TYPES],
            piece_list: [Pieces::NONE; NrOf::SQUARES],
            game_state: GameState::new(),
            history: History::new(),
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

    // Remove a piece from the board, for the given side, piece, and square.
    pub fn remove_piece(&mut self, side: Side, piece: Piece, square: Square) {
        self.pieces[piece][side] ^= square.to_bb();
        self.bb_side[side] ^= square.to_bb();
        self.piece_list[square.0] = Pieces::NONE;
        self.game_state.zobrist_key ^= self.zr.piece(side, piece, square);

        // Incremental updates
        // =============================================================
        let flip = side == Sides::WHITE;
        let s = if flip { FLIP[square.0] } else { square };
        self.game_state.psqt[side] -= PSQT_MG[piece][s];
    }

    // Put a piece onto the board, for the given side, piece, and square.
    pub fn put_piece(&mut self, side: Side, piece: Piece, square: Square) {
        self.pieces[piece][side] |= square.to_bb();
        self.bb_side[side] |= square.to_bb();
        self.piece_list[square.0] = piece;
        self.game_state.zobrist_key ^= self.zr.piece(side, piece, square);

        // Incremental updates
        // =============================================================
        let flip = side == Sides::WHITE;
        let s = if flip { FLIP[square.0] } else { square };
        self.game_state.psqt[side] += PSQT_MG[piece][s];
    }

    // Remove a piece from the from-square, and put it onto the to-square.
    pub fn move_piece(&mut self, side: Side, piece: Piece, from: Square, to: Square) {
        self.remove_piece(side, piece, from);
        self.put_piece(side, piece, to);
    }

    // Set a square as being the current ep-square.
    pub fn set_ep_square(&mut self, square: Square) {
        self.game_state.zobrist_key ^= self.zr.en_passant(self.game_state.en_passant);
        self.game_state.en_passant = Some(square as u8);
        self.game_state.zobrist_key ^= self.zr.en_passant(self.game_state.en_passant);
    }

    // Clear the ep-square. (If the ep-square is None already, nothing changes.)
    pub fn clear_ep_square(&mut self) {
        self.game_state.zobrist_key ^= self.zr.en_passant(self.game_state.en_passant);
        self.game_state.en_passant = None;
        self.game_state.zobrist_key ^= self.zr.en_passant(self.game_state.en_passant);
    }

    // Swap side from WHITE <==> BLACK
    pub fn swap_side(&mut self) {
        self.game_state.zobrist_key ^= self.zr.side(self.game_state.side_to_move as usize);
        self.game_state.side_to_move ^= 1;
        self.game_state.zobrist_key ^= self.zr.side(self.game_state.side_to_move as usize);
    }

    // Update castling permissions and take Zobrist-key into account.
    pub fn update_castling_permissions(&mut self, new_permissions: u8) {
        self.game_state.zobrist_key ^= self.zr.castling(self.game_state.castling);
        self.game_state.castling = new_permissions;
        self.game_state.zobrist_key ^= self.zr.castling(self.game_state.castling);
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