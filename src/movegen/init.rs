use super::MoveGenerator;
use crate::{bitboard::BitBoard, movegen::magics::Magic};

pub const BISHOP_MAGIC_NRS: [u64; 64] = [
    2310454429704290569u64, 37163502750244928u64, 145330200115150856u64, 573953659699200u64,
    9845999220824211456u64, 574016004032512u64, 10093699283674480640u64, 2306407060834902016u64,
    2883575003184432136u64, 1747410678824308864u64, 9259405249167245312u64, 936784527773139074u64,
    4629702641998381057u64, 201028145628315697u64, 4899992295377881088u64, 4630405483133404688u64,
    153474299838154784u64, 2286992943744036u64, 434597432802681416u64, 865817269052115456u64,
    9156750026475656u64, 599823317909770240u64, 4578375142474880u64, 2308525819264500224u64,
    18596057879421451u64, 18331093560345096u64, 2305880392877736000u64, 56602859688444160u64,
    5382084129205534724u64, 5767422822691897608u64, 283691220206592u64, 144398865845093376u64,
    1163523824685120u64, 20267333288223264u64, 325489801822240u64, 4755836425302245636u64,
    594475563668865152u64, 1162496335329427604u64, 9244765235704371236u64, 576667461564269056u64,
    146371454722771202u64, 426679365288452u64, 13724105480340736u64, 1152922330050364928u64,
    4620737202526097424u64, 1316476062695166464u64, 13981996823661781640u64, 12430506881068303489u64,
    5193780677221351424u64, 426612797737280u64, 37445932288049152u64, 1171147012042137601u64,
    504403227018657856u64, 4629845569785954560u64, 4686013077882208273u64, 1154056209263894528u64,
    613054853085794304u64, 9025075185721408u64, 9571249324951568u64, 10999715432448u64,
    290408795603472u64, 10664524198170591488u64, 5924513492108288u64, 90511840181764112u64,
];

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
