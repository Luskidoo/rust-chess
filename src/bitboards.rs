#[derive(Copy, Clone)]
pub struct BitMasks {
    pub SetMask: [u64; 64],
    pub ClearMask: [u64; 64],
}

impl BitMasks {
    pub fn init_masks() -> BitMasks {
        let mut set_mask: [u64; 64] = [0; 64];
        let mut clear_mask: [u64; 64] = [0; 64];
        for i in 0..64 {
            set_mask[i] |= (1 << i);
            clear_mask[i] = !set_mask[i];
        }
        BitMasks {
            SetMask: set_mask,
            ClearMask: clear_mask,
        }
    }
}

pub fn set_bit(mut bb: u64, sq: usize, mask: BitMasks) -> u64 {
    bb |= mask.SetMask[sq];
    return bb
}

pub fn clear_bit(mut bb: u64, sq: usize, mask: BitMasks) -> u64 {
    bb |= mask.ClearMask[sq];
    return bb
}