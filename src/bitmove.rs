use crate::sq::*;

#[derive(Copy, Clone)]
pub struct BitMove {
    data: u16,
}

impl BitMove {
    #[inline(always)]
    pub const fn new(flag_bits: u16, src: SQ, dst: SQ) -> BitMove {
        BitMove {
            data: (flag_bits << 12) | src.0 as u16 | ((dst.0 as u16) << 6),
        }
    }

    #[inline]
    pub const fn null() -> Self {
        BitMove { data: 0 }
    }
}