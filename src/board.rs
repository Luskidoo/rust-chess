pub use crate::data::*;
pub use crate::validate::*;
use rand::*;
use ndarray::*;
use once_cell::sync::Lazy;
pub use crate::movegen::*;

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