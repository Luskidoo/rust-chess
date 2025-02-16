use crate::{defs::{Castling, NrOf, Piece, Pieces, Side, Sides, Square}, BitBoard, MoveGenerator, Board, movegen::bit_move::Move};

// Castling Permissions Per Square
type CPSquare = [BitBoard; NrOf::SQUARES];
const CASTLING_PERMS: CPSquare = castling_permissions_per_square();
const fn castling_permissions_per_square() -> CPSquare {
    // First set all squares grant all castling permissions. This means
    // moving a piece on such square doesn't have any effect on castling
    // permissions.
    let mut cp: CPSquare = [Castling::ALL; NrOf::SQUARES];

    // Now disable castling permissions when moving pieces on certain
    // squares. For example, when the piece (rook) on A1 moves, disable
    // white castling to the queenside.
    cp[Square::A1.0] = BitBoard(cp[Square::A1.0].0 & !Castling::WQ.0);
    cp[Square::E1.0] = BitBoard(cp[Square::E1.0].0 & !Castling::WK.0 & !Castling::WQ.0);
    cp[Square::H1.0] = BitBoard(cp[Square::H1.0].0 & !Castling::WK.0);
    cp[Square::A8.0] = BitBoard(cp[Square::A8.0].0 & !Castling::BQ.0);
    cp[Square::E8.0] = BitBoard(cp[Square::E8.0].0 & !Castling::BK.0 & !Castling::BQ.0);
    cp[Square::H8.0] = BitBoard(cp[Square::H8.0].0 & !Castling::BK.0);

    cp
}

/*** ================================================================================ ***/

// Make() executes the given move and checks if it is legal. If it's not legal,
// the move is immediately reversed using unmake(), and the board is not changed.

impl Board {
    #[cfg_attr(debug_assertions, inline(never))]
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn make(&mut self, m: Move, mg: &MoveGenerator) -> bool {
        //println!("Making move: {} for side {}", m.as_string(), self.game_state.side_to_move);
        //println!("Board state before move:");
        //self.print_board();
        // Create the unmake info and store it.
        let mut current_game_state = self.game_state;
        current_game_state.next_move = m;
        self.history.push(current_game_state);

        // Set "us" and "opponent"
        let us = self.game_state.side_to_move;
        let opponent = us ^ 1;

        // Dissect the move so we don't need "m.function()" and type casts everywhere.
        let piece = m.piece();
        let from = m.from();
        let to = m.to();
        let captured = m.captured();
        let promoted = m.promoted();
        let castling = m.castling();
        let double_step = m.double_step();
        let en_passant = m.en_passant();

        // Shorthands
        let is_promotion = promoted != Pieces::NONE;
        let is_capture = captured != Pieces::NONE;
        let has_permissions = self.game_state.castling.0 > 0;

        // Assume this is not a pawn move or a capture.
        self.game_state.halfmove_clock += 1;

        // Every move except double_step unsets the ep-square.
        if self.game_state.en_passant.is_some() {
            self.clear_ep_square();
        }

        // If a piece was captured with this move then remove it. Also reset halfmove_clock.
        if is_capture {
            //println!("Board before capture on {}:", SQUARE_NAME[to.0]);
            //println!("{}", self.occupancy(opponent));
            self.remove_piece(opponent, captured, to.clone());
            //println!("Board after capture:");
            //println!("{}", self.occupancy(opponent));
            self.game_state.halfmove_clock = 0;
            // Change castling permissions on rook capture in the corner.
            if captured == Pieces::ROOK && has_permissions {
                self.update_castling_permissions(self.game_state.castling & CASTLING_PERMS[to.clone().0]);
            }
        }

        // Make the move. Just move the piece if it's not a pawn.
        if piece != Pieces::PAWN {
            self.move_piece(us, piece, from.clone(), to.clone());
        } else {
            // It's a pawn move. Take promotion into account and reset halfmove_clock.
            self.remove_piece(us, piece, from.clone());
            self.put_piece(us, if !is_promotion { piece } else { promoted }, to.clone());
            self.game_state.halfmove_clock = 0;

            // After an en-passant maneuver, the opponent's pawn must also be removed.
            if en_passant {
                self.remove_piece(opponent, Pieces::PAWN, Square(to.clone().0 ^ 8));
            }

            // A double-step is the only move that sets the ep-square.
            if double_step {
                self.set_ep_square(Square(to.0 ^ 8));
            }
        }

        // Remove castling permissions if king/rook leaves from starting square.
        // (This will also adjust permissions when castling, because the king moves.)
        if (piece == Pieces::KING || piece == Pieces::ROOK) && has_permissions {
            self.update_castling_permissions(self.game_state.castling & CASTLING_PERMS[from.clone().0]);
        }

        // If the king is castling, then also move the rook.
        if castling {
            match to {
                Square::G1 => self.move_piece(us, Pieces::ROOK, Square::H1, Square::F1),
                Square::C1 => self.move_piece(us, Pieces::ROOK, Square::A1, Square::D1),
                Square::G8 => self.move_piece(us, Pieces::ROOK, Square::H8, Square::F8),
                Square::C8 => self.move_piece(us, Pieces::ROOK, Square::A8, Square::D8),
                _ => panic!("Error moving rook during castling."),
            }
        }

        // Swap the side to move.
        self.swap_side();

        // Increase full move number if black has moved
        if us == Sides::BLACK {
            self.game_state.fullmove_number += 1;
        }

        //println!("Board state after move:");
        //self.print_board();

        /*** Validating move: see if "us" is in check. If so, undo everything. ***/
        let king_square = Square(self.pieces[Pieces::KING][us].0.trailing_zeros() as usize);
        let is_legal = !mg.square_attacked(self, opponent, &king_square);
        if !is_legal {
            //println!("Move is illegal, unmaking");
            self.unmake();
            //println!("Board state after unmaking:");
            //self.print_board();
        }

        // When running in debug mode, check the incrementally updated
        // values such as Zobrist key and meterial count.
        assert!(check_incrementals(self));

        // Report if the move was legal or not.
        is_legal
    }
}

/*** ================================================================================ ***/

// Unmake() reverses the last move. The game state is restored by popping it
// from the history array, all variables at once.
impl Board {
    #[cfg_attr(debug_assertions, inline(never))]
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn unmake(&mut self) {
        self.game_state = self.history.pop();

        // Set "us" and "opponent"
        let us = self.game_state.side_to_move;
        let opponent = us ^ 1;

        // Dissect the move to undo
        let m = self.game_state.next_move;
        let piece = m.piece();
        let from = m.from();
        let to = m.to();
        let captured = m.captured();
        let promoted = m.promoted();
        let castling = m.castling();
        let en_passant = m.en_passant();

        // Moving backwards...
        if promoted == Pieces::NONE {
            reverse_move(self, us, piece, to.clone(), from);
        } else {
            remove_piece(self, us, promoted, to.clone());
            put_piece(self, us, Pieces::PAWN, from);
        }

        // The king's move was already undone as a normal move.
        // Now undo the correct castling rook move.
        if castling {
            match to {
                Square::G1 => reverse_move(self, us, Pieces::ROOK, Square::F1, Square::H1),
                Square::C1 => reverse_move(self, us, Pieces::ROOK, Square::D1, Square::A1),
                Square::G8 => reverse_move(self, us, Pieces::ROOK, Square::F8, Square::H8),
                Square::C8 => reverse_move(self, us, Pieces::ROOK, Square::D8, Square::A8),
                _ => panic!("Error: Reversing castling rook move."),
            };
        }

        // If a piece was captured, put it back onto the to-square
        if captured != Pieces::NONE {
            put_piece(self, opponent, captured, to.clone());
        }

        // If this was an e-passant move, put the opponent's pawn back
        if en_passant {
            put_piece(self, opponent, Pieces::PAWN, Square(to.0 ^ 8));
        }
    }
}

/*** Functions local to playmove.rs ====================================================== ***/

// unamke() pops the entire game history from a list at the beginning. This
// includes the zobrist key, and any other incrementally updated values,
// such as material count and PSQT evaluation. Because these values are
// recovered instantly, they don't have to be recalculated backward.
// Therefore, this module has its own remove_piece and put_piece functions
// that omit the undoing of incremental updates.

// Removes a piece from the board without Zobrist key updates.
fn remove_piece(board: &mut Board, side: Side, piece: Piece, square: Square) {
    board.pieces[piece][side] ^= square.clone().to_bb();
    //board.bb_side[side] ^= square.to_bb();
    board.piece_list[square.0] = Pieces::NONE;
}

// Puts a piece onto the board without Zobrist key updates.
fn put_piece(board: &mut Board, side: Side, piece: Piece, square: Square) {
    board.pieces[piece][side] |= square.clone().to_bb();
    //board.bb_side[side] |= square.to_bb();
    board.piece_list[square.0] = piece;
}

// Moves a piece from one square to another.
fn reverse_move(board: &mut Board, side: Side, piece: Piece, remove: Square, put: Square) {
    remove_piece(board, side, piece, remove);
    put_piece(board, side, piece, put);
}

// This function can be used to check if incrementally updated values are
// kept correctly correctly during make() and unmake(). If one of the
// values is found to be incorrect (= different as compared to that value
// being generated from scratch), the engine will panic. This function only
// runs in debug mode.

fn check_incrementals(board: &Board) -> bool {
    let from_scratch_key = board.init_zobrist_key();
    //let from_scratch_psqt = crate::evaluation::psqt::apply(board);
    let mut result = true;

    // Waterfall: only report first error encountered and skip any others.
    if result && from_scratch_key != board.game_state.zobrist_key {
        println!("Check Incrementals: Error in Zobrist key.");
        result = false;
    };

    // if result && from_scratch_psqt.0 != board.game_state.psqt[Sides::WHITE] {
    //     println!("Check Incrementals: Error in PSQT for white.");
    //     result = false;
    // };

    // if result && from_scratch_psqt.1 != board.game_state.psqt[Sides::BLACK] {
    //     println!("Check Incrementals: Error in PSQT for black.");
    //     result = false;
    // };

    result
}
