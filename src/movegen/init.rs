use super::MoveGenerator;
use crate::{bitboard::BitBoard, movegen::magics::Magic};


impl MoveGenerator {
    pub fn init_white_pawn_attacks() -> [BitBoard; 64] {
        let mut moves = [BitBoard(0); 64];
        for sq in 0..64 {
            moves[sq] = Self::white_pawn_attacks((1u64 << sq) as u64);
        }
        moves
    }

    pub fn init_black_pawn_attacks() -> [BitBoard; 64] {
        let mut moves = [BitBoard(0); 64];
        for sq in 0..64 {
            moves[sq] = Self::black_pawn_attacks((1u64 << sq) as u64);
        }
        moves
    }

    pub fn init_knight_moves() -> [BitBoard; 64] {
        let mut moves = [BitBoard(0); 64];
        for sq in 0..64 {
            moves[sq] = Self::knight_moves((1u64 << sq) as u64);
        }
        moves
    }

    pub fn init_magics(&mut self, is_rook: bool) {
        let mut offset = 0;
        let magic_nr_array = Self::generate_magics(is_rook);
        for sq in 0..64 {
            let mask = if is_rook {
                MoveGenerator::rook_mask(sq)
            } else {
                MoveGenerator::bishop_mask(sq)
            };

            let bits = mask.0.count_ones(); // Number of set bits in the mask
            let permutations = 2u64.pow(bits); // Number of blocker boards to be indexed.
            let end = offset + permutations - 1; // End point in the attack table.
            let blocker_boards = MoveGenerator::blocker_boards(mask);
            let mut r_ab: Vec<BitBoard> = vec![];
            let mut b_ab: Vec<BitBoard> = vec![];
            let blocker_boards = MoveGenerator::blocker_boards(mask);
            
            let attack_boards: Vec<BitBoard> = blocker_boards.iter()
            .map(|blocker| if is_rook {
                MoveGenerator::rook_attacks(sq, *blocker)
            } else {
                MoveGenerator::bishop_attacks(sq, *blocker)
            })
            .collect();

            let mut magic: Magic = Magic::new();

            magic.mask = mask;
            magic.shift = (64 - bits) as u8;
            magic.offset = offset;
            magic.nr = magic_nr_array[sq as usize].0;

            let rook_table = &mut self.rook[..];
            let bishop_table = &mut self.bishop[..];
            let table = if is_rook { rook_table } else { bishop_table };
            for (i, &blocker) in blocker_boards.iter().enumerate() {
                let index = magic.get_index(blocker);
                
                if index < offset as usize || index > end as usize {
                    panic!("Indexing error. Error in Magics. Square: {}, Index: {}, Offset: {}, End: {}", sq, index, offset, end);
                }
                
                if table[index] == BitBoard(0) {
                    table[index] = attack_boards[i];
                } else {
                    panic!("Attack table index not empty. Error in Magics. Square: {}, Index: {}", sq, index);
                }
            }

            // No failures during indexing. Store this magic.
            if is_rook {
                self.rook_magics[sq as usize] = magic;
            } else {
                self.bishop_magics[sq as usize] = magic;
            }

            // Do the next magic.
            offset += permutations;
        }

        // All permutations (blocker boards) should have been indexed.
        let r_ts = 102400 as u64;
        let b_ts = 5_248 as u64;
        let expectation = if is_rook { r_ts } else { b_ts };
        const ERROR: &str = "Initializing magics failed. Check magic numbers.";

        assert!(offset == expectation, "{}", ERROR);
    }

}
