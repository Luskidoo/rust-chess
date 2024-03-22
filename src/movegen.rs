use crate::bitboard::*;
use crate::board::*;
use crate::defs::{Sides};

fn w_pawn_single_push(board: Board) -> BitBoard {
    board.pawns[Sides::WHITE].north_one() & board.empty_squares()
}

fn w_pawn_double_push(board: Board) -> BitBoard {
    let single_pushes = w_pawn_single_push(board);
    single_pushes.north_one() & board.empty_squares() & BitBoard::rank4
}

fn b_pawn_single_push(board: Board) -> BitBoard {
    board.pawns[Sides::WHITE].south_one() & board.empty_squares()
}

fn b_pawn_double_push(board: Board) -> BitBoard {
    let single_pushes = w_pawn_single_push(board);
    single_pushes.south_one() & board.empty_squares() & BitBoard::rank5
}

fn w_knight_moves(board: Board) -> BitBoard {
    board.knights[Sides::WHITE].knight_attacks() & board.white_empty()
}

pub fn generate_pawn_moves(board: Board) -> BitBoard {
    let w_pawns = w_pawn_single_push(board) | w_pawn_double_push(board);
    w_pawns
}

pub fn generate_knight_moves(board: Board) -> BitBoard {
    w_knight_moves(board)
}