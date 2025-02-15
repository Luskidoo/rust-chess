mod bitboard;
mod board;
mod sq;
//mod fen;
mod defs;
//mod misc;
mod movegen;
mod bitmove;
mod movelist;
mod perft;

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
    let test_fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    let max_moves = "R6R/3Q4/1Q4Q1/4Q3/2Q4Q/Q4Q2/pp1Q4/kBNN1KB1 w - - 0 1";
    let kiwi_fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
    let pos_4 = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
    let promotion = "n1n5/PPPk4/8/8/8/8/4Kppp/5N1N b - - 0 1";
    let fen_result = board.fen_read(Some(init_fen));
    let mut list = MoveList::default();
    let move_gen = MoveGenerator::new();
    perft::run(board, 2, move_gen);
    //println!("{:?}", MoveGenerator::rook_mask(0));
    //println!("{:?}", MoveGenerator::rook_attacks(0, BitBoard(258)));

}