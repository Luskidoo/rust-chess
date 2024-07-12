use std::ops::{Add, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, Not, Shl, ShlAssign, Shr, Sub};
use crate::{board::*, defs::Square};

pub type BitBoard =  u64;

pub const EMPTY: BitBoard = 0u64;
pub const RANK4: BitBoard = 0x00000000FF000000;
pub const RANK5: BitBoard = 0x000000FF00000000;
pub const K1: BitBoard = 0x5555555555555555; //  -1/3   
pub const K2: BitBoard = 0x3333333333333333; //  -1/5   
pub const K4: BitBoard = 0x0f0f0f0f0f0f0f0f; //  -1/17  
pub const KF: BitBoard = 0x0101010101010101; //  -1/255
pub const NOT_A_FILE: BitBoard = 0xfefefefefefefefe;
pub const NOT_H_FILE: BitBoard = 0x7f7f7f7f7f7f7f7f;  

// pub fn set_bit(x: BitBoard) -> BitBoard {
//     self | x
// }

pub fn south_one(bb: BitBoard) -> BitBoard {
    bb >> 8
}

pub fn north_one(bb: BitBoard) -> BitBoard {
    bb << 8
}

pub fn pop_count(bb: BitBoard) -> u32 {
    bb.count_ones()
}

pub fn east_one(bb: BitBoard) -> BitBoard {
    bb << 1 & NOT_A_FILE
}

pub fn west_one(bb: BitBoard) -> BitBoard {
    bb >> 1 & NOT_H_FILE
}



pub fn knight_attacks(bb: BitBoard) -> BitBoard {
    let l1: BitBoard = (bb >> 1) & 0x7f7f7f7f7f7f7f7f;
    let l2: BitBoard = (bb >> 2) & 0x3f3f3f3f3f3f3f3f;
    let r1: BitBoard = (bb << 1) & 0xfefefefefefefefe;
    let r2: BitBoard = (bb << 2) & 0xfcfcfcfcfcfcfcfc;
    let h1: BitBoard = l1 | r1;
    let h2: BitBoard = l2 | r2;
    (h1 << 16) | (h1 >> 16) | (h2 << 8) | (h2 >> 8)
}

pub fn next(bb: &mut BitBoard) -> Square {
    let square: u64 = bb.trailing_zeros() as u64;
    //println!("Bitboard before {:?}", bitboard);
    *bb ^= 1u64 << square;
    //println!("Bitboard after {:?}", bitboard);
    Square(square as usize)
}

// impl std::fmt::Debug for BitBoard {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "BitBoard(0x{:016x})", self.0)
//     }
// }

// impl std::fmt::Display for BitBoard {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         for rank in (0..8).rev() {
//             for file in 0..8 {
//                 let sq = rank * 8 + file;
//                 write!(f, "{} ", if self.0 & (1 << sq) != 0 { "1" } else { "0" })?;
//             }
//             writeln!(f)?;
//         }
//         Ok(())
//     }
// }