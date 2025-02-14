use std::fmt;

use crate::bitboard::*;

pub type Side = usize;
pub type Piece = usize;
pub struct Pieces;
impl Pieces {
    pub const KING: Piece = 0;
    pub const QUEEN: Piece = 1;
    pub const ROOK: Piece = 2;
    pub const BISHOP: Piece = 3;
    pub const KNIGHT: Piece = 4;
    pub const PAWN: Piece = 5;
    pub const NONE: Piece = 6;
}

#[derive(Copy, Clone, PartialEq)]
pub struct Sides;
impl Sides {
    pub const WHITE: Side = 0;
    pub const BLACK: Side = 1;
    pub const BOTH: Side = 2;
}

pub const FEN_START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const FEN_KIWIPETE_POSITION: &str =
    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

pub struct NrOf;
impl NrOf {
    pub const PIECE_TYPES: usize = 6;
    pub const CASTLING_PERMISSIONS: usize = 16; // 0-15
    pub const SQUARES: usize = 64;
    pub const FILES: usize = 8;
    pub const RANKS: usize = 8;
}

pub struct Castling;
impl Castling {
    pub const WK: BitBoard = BitBoard(1);
    pub const WQ: BitBoard = BitBoard(2);
    pub const BK: BitBoard = BitBoard(4);
    pub const BQ: BitBoard = BitBoard(8);
    pub const ALL: BitBoard = BitBoard(15);
}

#[derive(PartialEq, PartialOrd, Clone, Eq)]
pub struct Square(pub usize);
impl Square {
    // White side squares that are important for castling
    pub const A1: Self = Square(0);
    pub const B1: Self = Square(1);
    pub const C1: Self = Square(2);
    pub const D1: Self = Square(3);
    pub const E1: Self = Square(4);
    pub const F1: Self = Square(5);
    pub const G1: Self = Square(6);
    pub const H1: Self = Square(7);

    // Black side squares that are important for castling
    pub const A8: Self = Square(56);
    pub const B8: Self = Square(57);
    pub const C8: Self = Square(58);
    pub const D8: Self = Square(59);
    pub const E8: Self = Square(60);
    pub const F8: Self = Square(61);
    pub const G8: Self = Square(62);
    pub const H8: Self = Square(63);

    // White EP-squares start/end
    pub const A3: Self = Square(16);
    pub const H3: Self = Square(23);

    // Black EP-squares start/end
    pub const A6: Self = Square(40);
    pub const H6: Self = Square(47);

    pub fn to_bb(&self) -> BitBoard {
        BitBoard(1 << self.0)
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Files;
impl Files {
    pub const A: usize = 0;
    pub const B: usize = 1;
    pub const G: usize = 6;
    pub const H: usize = 7;
}

pub struct Ranks;
impl Ranks {
    pub const R1: usize = 0;
    pub const R2: usize = 1;
    pub const R4: usize = 3;
    pub const R5: usize = 4;
    pub const R7: usize = 6;
    pub const R8: usize = 7;
}

pub const MAX_MOVE_RULE: u8 = 100; // 50/75 move rule
pub const MAX_GAME_MOVES: usize = 2048;
pub const MAX_LEGAL_MOVES: usize = 255;

#[rustfmt::skip]
pub const SQUARE_NAME: [&str; 64] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8"
];

pub const PIECE_CHAR_SMALL: [&str; NrOf::PIECE_TYPES + 1] = ["k", "q", "r", "b", "n", "", ""];

// const fn init_bb_ranks() -> [BitBoard; 8] {
//     let BB_RANK_1: BitBoard = BitBoard(0xFF);
//     let mut bb_ranks = [BitBoard(0); 8];
//     let mut i = 0;

//     while i < 8 {
//         bb_ranks[i] = BB_RANK_1 << BitBoard((i * 8) as u64);
//         i += 1;
//     }

//     bb_ranks
// }

// pub const BB_RANKS: [BitBoard; 8] = init_bb_ranks();