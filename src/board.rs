mod fen;
mod game_state;
mod zobrist;
mod utils;
mod history;

use std::sync::Arc;

use game_state::GameState;
use history::History;
use zobrist::ZobristKey;
use zobrist::ZobristRandoms;

use crate::bitboard::*;
use crate::defs::*;

#[derive(Clone)]
pub struct Board {
    pub pieces: [[BitBoard; Sides::BOTH + 1]; NrOf::PIECE_TYPES],
    pub piece_list: [Piece; NrOf::SQUARES],
    pub game_state: GameState,
    pub history: History,
    zr: ZobristRandoms,
}

impl Board {
    pub fn new() -> Self { 
        Self {
            pieces: [[BitBoard(0); Sides::BOTH + 1]; NrOf::PIECE_TYPES],
            piece_list: [Pieces::NONE; NrOf::SQUARES],
            game_state: GameState::new(),
            history: History::new(),
            zr: ZobristRandoms::new(),
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
        self.pieces[piece][side] ^= square.clone().to_bb();
        //self.bb_side[side] ^= square.to_bb();
        self.piece_list[square.0] = Pieces::NONE;
        self.game_state.zobrist_key ^= self.zr.piece(side, piece, square);

        // Incremental updates
        // =============================================================
        let flip = side == Sides::WHITE;
        //let s = if flip { FLIP[square.0] } else { square };
        //self.game_state.psqt[side] -= PSQT_MG[piece][s];
    }

    // Put a piece onto the board, for the given side, piece, and square.
    pub fn put_piece(&mut self, side: Side, piece: Piece, square: Square) {
        self.pieces[piece][side] |= square.clone().to_bb();
        //self.bb_side[side] |= square.to_bb();
        self.piece_list[square.0] = piece;
        self.game_state.zobrist_key ^= self.zr.piece(side, piece, square);

        // Incremental updates
        // =============================================================
        //let flip = side == Sides::WHITE;
        //let s = if flip { FLIP[square.0] } else { square };
        //self.game_state.psqt[side] += PSQT_MG[piece][s];
    }

    // Remove a piece from the from-square, and put it onto the to-square.
    pub fn move_piece(&mut self, side: Side, piece: Piece, from: Square, to: Square) {
        self.remove_piece(side, piece, from);
        self.put_piece(side, piece, to);
    }

    // Set a square as being the current ep-square.
    pub fn set_ep_square(&mut self, square: Square) {
        self.game_state.zobrist_key ^= self.zr.en_passant(self.game_state.en_passant);
        self.game_state.en_passant = Some(square.0 as u8);
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
    pub fn update_castling_permissions(&mut self, new_permissions: BitBoard) {
        self.game_state.zobrist_key ^= self.zr.castling(self.game_state.castling);
        self.game_state.castling = new_permissions;
        self.game_state.zobrist_key ^= self.zr.castling(self.game_state.castling);
    }

    // Initialize the zobrist hash. This hash will later be updated incrementally.
    pub fn init_zobrist_key(&self) -> ZobristKey {
        // Keep the key here.
        let mut key: u64 = 0;

        // Same here: "bb_w" is shorthand for
        // "self.bb_pieces[Sides::WHITE]".
        let bb_w = self.pieces[Sides::WHITE];
        let bb_b = self.pieces[Sides::BLACK];

        // Iterate through all piece types, for both white and black.
        // "piece_type" is enumerated, and it'll start at 0 (KING), then 1
        // (QUEEN), and so on.
        for (piece_type, (w, b)) in bb_w.iter().zip(bb_b.iter()).enumerate() {
            // Assume the first iteration; piece_type will be 0 (KING). The
            // following two statements will thus get all the pieces of
            // type "KING" for white and black. (This will obviously only
            // be one king, but with rooks, there will be two in the
            // starting position.)
            let mut white_pieces = *w;
            let mut black_pieces = *b;

            // Iterate through all the piece locations of the current piece
            // type. Get the square the piece is on, and then hash that
            // square/piece combination into the zobrist key.
            while white_pieces > BitBoard(0) {
                let square = BitBoard::next(&mut white_pieces);
                key ^= self.zr.piece(Sides::WHITE, piece_type, square);
            }

            // Same for black.
            while black_pieces > BitBoard(0) {
                let square = BitBoard::next(&mut black_pieces);
                key ^= self.zr.piece(Sides::BLACK, piece_type, square);
            }
        }

        // Hash the castling, active color, and en-passant state into the key.
        key ^= self.zr.castling(self.game_state.castling);
        key ^= self.zr.side(self.game_state.side_to_move as usize);
        key ^= self.zr.en_passant(self.game_state.en_passant);

        // Done; return the key.
        key
    }

    pub fn print_board(&self) {
        let piece_chars = ['K', 'Q', 'R', 'B', 'N', 'P', 'k', 'q', 'r', 'b', 'n', 'p', '.'];
        
        println!("  a b c d e f g h");
        println!("  ---------------");
        for rank in (0..8).rev() {
            print!("{} ", rank + 1);
            for file in 0..8 {
                let square = rank * 8 + file;
                let mut piece_char = '.';
                
                for (piece_type, boards) in self.pieces.iter().enumerate() {
                    if boards[Sides::WHITE].0 & (1 << square) != 0 {
                        piece_char = piece_chars[piece_type];
                        break;
                    } else if boards[Sides::BLACK].0 & (1 << square) != 0 {
                        piece_char = piece_chars[piece_type + 6];
                        break;
                    }
                }
                
                print!("{} ", piece_char);
            }
            println!("| {}", rank + 1);
        }
        println!("  ---------------");
        println!("  a b c d e f g h");
        
        println!("Side to move: {}", if self.game_state.side_to_move == Sides::WHITE { "White" } else { "Black" });
        println!("Castling rights: {}{}{}{}", 
            if self.game_state.castling.0 & Castling::WK.0 != 0 { "K" } else { "" },
            if self.game_state.castling.0 & Castling::WQ.0 != 0 { "Q" } else { "" },
            if self.game_state.castling.0 & Castling::BK.0 != 0 { "k" } else { "" },
            if self.game_state.castling.0 & Castling::BQ.0 != 0 { "q" } else { "" }
        );
        println!("En passant square: {}", self.game_state.en_passant.map_or("None".to_string(), |sq| format!("{}", sq)));
        println!("Halfmove clock: {}", self.game_state.halfmove_clock);
        println!("Fullmove number: {}", self.game_state.fullmove_number);
        println!();
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