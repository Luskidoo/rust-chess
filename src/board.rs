pub use crate::data::*;
pub use crate::validate::*;
use rand::*;
use ndarray::*;
use once_cell::sync::Lazy;

#[derive(Copy, Clone, Debug)]
pub struct MoveBytes{
	from: char,
	to: char,
	promote: char,
	bits: char,
}

#[derive(Copy, Clone, Debug)]
pub struct Move {
	pub m: i32,
	pub score: i32,
}

#[derive(Copy, Clone, Debug)]
pub struct MoveList {
	pub moves: [Move; MAX_POSITION_MOVES],
	pub count: usize,
}

#[derive(Copy, Clone)]
pub struct Undo {
    pub m: i32,
    pub castlePerm: i32,
    pub enPas: i32,
}

impl Undo {
    pub const fn default() -> Undo {
        Undo {
            m: 0,
            castlePerm: 0,
            enPas: NO_SQ,
        }
    }
}

pub struct Board {
    pub pieces: [i32; BRD_SQ_NUM],
    pub pawns: [u64; 3],
    pub side: i32,
    pub enPas: i32,
    pub ply: i32,
    pub hisPly: i32,
    pub castlePerm: i32,
    pub pceNum: [i32; 13],
    pub bigPce: [i32; 2],
    pub majPce: [i32; 2],
    pub minPce: [i32; 2],
    pub history: [Undo; MAX_GAME_MOVES],
    pub pList: [[usize; 13]; 10],

}

impl Board {
    pub const fn default() -> Board { 
        Board {
            pieces: [OFFBOARD; BRD_SQ_NUM],
            pawns: [0; 3],
            side: BOTH,
            enPas: NO_SQ,
            ply: 0,
            hisPly: 0,
            castlePerm: 0,
            pceNum: [0; 13],
            bigPce: [0; 2],
            majPce: [0; 2],
            minPce: [0; 2],
            history: [Undo::default(); MAX_GAME_MOVES],
            pList: [[0; 13]; 10],
        }
    }

    pub fn from_fen() {
        todo!()
    }
}

/* an element of the history stack, with the information
   necessary to take a move back. */
pub struct HistT{
	m: Move,
	capture: i32,
	castle: i32,
	ep: i32,
	fifty: i32,
	hash: i32,
}

const empty_move: Move = Move {
    m: 0,
	score: 0,
};

static init_color: [i32; 64] = [
	1, 1, 1, 1, 1, 1, 1, 1,
	1, 1, 1, 1, 1, 1, 1, 1,
	3, 3, 3, 3, 3, 3, 3, 3,
	3, 3, 3, 3, 3, 3, 3, 3,
	3, 3, 3, 3, 3, 3, 3, 3,
	3, 3, 3, 3, 3, 3, 3, 3,
    1, 1, 1, 1, 1, 1, 1, 1,
	1, 1, 1, 1, 1, 1, 1, 1,
];

static init_piece: [i32; 64] = [	
    10, 8, 9, 12, 11, 9, 8, 10,
    7, 7, 7, 7, 7, 7, 7, 7,
	0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0,
	1, 1, 1, 1, 1, 1, 1, 1,
	4, 2, 3, 6, 5, 3, 2, 4,
];

static mailbox: [i32; 120] = [
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

static mailbox64: [usize; 64] = [
   21, 22, 23, 24, 25, 26, 27, 28,
   31, 32, 33, 34, 35, 36, 37, 38,
   41, 42, 43, 44, 45, 46, 47, 48,
   51, 52, 53, 54, 55, 56, 57, 58,
   61, 62, 63, 64, 65, 66, 67, 68,
   71, 72, 73, 74, 75, 76, 77, 78,
   81, 82, 83, 84, 85, 86, 87, 88,
   91, 92, 93, 94, 95, 96, 97, 98
];

static mut first_move: [i32; MAX_PLY] = [0; MAX_PLY];

/* slide, offsets, and offset are basically the vectors that
  pieces can move in. If slide for the piece is FALSE, it can
  only move one square in any one direction. offsets is the
  number of directions it can move in, and offset is an array
  of the actual directions. */

static slide: [usize; 6] = [
   FALSE, FALSE, TRUE, TRUE, TRUE, FALSE
];

static offsets: [i32; 13] = [
   0, 8, 4, 4, 8, 8, 0, 0, 8, 4, 4, 8, 8
];

static offset: [[i32; 8]; 6] = [
   [ 0, 0, 0, 0, 0, 0, 0, 0 ],
   [ -21, -19, -12, -8, 8, 12, 19, 21 ],
   [ -11, -9, 9, 11, 0, 0, 0, 0 ],
   [ -10, -1, 1, 10, 0, 0, 0, 0 ],
   [ -11, -10, -9, -1, 1, 9, 10, 11 ],
   [ -11, -10, -9, -1, 1, 9, 10, 11 ]
];

const loop_slide_pce: [i32; 8] = [
    wB, wR, wQ, 0, bB, bR, bQ, 0
];
   
const loop_non_slide_pce: [i32; 6] = [
    wN, wK, 0, bN, bK, 0
];

const loop_slide_index: [i32; 2] = [ 0, 4 ];
const loop_non_slide_index: [i32; 2] = [ 0, 3 ];

static pce_dir: [[i32; 8]; 13] = [
	[ 0, 0, 0, 0, 0, 0, 0, 0 ],
	[ 0, 0, 0, 0, 0, 0, 0, 0 ],
	[ -11, -10, -9, -1, 1, 9, 10, 11 ],
	[ -11, -9, 9, 11, 0, 0, 0, 0 ],
	[ -10, -1, 1, 10, 0, 0, 0, 0 ],
	[ -11, -10, -9, -1, 1, 9, 10, 11 ],
	[ -11, -10, -9, -1, 1, 9, 10, 11 ],
	[ 0, 0, 0, 0, 0, 0, 0, 0 ],
	[ -21, -19, -12, -8, 8, 12, 19, 21 ],
	[ -11, -9, 9, 11, 0, 0, 0, 0 ],
	[ -10, -1, 1, 10, 0, 0, 0, 0 ],
	[ -11, -10, -9, -1, 1, 9, 10, 11 ],
	[ -11, -10, -9, -1, 1, 9, 10, 11 ]
];

static num_dir: [i32; 13] = [
    0, 0, 8, 4, 4, 8, 8, 0, 8, 4, 4, 8, 8
];

static mut history: Lazy<Array2<i32>> = Lazy::new(|| Array2::<i32>::zeros((64, 64)));

/* random numbers used to compute hash; see set_hash() in board.c */
static mut hash_piece: Lazy<Array3<i32>> = Lazy::new(|| Array3::<i32>::zeros((2, 13, 64)));  /* indexed by piece [color][type][square] */
static mut hash_side: i32 = 0;
static mut hash_ep: [i32; 64] = [0; 64];

static mut count_moves: i32 = 0;

// pub fn init_board() {
//     unsafe {
//         for i in 0..64 {
//             //color[i] = init_color[i];
//             //piece[i] = init_piece[i];
//         }
//         side = WHITE;
//         //xside = BLACK;
//         castle = 15;
//         ep = -1;
//         fifty = 0;
//         ply = 0;
//         hply = 0;
//         //set_hash();  /* init_hash() must be called before this function */
//         first_move[0] = 0;
//     }
	
// }

fn hash_rand() -> i32 {
	let mut r: i32 = 0;
    let mut r2: i32;

	for i in 0..32 {
        r2 = random();
        r ^= r2 << i;
    }   
	r
}

// unsafe fn set_hash() {
//     let mut hp: i32 = 0;
// 	let mut local_hash: i32 = 0;	
// 	for i in 0..=64{
//         if color[i] != EMPTY {
//             hp = hash_piece[[color[i] as usize, piece[i] as usize, i]];
//             local_hash ^= hp;
//         }		
//         if side == BLACK {
//             local_hash ^= hash_side;
//         }

//         if ep != -1 {
//             local_hash ^= hash_ep[ep as usize];
//         }	
//     }
// }

pub unsafe fn init_hash() {
	for i in 0..2 {
        for j in 0..6 {
            for k in 0..64 {
                hash_piece[[i as usize, j as usize, k as usize]] = hash_rand();
            }
        }
    }
		
	hash_side = hash_rand();

	for i in 0..64 {
        hash_ep[i] = hash_rand();
    }
		
}

fn move_bytes(from: i32, to: i32, capture: i32, promote: i32, fl: i32) -> i32 {
    (from) | ((to) << 7) | ( (capture) << 14 ) | ( (promote) << 20 ) | (fl)
}

fn from_square(m: i32) -> usize {
    (m & 0x7F).try_into().unwrap()
}

fn to_square(m: i32) -> usize {
    (m >> 7 & 0x7F).try_into().unwrap()
}

unsafe fn add_quiet_move(m: i32, list: &mut MoveList) {

    //assert!(sq_on_board(from_sq(m)));
    //assert!(sq_on_board(to_sq(m)));

    list.moves[list.count].m = m;
    list.moves[list.count].score = history[[from_square(m), to_square(m)]];
    list.count += 1;
}

unsafe fn add_capture_move(m: i32, mut list: &mut MoveList) {

    //assert!(sq_on_board(from_sq(m)));
    //assert!(sq_on_board(to_sq(m)));

    list.moves[list.count].m = m;
    // TODO Add MvvLVA
    list.count += 1;
}

unsafe fn add_ep_move(m: i32, mut list: &mut MoveList) {

    assert!(sq_on_board(from_sq(m)));
    assert!(sq_on_board(to_sq(m)));

    list.moves[list.count].m = m;
    list.moves[list.count].score = 105 + 100000;
    list.count += 1;
}

unsafe fn add_white_pawn_move(from: i32, to: i32, list: &mut MoveList) {

    //assert!(sq_on_board(from));
    //assert!(sq_on_board(to));

    if rank_index(from as usize) == 6 {
        for i in wN..=wQ {
            add_quiet_move(move_bytes(from, to, EMPTY, i, 0), list);
        }
    } 
    else {
        add_quiet_move(move_bytes(from, to, EMPTY, EMPTY, 0), list);
    }    
}

unsafe fn add_white_pawn_cap_move(from: i32, to: i32, cap: i32, list: &mut MoveList) {

    //assert!(sq_on_board(from));
    //assert!(sq_on_board(to));

    if rank_index(from as usize) == 6 {
        for i in wN..=wQ {
            add_capture_move(move_bytes(from, to, EMPTY, i, 0), list);
        }
    } 
    else {
        add_capture_move(move_bytes(from, to, EMPTY, EMPTY, 0), list);
    }   
}

unsafe fn add_black_pawn_move(from: i32, to: i32, list: &mut MoveList) {

    //assert!(sq_on_board(from));
    //assert!(sq_on_board(to));

    if ranksbrd[from as usize] == RANK_2 {
        for i in bN..=bQ {
            add_quiet_move(move_bytes(from, to, EMPTY, i, 0), list);
        }
    } 
    else {
        add_quiet_move(move_bytes(from, to, EMPTY, EMPTY, 0), list);
    }    
}

unsafe fn add_black_pawn_cap_move(from: i32, to: i32, cap: i32, list: &mut MoveList) {

    //assert!(sq_on_board(from));
    //assert!(sq_on_board(to));

    if ranksbrd[from as usize] == RANK_2 {
        for i in bN..=bQ {
            add_capture_move(move_bytes(from, to, EMPTY, i, 0), list);
        }
    } 
    else {
        add_capture_move(move_bytes(from, to, EMPTY, EMPTY, 0), list);
    }   
}

pub static mut sq120tosq64: [i32; BRD_SQ_NUM] = [65; BRD_SQ_NUM];
pub static mut sq64tosq120: [i32; 65] = [120; 65];

pub static mut filesbrd: [i32; BRD_SQ_NUM] = [OFFBOARD; BRD_SQ_NUM];
pub static mut ranksbrd: [i32; BRD_SQ_NUM] = [OFFBOARD; BRD_SQ_NUM];

pub unsafe fn init_sq120_to_sq64() {
    let mut sq: i32 = A1;
    let mut sq64: i32 = 0;

    for rank in RANK_1..=RANK_8 {
        for file in FILE_A..=FILE_H {
            sq = fr2sq(file, rank);
            sq64tosq120[sq64 as usize] = sq;
			sq120tosq64[sq as usize] = sq64;
			sq64 += 1;
        }
    }
}

pub unsafe fn init_files_ranks_board() {
    let mut sq: i32 = A1;

    for rank in RANK_1..=RANK_8 {
        for file in FILE_A..=FILE_H {
            sq = fr2sq(file, rank);
            filesbrd[sq as usize] = file;
			ranksbrd[sq as usize] = rank;
        }
    }

    for x in filesbrd {
        print!("{} ", x);
    }
}


pub unsafe fn generate_all_moves(pos: &mut Board, list: &mut MoveList)
{
    list.count = 0;

    let mut pce = EMPTY;
    let side = pos.side;
    let mut sq: usize = 0;
    let mut t_sq: usize = 0;
    let mut pce_num = 0;
    let mut dir: usize = 0;
    let mut pce_index = 0;


    let mut n: i32 = 0;
    if side == WHITE {
        for pce_num in 0..pos.pceNum[wP as usize] {
            sq = pos.pList[pce_num as usize][wP as usize];
            if pos.pieces[sq + 10] == EMPTY {
                add_white_pawn_move(sq.try_into().unwrap(), (sq + 10).try_into().unwrap(), list);
                if rank_index(sq) == 1 && (pos.pieces[(sq + 20) as usize] == EMPTY) {
                    add_quiet_move(move_bytes(sq.try_into().unwrap(), (sq + 20).try_into().unwrap(), EMPTY, EMPTY, MFLAGPS), list);
                }   
            }
            if piece_col[pos.pieces[sq + 9] as usize] == BLACK {
                add_white_pawn_cap_move(sq.try_into().unwrap(), (sq + 9).try_into().unwrap(), pos.pieces[(sq + 9) as usize], list);
            } 
            if piece_col[pos.pieces[sq + 11] as usize] == BLACK {
                add_white_pawn_cap_move(sq.try_into().unwrap(), (sq + 11).try_into().unwrap(), pos.pieces[(sq + 11) as usize], list);
            } 
                
            // generate en passent moves
            if pos.enPas != NO_SQ {
                if (sq + 9) == pos.enPas.try_into().unwrap() {
                    add_ep_move(move_bytes((sq).try_into().unwrap(), (sq + 9).try_into().unwrap(), EMPTY, EMPTY, MFLAGEP), list);
                }  
                if (sq + 11) == pos.enPas.try_into().unwrap() {
                    add_ep_move(move_bytes((sq).try_into().unwrap(), (sq + 11).try_into().unwrap(), EMPTY, EMPTY, MFLAGEP), list);
                }
            }

            // generate castle moves 
            if pos.castlePerm & 1 == 1 {
                if pos.pieces[F1 as usize] == EMPTY && pos.pieces[G1 as usize] == EMPTY {
                    add_quiet_move(move_bytes(E1, G1, EMPTY, EMPTY, MFLAGCA), list);
                }
            }

            if pos.castlePerm & 2 == 1 {
                if pos.pieces[B1 as usize] == EMPTY && pos.pieces[C1 as usize] == EMPTY && pos.pieces[D1 as usize] == EMPTY{
                    add_quiet_move(move_bytes(E1, C1, EMPTY, EMPTY, MFLAGCA), list);
                }
            }
        }
    }

    else {
        for pce_num in 0..pos.pceNum[bP as usize] {
            sq = pos.pList[pce_num as usize][bP as usize];
            if pos.pieces[sq - 10] == EMPTY {
                add_white_pawn_move(sq.try_into().unwrap(), (sq - 10).try_into().unwrap(), list);
                if rank_index(sq) == 1 && (pos.pieces[(sq - 20) as usize] == EMPTY) {
                    add_quiet_move(move_bytes(sq.try_into().unwrap(), (sq - 20).try_into().unwrap(), EMPTY, EMPTY, MFLAGPS), list);
                }   
            }
            if piece_col[pos.pieces[sq - 9] as usize] == WHITE {
                add_white_pawn_cap_move(sq.try_into().unwrap(), (sq - 9).try_into().unwrap(), pos.pieces[sq - 9], list);
            } 
            if piece_col[pos.pieces[sq - 11] as usize] == WHITE {
                add_white_pawn_cap_move(sq.try_into().unwrap(), (sq - 11).try_into().unwrap(), pos.pieces[sq - 11], list);
            }
            // generate en passent moves
            if pos.enPas != NO_SQ {
                if (sq - 9) == pos.enPas.try_into().unwrap() {
                    add_ep_move(move_bytes((sq).try_into().unwrap(), (sq - 9).try_into().unwrap(), EMPTY, EMPTY, MFLAGEP), list);
                }  
                if (sq - 11) == pos.enPas.try_into().unwrap() {
                    add_ep_move(move_bytes((sq).try_into().unwrap(), (sq - 11).try_into().unwrap(), EMPTY, EMPTY, MFLAGEP), list);
                }
            }
        } 
        
        if pos.castlePerm & 4 == 1 {
            if pos.pieces[F8 as usize] == EMPTY && pos.pieces[G8 as usize] == EMPTY {
                add_quiet_move(move_bytes(E8, G8, EMPTY, EMPTY, MFLAGCA), list);
            }
        }
                    
        if pos.castlePerm & 8 == 1 {
            if pos.pieces[B8 as usize] == EMPTY && pos.pieces[C8 as usize] == EMPTY && pos.pieces[D8 as usize] == EMPTY {
                add_quiet_move(move_bytes(E8, C8, EMPTY, EMPTY, MFLAGCA), list);
            }
        }

    }

    // sliding pieces loop
    let mut pce_index: i32 = loop_slide_index[side as usize];
    pce = loop_slide_pce[pce_index as usize];
    pce_index += 1;

    while pce != 0 {
        for pce_num in 0..pos.pceNum[pce as usize] {
            sq = pos.pList[pce as usize][pce_num as usize];
            for i in 0..num_dir[pce as usize] {
                dir = pce_dir[i as usize][pce as usize] as usize;
                t_sq = sq + dir;
                if pos.pieces[t_sq] != EMPTY {
                    if piece_col[pos.pieces[t_sq as usize] as usize] == (side ^ 1) {
                        add_capture_move(move_bytes(sq.try_into().unwrap(), t_sq.try_into().unwrap(), pos.pieces[t_sq as usize], EMPTY, 0), list);

                    }
                    break
                }
                else {
                    add_quiet_move(move_bytes(sq.try_into().unwrap(), t_sq.try_into().unwrap(), EMPTY, EMPTY, 0), list);
                    t_sq += dir;
                }
            }
        }
        pce = loop_slide_pce[pce_index as usize];
        pce_index += 1;
    }

    // non sliding pieces loop
    pce_index = loop_non_slide_index[side as usize];
    pce = loop_non_slide_index[pce_index as usize];
    pce_index += 1;

    while pce != 0 {
        for pce_num in 0..pos.pceNum[pce as usize] {
            sq = pos.pList[pce as usize][pce_num as usize];
            for i in 0..num_dir[pce as usize] {
                t_sq = sq + dir;

                if pos.pieces[t_sq as usize] != EMPTY {
                    if piece_col[pos.pieces[t_sq as usize] as usize] == (side ^ 1) {
                        add_capture_move(move_bytes(sq.try_into().unwrap(), t_sq.try_into().unwrap(), pos.pieces[t_sq as usize], EMPTY, 0), list);
                    }
                    continue;
                }
                add_quiet_move(move_bytes(sq.try_into().unwrap(), t_sq.try_into().unwrap(), EMPTY, EMPTY, 0), list);
            }
        }
        pce = loop_non_slide_pce[pce_index as usize];
        pce_index += 1;
    }
    println!("Total moves {}", &list.count);
}