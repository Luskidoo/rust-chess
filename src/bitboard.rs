use std::ops::{BitOr, BitXor, BitXorAssign, BitOrAssign, Shl, Shr, BitAnd, BitAndAssign, Not, Add, Mul, Sub};
use crate::board::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub const empty: BitBoard = BitBoard(0u64);
    pub const rank4: BitBoard = BitBoard(0x00000000FF000000);
    pub const rank5: BitBoard = BitBoard(0x000000FF00000000);
    pub const k1: BitBoard = BitBoard(0x5555555555555555); //  -1/3   
    pub const k2: BitBoard = BitBoard(0x3333333333333333); //  -1/5   
    pub const k4: BitBoard = BitBoard(0x0f0f0f0f0f0f0f0f); //  -1/17  
    pub const kf: BitBoard = BitBoard(0x0101010101010101); //  -1/255
    pub const not_a_file: BitBoard = BitBoard(0xfefefefefefefefe);
    pub const not_h_file: BitBoard = BitBoard(0x7f7f7f7f7f7f7f7f);  
    pub fn set_bit(mut self, x: BitBoard) -> BitBoard {
        self | x
    }

    pub fn south_one(mut self) -> Self {
        self >> BitBoard(8)
    }

    pub fn north_one(mut self) -> Self {
        self << BitBoard(8)
    }

    pub fn pop_count(mut self) -> u64 {
        self = self - ((self >> BitBoard(1)) & BitBoard::k1);
        self = (self & BitBoard::k2) + ((self >> BitBoard(2)) & BitBoard::k2);
        self = self + (self >> BitBoard(4)) & BitBoard::k4;
        self = BitBoard(self.0.wrapping_mul((BitBoard::kf).0)) >> BitBoard(56);
        self.0
    }

    pub fn knight_attacks(mut self) -> BitBoard {
        let l1: BitBoard = (self >> BitBoard(1)) & BitBoard(0x7f7f7f7f7f7f7f7f);
        let l2: BitBoard = (self >> BitBoard(2)) & BitBoard(0x3f3f3f3f3f3f3f3f);
        let r1: BitBoard = (self << BitBoard(1)) & BitBoard(0xfefefefefefefefe);
        let r2: BitBoard = (self << BitBoard(2)) & BitBoard(0xfcfcfcfcfcfcfcfc);
        let h1: BitBoard = l1 | r1;
        let h2: BitBoard = l2 | r2;
        BitBoard((h1.0 << 16) | (h1.0 >> 16) | (h2.0 << 8) | (h2.0 >> 8))
    }

    // pub fn bit_scan_forward_with_reset(mut bb: &BitBoard) -> u64 { // also called dropForward
    //     let idx = Self::bit_scan_forward(bb);
    //     bb &= bb - BitBoard(1); // reset bit outside
    //     return idx;
    // }
    
    // fn bit_scan_forward(bb: &BitBoard) -> u64 {
    //     let debruijn64: u64 = 0x03f79d71b4cb0a89u64;
    //     return index64[(((bb ^ BitBoard((bb.0.wrapping_sub(1)))).0.wrapping_mul(debruijn64).wrapping_shr(58))) as usize].try_into().unwrap();
    // }

    pub fn next(bitboard: &mut BitBoard) -> u64 {
        let square: u64 = bitboard.0.trailing_zeros() as u64;
        *bitboard ^= BitBoard(1u64 << square);
        square
    }
}

const index64: [u64; 64] = [
    0, 47,  1, 56, 48, 27,  2, 60,
   57, 49, 41, 37, 28, 16,  3, 61,
   54, 58, 35, 52, 50, 42, 21, 44,
   38, 32, 29, 23, 17, 11,  4, 62,
   46, 55, 26, 59, 40, 36, 15, 53,
   34, 51, 20, 43, 31, 22, 10, 45,
   25, 39, 14, 33, 19, 30,  9, 24,
   13, 18,  8, 12,  7,  6,  5, 63
];

impl BitOr for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitor(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 | other.0)
    }
}

impl BitOrAssign for BitBoard {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl Shl for BitBoard {
    type Output = BitBoard;
    fn shl(self, rhs: Self) -> Self{
        BitBoard(self.0 << rhs.0)
    }
}

impl ~const Shl for BitBoard {
    type Output = BitBoard;
    fn shl(self, rhs: Self) -> Self{
        BitBoard(self.0 << rhs.0)
    }
}

impl Shr for BitBoard {
    type Output = BitBoard;
    fn shr(self, rhs: Self) -> Self{
        BitBoard(self.0 >> rhs.0)
    }
}

impl BitAnd for BitBoard {
    type Output = BitBoard;
    fn bitand(self, rhs: Self) -> Self{
        BitBoard(self.0 & rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitXor for BitBoard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}

impl Not for BitBoard {
    type Output = BitBoard;
    fn not(self) -> Self{
        BitBoard(!self.0)
    }
}

impl Add for BitBoard {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        BitBoard(self.0 + rhs.0)
    }
}

impl Mul for BitBoard {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        BitBoard(self.0 * rhs.0)
    }
}

impl Sub for BitBoard {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        BitBoard(self.0 - rhs.0)
    }
}