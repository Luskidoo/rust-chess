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
        let r_magic_nr_array = Self::generate_magics(true);
        let b_magic_nr_array = Self::generate_magics(false);
        for sq in 0..64 {
            let r_mask = MoveGenerator::rook_mask(sq);
            let b_mask = MoveGenerator::bishop_mask(sq);
            let mask = if is_rook { r_mask } else { b_mask };

            let bits = mask.pop_count(); // Number of set bits in the mask
            let permutations = 2u64.pow(bits); // Number of blocker boards to be indexed.
            let end = offset + permutations - 1; // End point in the attack table.
            let blocker_boards = MoveGenerator::blocker_boards(mask);
            let mut r_ab: Vec<BitBoard> = vec![];
            let mut b_ab: Vec<BitBoard> = vec![];
            for blocker in blocker_boards.iter() {
                r_ab.push(MoveGenerator::rook_attacks(sq, *blocker));
                b_ab.push(MoveGenerator::bishop_attacks(sq, *blocker));
            };
            
            let attack_boards = if is_rook { r_ab } else { b_ab };

            let mut magic: Magic = Magic::new();
            let r_magic_nr = r_magic_nr_array[sq as usize];
            let b_magic_nr = b_magic_nr_array[sq as usize];

            magic.mask = mask;
            magic.shift = (64u64 - bits as u64) as u8;
            magic.offset = offset;
            magic.nr = if is_rook { r_magic_nr.0 } else { b_magic_nr.0 };

            for i in 0..permutations {
                //let next = i as usize;
                let index = magic.get_index(blocker_boards[i as usize]);
                let rook_table = &mut self.rook[..];
                let bishop_table = &mut self.bishop[..];
                let table = if is_rook { rook_table } else { bishop_table };

                if table[index] == BitBoard(0) {
                    let fail_low = index < offset as usize;
                    let fail_high = index > end as usize;
                    assert!(!fail_low && !fail_high, "Indexing error. Error in Magics.");
                    table[index] = attack_boards[i as usize];
                } else {
                    panic!("Attack table index not empty. Error in Magics.");
                }
            }

            // No failures  during indexing. Store this magic.
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
