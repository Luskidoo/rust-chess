pub const TRUE: usize = 1;
pub const FALSE: usize	= 0;

pub const BRD_SQ_NUM: usize = 120;

pub const GEN_STACK: usize	= 1120;
pub const MAX_PLY: usize = 32;
pub const HIST_STACK: usize = 400;

pub const WHITE: i32 = 0;
pub const BLACK: i32 = 1;
pub const BOTH: i32 = 2;

pub const MAX_POSITION_MOVES: usize = 256;

pub const EMPTY: i32 = 0;
pub const wP: i32 = 1;
pub const wN: i32 = 2;
pub const wB: i32 = 3;
pub const wR: i32 = 4;
pub const wQ: i32 = 5;
pub const wK: i32 = 6;
pub const bP: i32 = 7;
pub const bN: i32 = 8;
pub const bB: i32 = 9;
pub const bR: i32 = 10;
pub const bQ: i32 = 11;
pub const bK: i32 = 12;

pub const piececol: [i32; 13] = [
	BOTH, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK
];

pub const MFLAGEP: i32 = 0x40000;
pub const MFLAGPS: i32 = 0x80000;
pub const MFLAGCA: i32 = 0x1000000;

pub const MFLAGCAP: i32 = 0x7C000;
pub const MFLAGPROM: i32 = 0xF00000;

/* useful squares */
pub const A1: i32 = 56;
pub const B1: i32 = 57;
pub const C1: i32 = 58;
pub const D1: i32 = 59;
pub const E1: i32 = 60;
pub const F1: i32 = 61;
pub const G1: i32 = 62;
pub const H1: i32 = 63;
pub const A8: i32 = 0;
pub const B8: i32 = 1;
pub const C8: i32 = 2;
pub const D8: i32 = 3;
pub const E8: i32 = 4;
pub const F8: i32 = 5;
pub const G8: i32 = 6;
pub const H8: i32 = 7;

pub const FILE_A: i32 = 0;
pub const FILE_B: i32 = 1;
pub const FILE_C: i32 = 2;
pub const FILE_D: i32 = 3;
pub const FILE_E: i32 = 4;
pub const FILE_F: i32 = 5;
pub const FILE_G: i32 = 6;
pub const FILE_H: i32 = 7;
pub const FILE_NONE: i32 = 8;

pub const RANK_1: i32 = 0;
pub const RANK_2: i32 = 1;
pub const RANK_3: i32 = 2;
pub const RANK_4: i32 = 3;
pub const RANK_5: i32 = 4;
pub const RANK_6: i32 = 5;
pub const RANK_7: i32 = 6;
pub const RANK_8: i32 = 7;
pub const RANK_NONE: i32 = 8;

pub const OFFBOARD: i32 = 100;

pub fn ROW(x: usize) -> usize {
    x >> 3
}
pub fn COL(x: usize) -> usize {
    x & 7
}

pub fn fr2sq(f: i32, r: i32) -> i32 {
    (21 + f) + r*10
}

pub fn from_sq(m: i32) -> i32 {
    m & 0x7F
}

pub fn to_sq(m: i32) -> i32 {
    (m >> 7) & 0x7F
}


/* the piece letters, for print_board() */
pub const piece_char: [char; 13] = [
	'.', 'P', 'N', 'B', 'R', 'Q', 'K', 'p', 'n', 'b', 'r', 'q', 'k'
];
