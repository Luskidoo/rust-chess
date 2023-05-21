pub const TRUE: usize = 1;
pub const FALSE: usize	= 0;

pub const GEN_STACK: usize	= 1120;
pub const MAX_PLY: usize = 32;
pub const HIST_STACK: usize = 400;

pub const LIGHT: i32 = 0;
pub const DARK: i32 = 1;

pub const MAX_POSITION_MOVES: usize = 256;

pub const PAWN: i32 = 0;
pub const KNIGHT: i32 = 1;
pub const BISHOP: i32 = 2;
pub const ROOK: i32 = 3;
pub const QUEEN: i32	= 4;
pub const KING: i32 = 5;

pub const EMPTY: i32 = 6;

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

pub fn ROW(x: usize) -> usize {
    x >> 3
}
pub fn COL(x: usize) -> usize {
    x & 7
}


/* the piece letters, for print_board() */
pub const piece_char: [char; 6] = [
	'P', 'N', 'B', 'R', 'Q', 'K'
];
