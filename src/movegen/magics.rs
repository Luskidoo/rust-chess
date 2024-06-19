use crate::BitBoard;

use super::{MoveGenerator, SQ};

impl MoveGenerator {

    fn rook_mask(sq: u8) -> BitBoard {
        let rank = sq/8;
        let file = sq % 8;
        let mut result = BitBoard(0);
        for r in (rank + 1)..=6 {
            result |= BitBoard(1u64 << (file + r*8));
        }

        for r in (0..(rank - 1)).rev() {
            result |= BitBoard(1u64 << (file + r*8))
        }

        for f in (file + 1)..=6 {
            result |= BitBoard(1u64 << (f + rank*8));
        }

        for f in (0..(file - 1)).rev() {
            result |= BitBoard(1u64 << (f + rank*8))
        }
        result
    }

    fn bishop_mask(sq: u8) -> BitBoard {
        let rank = sq/8;
        let file = sq % 8;
        let mut result = BitBoard(0);
        for r in (rank + 1..=6) {
            for f in (file + 1..=6) {
                result |= BitBoard(1u64 << (f + r*8));
            }
        }

        for r in (rank + 1..=6) {
            for f in (0..(file - 1)).rev() {
                result |= BitBoard(1u64 << (f + r*8));
            }
        }

        for r in (0..(rank - 1)).rev() {
            for f in ((file  + 1)..=6) {
                result |= BitBoard(1u64 << (f + r*8));
            }
        }

        for r in (0..(rank - 1)).rev() {
            for f in (0..(file - 1)).rev() {
                result |= BitBoard(1u64 << (f + r*8));
            }
        }
        result
    }

    fn generate_magics() {
          
          uint64 ratt(int sq, uint64 block) {
            uint64 result = 0ULL;
            int rk = sq/8, fl = sq%8, r, f;
            for(r = rk+1; r <= 7; r++) {
              result |= (1ULL << (fl + r*8));
              if(block & (1ULL << (fl + r*8))) break;
            }
            for(r = rk-1; r >= 0; r--) {
              result |= (1ULL << (fl + r*8));
              if(block & (1ULL << (fl + r*8))) break;
            }
            for(f = fl+1; f <= 7; f++) {
              result |= (1ULL << (f + rk*8));
              if(block & (1ULL << (f + rk*8))) break;
            }
            for(f = fl-1; f >= 0; f--) {
              result |= (1ULL << (f + rk*8));
              if(block & (1ULL << (f + rk*8))) break;
            }
            return result;
          }
          
          uint64 batt(int sq, uint64 block) {
            uint64 result = 0ULL;
            int rk = sq/8, fl = sq%8, r, f;
            for(r = rk+1, f = fl+1; r <= 7 && f <= 7; r++, f++) {
              result |= (1ULL << (f + r*8));
              if(block & (1ULL << (f + r * 8))) break;
            }
            for(r = rk+1, f = fl-1; r <= 7 && f >= 0; r++, f--) {
              result |= (1ULL << (f + r*8));
              if(block & (1ULL << (f + r * 8))) break;
            }
            for(r = rk-1, f = fl+1; r >= 0 && f <= 7; r--, f++) {
              result |= (1ULL << (f + r*8));
              if(block & (1ULL << (f + r * 8))) break;
            }
            for(r = rk-1, f = fl-1; r >= 0 && f >= 0; r--, f--) {
              result |= (1ULL << (f + r*8));
              if(block & (1ULL << (f + r * 8))) break;
            }
            return result;
          }
    }
}