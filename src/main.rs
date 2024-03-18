mod bitboard;
use crate::bitboard::*;

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


fn bitScanForwardWithReset(mut bb: u64) -> u64 { // also called dropForward
    let idx = bitScanForward(bb);
    bb &= bb - 1; // reset bit outside
    return idx;
}

fn bitScanForward(bb: u64) -> u64 {
    let debruijn64: u64 = 0x03f79d71b4cb0a89u64;
    return index64[(((bb ^ (bb.wrapping_sub(1))).wrapping_mul(debruijn64).wrapping_shr(58))) as usize].try_into().unwrap();
}

fn main() {
    let mut bb : BitBoard = BitBoard(0b11111111000000000000000000000000000000000000000000000000);

    println!("{}", bitScanForwardWithReset(bb.0));
}