use crate::bitboard::*;
use crate::board::*;
use crate::bitmove::*;
use crate::movelist::*;
use crate::sq::*;
use crate::defs::{Sides};

fn w_pawn_single_push(bb: BitBoard, empty: BitBoard) -> BitBoard {
    bb.north_one() & empty
}

fn w_pawn_double_push(bb: BitBoard, empty: BitBoard) -> BitBoard {
    let single_pushes = w_pawn_single_push(bb, empty);
    single_pushes.north_one() & empty & BitBoard::rank4
}

fn b_pawn_single_push(bb: BitBoard, empty: BitBoard) -> BitBoard {
    bb.south_one() & empty
}

fn b_pawn_double_push(bb: BitBoard, empty: BitBoard) -> BitBoard {
    let single_pushes = b_pawn_single_push(bb, empty);
    single_pushes.south_one() & empty & BitBoard::rank5
}

fn w_knight_moves(mut bb: BitBoard, w_empty: BitBoard, list: &mut MoveList) {
    while bb > BitBoard(0) {
        let from = BitBoard::next(&mut bb);
        println!("From {}", from);
        let from_bb: BitBoard = BitBoard(1) << BitBoard(from);
        println!("From bb {:?}", from_bb);
        let mut to_bb: BitBoard = bb.knight_attacks() & w_empty;
        println!("To bb {:?}", to_bb);
        while to_bb > BitBoard(0) {
            let to = BitBoard::next(&mut to_bb);
            list.push(BitMove::new(0, SQ(from as u8), SQ(to as u8)));
            println!("Knight move from {} to {}", from, to);
        }
    }
    //bb.knight_attacks() & w_empty
}

pub fn generate_w_pawn_moves(board: Board, list: &mut MoveList) {
    let mut w_pawns = board.pawns[Sides::WHITE];
    let empty_bb: BitBoard = board.empty_squares();
    while w_pawns > BitBoard(0) {
        let from = BitBoard::next(&mut w_pawns);
        let from_bb: BitBoard = BitBoard(1) << BitBoard(from);
        let mut to_bb: BitBoard = w_pawn_single_push(from_bb, empty_bb) | w_pawn_double_push(from_bb, empty_bb);
        while to_bb > BitBoard(0) {
            let to = BitBoard::next(&mut to_bb);
            list.push(BitMove::new(0, SQ(from as u8), SQ(to as u8)));
            //println!("Pawn move from {} to {}", from, to);
        }
    }
}

pub fn generate_b_pawn_moves(board: Board) {
    let mut b_pawns = board.pawns[Sides::WHITE];
    let empty_bb: BitBoard = board.empty_squares();
    while b_pawns > BitBoard(0) {
        let from = BitBoard::next(&mut b_pawns);
        let from_bb: BitBoard = BitBoard(1) << BitBoard(from);
        let mut to_bb: BitBoard = b_pawn_single_push(from_bb, empty_bb) | b_pawn_double_push(from_bb, empty_bb);
        while to_bb > BitBoard(0) {
            let to = BitBoard::next(&mut to_bb);
            println!("Move from {} to {}", from, to);
        }
    }
}

pub fn generate_knight_moves(board: Board, list: &mut MoveList) {
    let w_knights = board.knights[Sides::WHITE];
    let w_empty = board.white_empty();
    w_knight_moves(w_knights, w_empty, list);
}

pub fn generate_all_moves(board: Board, list: &mut MoveList) {
    generate_w_pawn_moves(board, list);
    generate_knight_moves(board, list);
}