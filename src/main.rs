mod bitboard;
mod board;
mod sq;
//mod fen;
mod defs;
mod misc;
mod movegen;
mod bitmove;
mod movelist;

//mod extra;
use crate::bitmove::*;
use crate::movelist::*;
//use crate::fen::*;
use board::*;
use crate::bitboard::*;
//use crate::extra::magic::*;
//use fen::*;
use crate::movegen::MoveGenerator;

fn main() {
    let mut board = Board::new();
    let init_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let test_fen = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -";
    let max_moves = "R6R/3Q4/1Q4Q1/4Q3/2Q4Q/Q4Q2/pp1Q4/kBNN1KB1 w - - 0 1";
    // let kiwi_fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
    let fen_result = board.fen_read(Some(max_moves));
    let mut list = MoveList::default();
    let move_gen = MoveGenerator::new();
    move_gen.generate_all_moves(board, &mut list);
    println!("{} moves", list.count);
    //println!("{:?}", MoveGenerator::rook_mask(0));
    //println!("{:?}", MoveGenerator::rook_attacks(0, BitBoard(258)));

}