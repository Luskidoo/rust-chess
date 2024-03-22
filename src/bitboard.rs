use std::ops::{BitOr, BitOrAssign, Shl, Shr, BitAnd, Not, Add, Mul, Sub};
use crate::board::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub const empty: BitBoard = BitBoard(0u64);
    pub const rank4: BitBoard = BitBoard(0x00000000FF000000);
    pub const rank5: BitBoard = BitBoard(0x000000FF00000000);
    pub const k1: BitBoard = BitBoard(0x5555555555555555); //  -1/3   
    pub const k2: BitBoard = BitBoard(0x3333333333333333); //  -1/5   
    pub const k4: BitBoard = BitBoard(0x0f0f0f0f0f0f0f0f); //  -1/17  
    pub const kf: BitBoard = BitBoard(0x0101010101010101); //  -1/255 
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
}

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