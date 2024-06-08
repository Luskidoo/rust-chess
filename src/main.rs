mod bitboard;
mod board;
mod sq;
//mod fen;
mod defs;
mod misc;
mod movegen;
mod bitmove;
mod movelist;
use crate::movegen::*;
use crate::bitmove::*;
use crate::movelist::*;
//use crate::fen::*;
use board::*;
use crate::bitboard::*;
//use fen::*;



fn get_piece() {
    todo!();
}

fn from_fen(fen: String, bb: BitBoard) -> BitBoard {
    todo!()
}

fn main() {
    let mut board = Board::new();
    let init_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let kiwi_fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
    let fen_result = board.fen_read(Some(init_fen));
    let mut list = MoveList::default();
    let moves = generate_all_moves(board, &mut list);
    println!("{} moves", list.count);
    //println!("Move count {}", list.count);
    //println!("{:?}", board.knights[0]);
    //println!("{:?}", BitBoard::knight_attacks(BitBoard(33792)));
}