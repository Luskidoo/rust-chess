use std::ops::{BitOr, BitOrAssign};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub const empty: BitBoard = BitBoard(0u64);
    pub fn set_bit(mut self, x: BitBoard) -> BitBoard {
        self | x
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
        *self |= rhs;
    }
}