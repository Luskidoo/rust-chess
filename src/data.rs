pub const mailbox: [i32; 120] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1,  0,  1,  2,  3,  4,  5,  6,  7, -1,
    -1,  8,  9, 10, 11, 12, 13, 14, 15, -1,
    -1, 16, 17, 18, 19, 20, 21, 22, 23, -1,
    -1, 24, 25, 26, 27, 28, 29, 30, 31, -1,
    -1, 32, 33, 34, 35, 36, 37, 38, 39, -1,
    -1, 40, 41, 42, 43, 44, 45, 46, 47, -1,
    -1, 48, 49, 50, 51, 52, 53, 54, 55, -1,
    -1, 56, 57, 58, 59, 60, 61, 62, 63, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1
];

pub const mailbox64: [i32; 64] = [
   21, 22, 23, 24, 25, 26, 27, 28,
   31, 32, 33, 34, 35, 36, 37, 38,
   41, 42, 43, 44, 45, 46, 47, 48,
   51, 52, 53, 54, 55, 56, 57, 58,
   61, 62, 63, 64, 65, 66, 67, 68,
   71, 72, 73, 74, 75, 76, 77, 78,
   81, 82, 83, 84, 85, 86, 87, 88,
   91, 92, 93, 94, 95, 96, 97, 98
];

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

pub const MAX_POSITION_MOVES: usize = 256;

pub const RANK_1: i32 = 0;
pub const RANK_2: i32 = 1;
pub const RANK_3: i32 = 2;
pub const RANK_4: i32 = 3;
pub const RANK_5: i32 = 4;
pub const RANK_6: i32 = 5;
pub const RANK_7: i32 = 6;
pub const RANK_8: i32 = 7;

pub const FILE_1: i32 = 0;
pub const FILE_2: i32 = 1;
pub const FILE_3: i32 = 2;
pub const FILE_4: i32 = 3;
pub const FILE_5: i32 = 4;
pub const FILE_6: i32 = 5;
pub const FILE_7: i32 = 6;
pub const FILE_8: i32 = 7;

pub const init_pieces: [i32; 64] = [
    4, 2, 3, 5, 6, 3, 2, 4,
    1, 1, 1, 1, 1, 1, 1, 1,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    7, 7, 7, 7, 7, 7, 7, 7,
    10, 8, 9, 11, 12, 9, 8, 10,
];

pub const piece_char: [char; 13] = [
    '.', 'P', 'N', 'B', 'R', 'Q', 'K', 'p', 'n', 'b', 'r', 'q', 'k'
];

pub fn file(sq: i32) -> i32 {
    sq & 7
}

pub fn rank(sq: i32) -> i32 {
    sq >> 3
}