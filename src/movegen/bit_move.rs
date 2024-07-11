use crate::defs::{Piece, Square};

/* "Shift" is an enum which contains the number of bits that needed to be shifted to store
 * move data in a specific place within the u64 integer. This makes sure that, should the
 * format change, the location needs to be changed only within the integer. */
pub struct Shift;
impl Shift {
     pub const PIECE: usize = 0;
     pub const FROM_SQ: usize = 3;
     pub const TO_SQ: usize = 9;
     pub const CAPTURE: usize = 15;
     pub const PROMOTION: usize = 18;
     pub const EN_PASSANT: usize = 21;
     pub const DOUBLE_STEP: usize = 22;
     pub const CASTLING: usize = 23;
     pub const SORTSCORE: usize = 24;
}

#[derive(Copy, Clone, PartialEq)]
pub struct Move {
    data: usize,
}

// These functions decode the move data.
impl Move {
    pub fn new(data: usize) -> Self {
        Self { data }
    }

    pub fn piece(&self) -> Piece {
        ((self.data >> Shift::PIECE as u64) & 0x7) as Piece
    }

    pub fn from(&self) -> Square {
        Square((self.data >> Shift::FROM_SQ as u64) & 0x3F)
    }

    pub fn to(&self) -> Square {
        Square((self.data >> Shift::TO_SQ as u64) & 0x3F)
    }

    pub fn captured(&self) -> Piece {
        ((self.data >> Shift::CAPTURE as u64) & 0x7) as Piece
    }

    pub fn promoted(&self) -> Piece {
        ((self.data >> Shift::PROMOTION as u64) & 0x7) as Piece
    }

    pub fn en_passant(&self) -> bool {
        ((self.data >> Shift::EN_PASSANT as u64) & 0x1) as u8 == 1
    }

    pub fn double_step(&self) -> bool {
        ((self.data >> Shift::DOUBLE_STEP as u64) & 0x1) as u8 == 1
    }

    pub fn castling(&self) -> bool {
        ((self.data >> Shift::CASTLING as u64) & 0x1) as u8 == 1
    }
    #[inline]
    pub const fn null() -> Self {
        Self { data: 0 }
    }
}