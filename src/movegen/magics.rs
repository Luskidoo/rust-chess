use crate::{sq, BitBoard};
use rand::{rngs::ThreadRng, Rng};

use super::{MoveGenerator, SQ};

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

impl MoveGenerator {

    pub fn rook_mask(sq: u8) -> BitBoard {
      let rank = sq/8;
      let file = sq % 8;
      let mut result = BitBoard(0);
      for r in (rank + 1)..=6 {
        result |= BitBoard(1u64 << (file + r*8));
      }
      
      if rank >= 1 {
        for r in (1..(rank - 1)).rev() {
          result |= BitBoard(1u64 << ((file + r*8)))
        }
      }
      
      
      for f in (file + 1)..=6 {
        result |= BitBoard(1u64 << (f + rank*8));
      }

      if file >= 1 {
        for f in (1..(file - 1)).rev() {
            result |= BitBoard(1u64.wrapping_shl((f + rank.wrapping_mul(8)) as u32))
          }
      }
      result
    }

    pub fn bishop_mask(sq: u8) -> BitBoard {
        let rank = sq/8;
        let file = sq % 8;
        let mut result = BitBoard(0);
        let mut r = rank + 1;
        let mut f = file + 1;
        while r <= 6 && f <= 6 {
          result |= BitBoard(1u64 << (f + r*8));
          r += 1;
          f += 1;
        }

        if file >= 1 {
          r = rank + 1;
          f = file - 1;
          while r <= 6 && f >= 1 {
            result |= BitBoard(1u64 << (f + r*8));
            r += 1;
            f -= 1;
          }
        }
        
        if rank >= 1 {
          r = rank - 1;
          f = file + 1;
          while r >= 1 && f <= 6 {
            result |= BitBoard(1u64 << (f + r*8));
            r -= 1;
            f += 1;
          }
        }

        if rank >= 1 && file >= 1 {
          r = rank - 1;
          f = file - 1;
          while r >= 1 && f >= 1 {
            result |= BitBoard(1u64 << (f + r*8));
            r -= 1;
            f -= 1;
          }
        }
        result
    }

    fn rook_attacks(sq: u8, block: BitBoard) -> BitBoard {
      let rank = sq/8;
      let file = sq % 8;
      let mut result = BitBoard(0);

      for r in (rank + 1)..=7 {
        result |= BitBoard(1u64 << (file + r*8));
        if (block & BitBoard(1u64 << (file + r*8))) == BitBoard(0xffffffffffffffff) {
          break;  
        }
      };

      if rank >= 2 {
        for r in (0..(rank - 1)).rev() {
          result |= BitBoard(1u64 << (file + r*8));
          if (block & BitBoard(1u64 << (file + r*8))) == BitBoard(0xffffffffffffffff) {
            break;  
          }
        }
      }
      
      for f in (file + 1)..=7 {
        result |= BitBoard(1u64 << (f + rank*8));
        if (block & BitBoard(1u64 << (f + rank*8))) == BitBoard(0xffffffffffffffff) {
          break;  
        }
      }

      if file >= 2 {
        for f in (0..(file - 1)).rev() {
          result |= BitBoard(1u64 << (f + rank*8));
          if (block & BitBoard(1u64 << (f + rank*8))) == BitBoard(0xffffffffffffffff) {
            break;  
          }
        }
      }

      result
    }

    fn bishop_attacks(sq: u8, block: BitBoard) -> BitBoard {
      let rank = sq/8;
      let file = sq % 8;
      let mut result = BitBoard(0);
      let mut r: u64 = (rank + 1).into();
      let mut f: u64 = (file + 1).into();
      
      while r <= 7 && f <= 7 {
        result |= BitBoard(1u64 << (f + r*8));
        r += 1;
        f += 1;
        if (block & BitBoard(1u64 << ((file as u64) + r*8))) == BitBoard(0xffffffffffffffff) {
          break;  
        }
      }

      r = (rank + 1).into();
      f = (file - 1).into();
      while r <= 7 && f >= 0 {
        result |= BitBoard(1u64 << (f + r*8));
        r += 1;
        f -= 1;
        if (block & BitBoard(1u64 << ((file as u64) + r*8))) == BitBoard(0xffffffffffffffff) {
          break;  
        }
      }

      r = (rank - 1).into();
      f = (file + 1).into();
      while r >= 0 && f <= 7 {
        result |= BitBoard(1u64 << (f + r*8));
        r -= 1;
        f += 1;
        if (block & BitBoard(1u64 << ((file as u64) + r*8))) == BitBoard(0xffffffffffffffff) {
          break;  
        }
      }

      r = (rank - 1).into();
      f = (file - 1).into();
      while r >= 0 && f >= 0 {
        result |= BitBoard(1u64 << (f + r*8));
        r -= 1;
        f += 1;
        if (block & BitBoard(1u64 << ((file as u64) + r*8))) == BitBoard(0xffffffffffffffff) {
          break;  
        }
      }
      result
    }

    fn transform(bb: BitBoard, magic: BitBoard, bits: u64) -> BitBoard {
      BitBoard(bb.0.wrapping_mul(magic.0)) >> BitBoard(64 - bits)
    }

    fn index_to_u64(index: usize, bits: u64, mut m: BitBoard) -> BitBoard {
      let mut result = BitBoard(0);
      let mut j: u64 = 0;
      for i in 0..bits {
        j = BitBoard::next(&mut m);
        if index & (1 << i) == 1 {
          result |= BitBoard(1u64 << j);
        }
      }
      result
    }

    fn find_magic(sq: u8, m: u64, is_rook: bool, mut rng: ThreadRng) -> BitBoard {
      let mut b: [BitBoard; 4096] = [BitBoard(0); 4096];
      let mut a: [BitBoard; 4096] = [BitBoard(0); 4096];
      let mut used: [BitBoard; 4096] = [BitBoard(0); 4096];
      let mut j: BitBoard = BitBoard(0);
      let rook_mask = Self::rook_mask(sq);
      let bishop_mask = Self::bishop_mask(sq);
      let mask = if is_rook { rook_mask } else { bishop_mask };
      let n = mask.pop_count();

      for i in 0..(1 << n) {
        b[i] = Self::index_to_u64(i, n.into(), mask);
        a[i] = if is_rook { Self::rook_attacks(sq, b[i]) } else { Self::bishop_attacks(sq, b[i]) };
      }
    
      let mut fail = 0;
      for k in 0..100000000 {
        //println!("k: {}", k);
        let magic = BitBoard(rng.gen::<u64>());
        if (BitBoard(mask.0.wrapping_mul(magic.0)) & BitBoard(0xFF00000000000000u64)).pop_count() < 6 {
          continue;
        }
        for i in 0..(1 << n) {
          //println!("{} out of {}", i, 1 << n);
          j = Self::transform(b[i], magic, m);
          if used[j.0 as usize] == BitBoard(0) {
            used[j.0 as usize] = a[i];
          }
          else if used[j.0 as usize] != a[i] {
            fail = 1;
            println!("Failed");
          }
        }
        if fail == 0 {
          return magic;
        }
      }
      BitBoard(0)
    }

    pub fn generate_magics() {
      let mut rng = rand::thread_rng();
      println!("Rook magics");
      for square in 0..64 {
        println!("{}, {:?}", square, Self::find_magic(square, r_bits[square as usize], true, rng.clone()))
      }
    }
}