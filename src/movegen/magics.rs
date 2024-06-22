use crate::{sq, BitBoard};
use rand::{rngs::ThreadRng, Rng};

use super::{MoveGenerator, SQ};

#[derive(Copy, Clone)]
pub struct Magic {
    pub mask: BitBoard,
    pub shift: u8,
    pub offset: u64,
    pub nr: u64,
}

impl Magic {
    pub fn new() -> Self {
        Self {
            mask: BitBoard(0),
            shift: 0,
            offset: 0,
            nr: 0
        }
    }

    pub fn get_index(&self, occupancy: BitBoard) -> usize {
      let blockerboard = occupancy & self.mask;
      ((blockerboard.0.wrapping_mul(self.nr) >> self.shift) + self.offset) as usize
  }
}

static bit_table: [u64; 64] = [
  63, 30, 3, 32, 25, 41, 22, 33, 15, 50, 42, 13, 11, 53, 19, 34, 61, 29, 2,
  51, 21, 43, 45, 10, 18, 47, 1, 54, 9, 57, 0, 35, 62, 31, 40, 4, 49, 5, 52,
  26, 60, 6, 23, 44, 46, 27, 56, 16, 7, 39, 48, 24, 59, 14, 12, 55, 38, 28,
  58, 20, 37, 17, 36, 8
];

static r_bits: [u64; 64] = [
  12, 11, 11, 11, 11, 11, 11, 12,
  11, 10, 10, 10, 10, 10, 10, 11,
  11, 10, 10, 10, 10, 10, 10, 11,
  11, 10, 10, 10, 10, 10, 10, 11,
  11, 10, 10, 10, 10, 10, 10, 11,
  11, 10, 10, 10, 10, 10, 10, 11,
  11, 10, 10, 10, 10, 10, 10, 11,
  12, 11, 11, 11, 11, 11, 11, 12
];

static b_bits: [u64; 64] = [
  6, 5, 5, 5, 5, 5, 5, 6,
  5, 5, 5, 5, 5, 5, 5, 5,
  5, 5, 7, 7, 7, 7, 5, 5,
  5, 5, 7, 9, 9, 7, 5, 5,
  5, 5, 7, 9, 9, 7, 5, 5,
  5, 5, 7, 7, 7, 7, 5, 5,
  5, 5, 5, 5, 5, 5, 5, 5,
  6, 5, 5, 5, 5, 5, 5, 6
];

const BIT_TABLE: [u64; 64] = [
    63, 30, 3, 32, 25, 41, 22, 33, 15, 50, 42, 13, 11, 53, 19, 34, 61, 29, 2,
    51, 21, 43, 45, 10, 18, 47, 1, 54, 9, 57, 0, 35, 62, 31, 40, 4, 49, 5, 52,
    26, 60, 6, 23, 44, 46, 27, 56, 16, 7, 39, 48, 24, 59, 14, 12, 55, 38, 28,
    58, 20, 37, 17, 36, 8
];

fn pop_1st_bit(bb: &mut u64) -> u64 {
    let b = *bb ^ (*bb - 1);
    let fold = ((b & 0xffffffff) ^ (b >> 32)) as u32;
    *bb &= *bb - 1;
    BIT_TABLE[((fold.wrapping_mul(0x783a9b23)) >> 26) as usize]
}

fn random_u64() -> u64 {
  let mut rng = rand::thread_rng();
  let u1: u64 = rng.gen::<u16>() as u64;
  let u2: u64 = rng.gen::<u16>() as u64;
  let u3: u64 = rng.gen::<u16>() as u64;
  let u4: u64 = rng.gen::<u16>() as u64;
  u1 | (u2 << 16) | (u3 << 32) | (u4 << 48)
}

fn random_u64_fewbits() -> u64 {
  random_u64() & random_u64() & random_u64()
}

impl MoveGenerator {

    pub fn rook_mask(sq: u8) -> BitBoard {
      let mut result = 0u64;
      let rk = sq / 8;
      let fl = sq % 8;
      for r in (rk + 1)..=6 { result |= 1u64 << (fl + r * 8); }
      for r in (1..rk).rev() { result |= 1u64 << (fl + r * 8); }
      for f in (fl + 1)..=6 { result |= 1u64 << (f + rk * 8); }
      for f in (1..fl).rev() { result |= 1u64 << (f + rk * 8); }
      BitBoard(result)
    }

    pub fn bishop_mask(sq: u8) -> BitBoard {
      let mut result = 0u64;
      let rk = sq / 8;
      let fl = sq % 8;
      for (r, f) in (rk + 1..=6).zip(fl + 1..=6) { result |= 1u64 << (f + r * 8); }
      for (r, f) in (rk + 1..=6).zip((1..fl).rev()) { result |= 1u64 << (f + r * 8); }
      for (r, f) in (1..rk).rev().zip(fl + 1..=6) { result |= 1u64 << (f + r * 8); }
      for (r, f) in (1..rk).rev().zip((1..fl).rev()) { result |= 1u64 << (f + r * 8); }
      BitBoard(result)
    }

    pub fn rook_attacks(sq: u8, block: BitBoard) -> BitBoard {
      let mut result = 0u64;
      let rk = sq / 8;
      let fl = sq % 8;
      for r in rk + 1..=7 {
          result |= 1u64 << (fl + r * 8);
          if block.0 & (1u64 << (fl + r * 8)) != 0 { break; }
      }
      for r in (0..rk).rev() {
          result |= 1u64 << (fl + r * 8);
          if block.0 & (1u64 << (fl + r * 8)) != 0 { break; }
      }
      for f in fl + 1..=7 {
          result |= 1u64 << (f + rk * 8);
          if block.0 & (1u64 << (f + rk * 8)) != 0 { break; }
      }
      for f in (0..fl).rev() {
          result |= 1u64 << (f + rk * 8);
          if block.0 & (1u64 << (f + rk * 8)) != 0 { break; }
      }
      BitBoard(result)
    }

    pub fn bishop_attacks(sq: u8, block: BitBoard) -> BitBoard {
      let mut result = 0u64;
      let rk = sq / 8;
      let fl = sq % 8;
      for (r, f) in (rk + 1..=7).zip(fl + 1..=7) {
          result |= 1u64 << (f + r * 8);
          if block.0 & (1u64 << (f + r * 8)) != 0 { break; }
      }
      for (r, f) in (rk + 1..=7).zip((0..fl).rev()) {
          result |= 1u64 << (f + r * 8);
          if block.0 & (1u64 << (f + r * 8)) != 0 { break; }
      }
      for (r, f) in (0..rk).rev().zip(fl + 1..=7) {
          result |= 1u64 << (f + r * 8);
          if block.0 & (1u64 << (f + r * 8)) != 0 { break; }
      }
      for (r, f) in (0..rk).rev().zip((0..fl).rev()) {
          result |= 1u64 << (f + r * 8);
          if block.0 & (1u64 << (f + r * 8)) != 0 { break; }
      }
    BitBoard(result)
    }

    fn transform(bb: BitBoard, magic: BitBoard, bits: u64) -> usize {
      (bb.0.wrapping_mul(magic.0) >> (64 - bits)) as usize
    }

    fn index_to_u64(index: usize, bits: u64, mut m: BitBoard) -> BitBoard {
      let mut result = BitBoard(0);
      let mut j: u64 = 0;
      for i in 0..bits {
        j = pop_1st_bit(&mut m.0);
        if index & (1 << i) != 0 {
          result |= BitBoard(1u64 << j);
        }
      }
      result
    }

    fn find_magic(sq: u8, m: u64, is_rook: bool, mut rng: ThreadRng) -> BitBoard {
      let mut b: [BitBoard; 4096] = [BitBoard(0); 4096];
      let mut a: [BitBoard; 4096] = [BitBoard(0); 4096];
      let mut used: [BitBoard; 4096] = [BitBoard(0); 4096];
      let mut j: usize = 0;
      let rook_mask = Self::rook_mask(sq);
      let bishop_mask = Self::bishop_mask(sq);
      let mask = if is_rook { rook_mask } else { bishop_mask };
      let n = mask.pop_count();

      for i in 0..(1 << n) {
        b[i] = Self::index_to_u64(i, n.into(), mask);
        a[i] = if is_rook { Self::rook_attacks(sq, b[i]) } else { Self::bishop_attacks(sq, b[i]) };
      }
    
      for k in 0..100000000 {
        // if k % 100000 == 0 {
        //   println!("k: {}", k);
        // }
        let magic = BitBoard(random_u64_fewbits());
        if BitBoard(mask.0.wrapping_mul(magic.0) & 0xFF00000000000000u64).pop_count() < 6 {
          continue;
        }
        used.fill(BitBoard(0));
        let mut fail = false;
        for i in 0..(1 << n) {
          //println!("{} out of {}", i, 1 << n);
          j = Self::transform(b[i], magic, m);
          if used[j] == BitBoard(0) {
            used[j] = a[i];
          }
          else if used[j] != a[i] {
            fail = true;
            //println!("Failed");
          }
        }
        if !fail {
          return magic;
        }
      }
      BitBoard(0)
    }

    pub fn generate_magics(is_rook: bool) -> [BitBoard; 64] {
      let mut rng = rand::thread_rng();
      let mut magics = [BitBoard(0); 64];
      for sq in 0..64 {
        let magic = Self::find_magic(sq, r_bits[sq as usize], is_rook, rng.clone());
        println!("{}, {:?}", sq, magic);
        magics[sq as usize] = magic;
      }
      
      magics
    }

    pub fn blocker_boards(mask: BitBoard) -> Vec<BitBoard> {
      let d: BitBoard = mask;
      let mut bb_blocker_boards: Vec<BitBoard> = Vec::new();
      let mut n: BitBoard = BitBoard(0);

      // Carry-Rippler
      // https://www.chessprogramming.org/Traversing_Subsets_of_a_Set
      loop {
          bb_blocker_boards.push(n);
          n = BitBoard(n.0.wrapping_sub(d.0) & d.0);
          if n == BitBoard(0) {
              break;
          }
      }

      bb_blocker_boards
  }
}