use crate::bitboard::BitBoard;
use rand::SeedableRng;
use rand_chacha::ChaChaRng;

const ROOK_TABLE_SIZE: usize = 102400;
const BISHOP_TABLE_SIZE: usize = 5248;

pub type Location = (u8, u8);
pub type Square = usize;
type TBBFiles = [BitBoard; 8];
type TBBRanks = [BitBoard; 8];
type TBBSquares = [BitBoard; 64];

pub fn bb_ray(bb_in: BitBoard, square: Square, direction: Direction) -> Bitboard {
    let mut file = square_on_file_rank(square).0 as usize;
    let mut rank = square_on_file_rank(square).1 as usize;
    let mut bb_square = BB_SQUARES[square];
    let mut bb_ray = 0;
    let mut done = false;
    while !done {
        done = true;
        match direction {
            Direction::Up => {
                if rank != Ranks::R8 {
                    bb_square <<= 8;
                    bb_ray |= bb_square;
                    rank += 1;
                    done = (bb_square & bb_in) > 0;
                }
            }
            Direction::Right => {
                if file != Files::H {
                    bb_square <<= 1;
                    bb_ray |= bb_square;
                    file += 1;
                    done = (bb_square & bb_in) > 0;
                }
            }
            Direction::Down => {
                if rank != Ranks::R1 {
                    bb_square >>= 8;
                    bb_ray |= bb_square;
                    rank -= 1;
                    done = (bb_square & bb_in) > 0;
                }
            }
            Direction::Left => {
                if file != Files::A {
                    bb_square >>= 1;
                    bb_ray |= bb_square;
                    file -= 1;
                    done = (bb_square & bb_in) > 0;
                }
            }
            Direction::UpLeft => {
                if (rank != Ranks::R8) && (file != Files::A) {
                    bb_square <<= 7;
                    bb_ray |= bb_square;
                    rank += 1;
                    file -= 1;
                    done = (bb_square & bb_in) > 0;
                }
            }
            Direction::UpRight => {
                if (rank != Ranks::R8) && (file != Files::H) {
                    bb_square <<= 9;
                    bb_ray |= bb_square;
                    rank += 1;
                    file += 1;
                    done = (bb_square & bb_in) > 0;
                }
            }
            Direction::DownRight => {
                if (rank != Ranks::R1) && (file != Files::H) {
                    bb_square >>= 7;
                    bb_ray |= bb_square;
                    rank -= 1;
                    file += 1;
                    done = (bb_square & bb_in) > 0;
                }
            }
            Direction::DownLeft => {
                if (rank != Ranks::R1) && (file != Files::A) {
                    bb_square >>= 9;
                    bb_ray |= bb_square;
                    rank -= 1;
                    file -= 1;
                    done = (bb_square & bb_in) > 0;
                }
            }
        };
    }
    bb_ray
}

const fn init_bb_files() -> TBBFiles {
    const BB_FILE_A: u64 = 0x0101_0101_0101_0101;
    let mut bb_files: TBBFiles = [BitBoard(0); 8];
    let mut i = 0;

    while i < (8) {
        bb_files[i] = BitBoard(BB_FILE_A << (i as u64));
        i += 1;
    }

    bb_files
}

const fn init_bb_ranks() -> TBBRanks {
    let bb_rank_1: u64 = 0xFF;
    let mut bb_ranks = [BitBoard(0); 8];
    let mut i = 0;

    while i < 8 {
        bb_ranks[i] = BitBoard(bb_rank_1 << ((i * 8) as u64));
        i += 1;
    }

    bb_ranks
}

const fn init_bb_squares() -> TBBSquares {
    let mut bb_squares: TBBSquares = [BitBoard(0); 64];
    let mut i = 0;

    while i < 64 {
        bb_squares[i] = BitBoard(1u64 << i as u64);
        i += 1;
    }

    bb_squares
}


const BB_FILES: TBBFiles = init_bb_files();
const BB_RANKS: TBBRanks = init_bb_ranks();
const BB_SQUARES: TBBSquares = init_bb_squares();


pub fn square_on_file_rank(square: Square) -> Location {
    let file = (square % 8) as u8; // square mod 8
    let rank = (square / 8) as u8; // square div 8
    (file, rank)
}

pub fn rook_mask(square: Square) -> BitBoard {
    let location = square_on_file_rank(square);
    let bb_rook_square = BB_SQUARES[square];
    let bb_edges = edges_without_piece(location);
    let bb_mask = BB_FILES[location.0 as usize] | BB_RANKS[location.1 as usize];

    bb_mask & !bb_edges & !bb_rook_square
}

pub fn bishop_mask(square: Square) -> BitBoard {
    let location = square_on_file_rank(square);
    let bb_edges = edges_without_piece(location);
    let bb_up_left = MoveGenerator::bb_ray(0, square, Direction::UpLeft);
    let bb_up_right = MoveGenerator::bb_ray(0, square, Direction::UpRight);
    let bb_down_right = MoveGenerator::bb_ray(0, square, Direction::DownRight);
    let bb_down_left = MoveGenerator::bb_ray(0, square, Direction::DownLeft);

    (bb_up_left | bb_up_right | bb_down_right | bb_down_left) & !bb_edges
}
pub struct Files;
impl Files {
    pub const A: usize = 0;
    pub const B: usize = 1;
    pub const G: usize = 6;
    pub const H: usize = 7;
}

pub struct Ranks;
impl Ranks {
    pub const R1: usize = 0;
    pub const R2: usize = 1;
    pub const R4: usize = 3;
    pub const R5: usize = 4;
    pub const R7: usize = 6;
    pub const R8: usize = 7;
}

fn edges_without_piece(location: Location) -> BitBoard {
    let bb_piece_file = BB_FILES[location.0 as usize];
    let bb_piece_rank = BB_RANKS[location.1 as usize];

    (BB_FILES[Files::A] & !bb_piece_file)
        | (BB_FILES[Files::H] & !bb_piece_file)
        | (BB_RANKS[Ranks::R1] & !bb_piece_rank)
        | (BB_RANKS[Ranks::R8] & !bb_piece_rank)
}

pub type Piece = usize;
pub struct Pieces;
impl Pieces {
    pub const KING: Piece = 0;
    pub const QUEEN: Piece = 1;
    pub const ROOK: Piece = 2;
    pub const BISHOP: Piece = 3;
    pub const KNIGHT: Piece = 4;
    pub const PAWN: Piece = 5;
    pub const NONE: Piece = 6;
}

pub const PIECE_NAME: [&str; 6 + 1] =
    ["King", "Queen", "Rook", "Bishop", "Knight", "Pawn", "-"];

pub fn find_magics(piece: Piece) {
    // First check if we're actually dealing with a rook or a bishop.
    let ok = piece == Pieces::ROOK || piece == Pieces::BISHOP;
    assert!(ok, "Illegal piece: {piece}");

    // Create working variables.
    let is_rook = piece == Pieces::ROOK;
    let mut rook_table: Vec<BitBoard> = vec![BitBoard(0); ROOK_TABLE_SIZE];
    let mut bishop_table: Vec<BitBoard> = vec![BitBoard(0); BISHOP_TABLE_SIZE];
    let mut random = ChaChaRng::from_entropy();
    let mut offset = 0;

    println!("Finding magics for: {}", PIECE_NAME[piece]);
    for sq in 0..64 {
        // Create the mask for either the rook or bishop.
        let r_mask = rook_mask(sq);
        let b_mask = bishop_mask(sq);
        let mask = if is_rook { r_mask } else { b_mask };

        // Precalculate needed values.
        let bits = mask.count_ones(); // Number of set bits in the mask
        let permutations = 2u64.pow(bits); // Number of blocker boards to be indexed.
        let end = offset + permutations - 1; // End index in the attack table.

        // Create blocker boards for the current mask.
        let blocker_boards = MoveGenerator::blocker_boards(mask);

        // Create attack boards for the current square/blocker combo (either
        // rook or bishop).
        let r_ab = MoveGenerator::rook_attack_boards(sq, &blocker_boards);
        let b_ab = MoveGenerator::bishop_attack_boards(sq, &blocker_boards);
        let attack_boards = if is_rook { r_ab } else { b_ab };

        // Done calculating needed data. Create a new magic.
        let mut try_this: Magic = Default::default(); // New magic
        let mut found = false; // While loop breaker if magic works;
        let mut attempts = 0; // Track needed attempts to find the magic.

        // Set up the new magic with the values we already know.
        try_this.mask = mask;
        try_this.shift = (64 - bits) as u8;
        try_this.offset = offset;

        // Start finding a magic that works for this square, for all permuations.
        while !found {
            attempts += 1; // Next attempt to find magic.
            found = true; // Assume this new magic will work.

            // Create a random magic number to test.
            try_this.nr = random.gen::<u64>() & random.gen::<u64>() & random.gen::<u64>();

            // Now try all possible permutations of blocker boards on this square.
            for i in 0..permutations {
                // Get the index where the magic for this blocker board
                // needs to go (if it works.)
                let next = i as usize;
                let index = try_this.get_index(blocker_boards[next]);

                // Use either a reference to the rook or bishop table.
                let r_table = &mut rook_table[..];
                let b_table = &mut bishop_table[..];
                let table: &mut [BitBoard] = if is_rook { r_table } else { b_table };

                // If the table at this index is empty...
                if table[index] == 0 {
                    // Check if we're within the expected range
                    let fail_low = index < offset as usize;
                    let fail_high = index > end as usize;
                    assert!(!fail_low && !fail_high, "Indexing error.");

                    // We found a working magic.
                    table[index] = attack_boards[next];
                } else {
                    // The table at this index is not empty. We have a
                    // collision. This magic doesn't work. Wipe the part of
                    // the table we are working with. Try a new number.
                    for wipe_index in offset..=end {
                        table[wipe_index as usize] = 0;
                    }
                    found = false;
                    break;
                }
            }
        }

        // We got out of the loop and found a random magic number that can
        // index all the attack boards for a rook/bishop for a single
        // square without a collision. Report this number.
        found_magic(sq, try_this, offset, end, attempts);

        // Set table offset for next magic.
        offset += permutations;
    }

    // Check if the entire table is correct. The offset should be equal to
    // the size of the table. If it isn't, we skipped permuations and thus
    // have some sort of error in our code above.
    let r_ts = ROOK_TABLE_SIZE as u64;
    let b_ts = BISHOP_TABLE_SIZE as u64;
    let expected = if is_rook { r_ts } else { b_ts };
    const ERROR: &str = "Creating magics failed. Permutations were skipped.";

    assert!(offset == expected, "{}", ERROR);
}

// Print the magic number.
fn found_magic(square: Square, m: Magic, offset: u64, end: u64, attempts: u64) {
    println!(
        "{}: {:24}u64 (offset: {:6}, end: {:6}, attempts: {})",
        SQUARE_NAME[square], m.nr, offset, end, attempts
    );
}