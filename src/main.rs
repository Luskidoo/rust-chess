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


fn get_piece() {
    todo!();
}

fn from_fen(fen: String, bb: BitBoard) -> BitBoard {
    todo!()
}

fn main() {
    // let mut board = Board::new();
    // let init_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    // let kiwi_fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
    // let fen_result = board.fen_read(Some(init_fen));
    // let mut list = MoveList::default();
    //let move_gen = MoveGenerator::new();
    // move_gen.generate_all_moves(board, &mut list);
    // println!("{} moves", list.count);
    let sq: u8 = 0;
    println!("{:?}", MoveGenerator::rook_mask(sq));
    println!("{:?}", MoveGenerator::bishop_mask(sq));
}