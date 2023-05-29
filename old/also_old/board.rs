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
    pub pList: Lazy<Array2<i32>>,

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
	0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0,
	6, 6, 6, 6, 6, 6, 6, 6,
	6, 6, 6, 6, 6, 6, 6, 6,
	6, 6, 6, 6, 6, 6, 6, 6,
	6, 6, 6, 6, 6, 6, 6, 6,
	1, 1, 1, 1, 1, 1, 1, 1,
	1, 1, 1, 1, 1, 1, 1, 1,
];

static init_piece: [i32; 64] = [	
    4, 2, 3, 5, 6, 3, 2, 4,
	1, 1, 1, 1, 1, 1, 1, 1,
	0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0,
	7, 7, 7, 7, 7, 7, 7, 7,
	10, 8, 9, 11, 12, 9, 8, 10
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

static loopSlidePce: [i32; 8] = [
    wB, wR, wQ, 0, bB, bR, bQ, 0
];
   
static loopNonSlidePce: [i32; 6] = [
    wN, wK, 0, bN, bK, 0
];

static loopSlideIndex: [i32; 2] = [ 0, 4 ];
static loopNonSlideIndex: [i32; 2] = [ 0, 3 ];

static pceDir: [[i32; 8]; 13] = [
	[ 0, 0, 0, 0, 0, 0, 0, 0 ],
	[ 0, 0, 0, 0, 0, 0, 0, 0 ],
	[ -21, -19, -12, -8, 8, 12, 19, 21 ],
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

static numDir: [i32; 13] = [
 0, 0, 8, 4, 4, 8, 8, 0, 8, 4, 4, 8, 8
];

static mut history: Lazy<Array2<i32>> = Lazy::new(|| Array2::<i32>::zeros((64, 64)));

/* the board representation */
pub static mut color: [i32; 64] = [0; 64];  /* WHITE, BLACK, or EMPTY */
pub static mut piece: [i32; 64] = [0; 64];  /* PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING, or EMPTY */
static mut side: i32 = 0;  /* the side to move */
static mut xside: i32 = 0;  /* the side not to move */
static mut castle: usize = 0;  /* a bitfield with the castle permissions. if 1 is set,
                white can still castle kingside. 2 is white queenside.
				4 is black kingside. 8 is black queenside. */
static mut ep: isize = 0;  /* the en passant square. if white moves e2e4, the en passant
            square is set to e3, because that's where a pawn would move
			in an en passant capture */
static mut fifty: usize = 0;  /* the number of moves since a capture or pawn move, used
               to handle the fifty-move-draw rule */
static mut hash: usize = 0;  /* a (more or less) unique number that corresponds to the
              position */
static mut ply: usize = 0;  /* the number of half-moves (ply) since the
             root of the search tree */
static mut hply: usize = 0;  /* h for history; the number of ply since the beginning
              of the game */

/* random numbers used to compute hash; see set_hash() in board.c */
static mut hash_piece: Lazy<Array3<i32>> = Lazy::new(|| Array3::<i32>::zeros((2, 13, 64)));  /* indexed by piece [color][type][square] */
static mut hash_side: i32 = 0;
static mut hash_ep: [i32; 64] = [0; 64];

static mut count_moves: i32 = 0;

pub fn init_board() {
    unsafe {
        for i in 0..64 {
            color[i] = init_color[i];
            piece[i] = init_piece[i];
        }
        side = WHITE;
        xside = BLACK;
        castle = 15;
        ep = -1;
        fifty = 0;
        ply = 0;
        hply = 0;
        //set_hash();  /* init_hash() must be called before this function */
        first_move[0] = 0;
    }
	
}

fn hash_rand() -> i32 {
	let mut r: i32 = 0;
    let mut r2: i32;

	for i in 0..32 {
        r2 = random();
        r ^= r2 << i;
    }   
	r
}

unsafe fn set_hash() {
    let mut hp: i32 = 0;
	let mut local_hash: i32 = 0;	
	for i in 0..=64{
        if color[i] != EMPTY {
            hp = hash_piece[[color[i] as usize, piece[i] as usize, i]];
            local_hash ^= hp;
        }		
        if side == BLACK {
            local_hash ^= hash_side;
        }

        if ep != -1 {
            local_hash ^= hash_ep[ep as usize];
        }	
    }
}

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

    assert!(sq_on_board(from_sq(m)));
    assert!(sq_on_board(to_sq(m)));

    list.moves[list.count].m = m;
    list.moves[list.count].score = history[[from_square(m), to_square(m)]];
    list.count += 1;
}

unsafe fn add_capture_move(m: i32, mut list: &mut MoveList) {

    assert!(sq_on_board(from_sq(m)));
    assert!(sq_on_board(to_sq(m)));

    list.moves[list.count].m = m;
    // Add MvvLVA
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

    assert!(sq_on_board(from));
    assert!(sq_on_board(to));

    if ranksbrd[from as usize] == RANK_7 {
        for i in wN..=wQ {
            add_quiet_move(move_bytes(from, to, EMPTY, i, 0), list);
        }
    } 
    else {
        add_quiet_move(move_bytes(from, to, EMPTY, EMPTY, 0), list);
    }    
}

unsafe fn add_white_pawn_cap_move(from: i32, to: i32, cap: i32, list: &mut MoveList) {

    assert!(sq_on_board(from));
    assert!(sq_on_board(to));

    if ranksbrd[from as usize] == RANK_7 {
        for i in wN..=wQ {
            add_capture_move(move_bytes(from, to, EMPTY, i, 0), list);
        }
    } 
    else {
        add_capture_move(move_bytes(from, to, EMPTY, EMPTY, 0), list);
    }   
}

unsafe fn add_black_pawn_move(from: i32, to: i32, list: &mut MoveList) {

    assert!(sq_on_board(from));
    assert!(sq_on_board(to));

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

    assert!(sq_on_board(from));
    assert!(sq_on_board(to));

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

/* gen() generates pseudo-legal moves for the current position.
   It scans the board to find friendly pieces and then determines
   what squares they attack. When it finds a piece/square
   combination, it calls gen_push to put the move on the "move
   stack." */

pub unsafe fn gen(list: &mut MoveList)
{
    list.count = 0;
    /* so far, we have no moves for the current ply */
    first_move[ply + 1] = first_move[ply];
    let mut n: i32 = 0;
    for i in 0..64 {
        if side == WHITE {
            if piececol[piece[i] as usize] == side {
                if piece[i] == wP {
                    println!("i {}", i);
                    if COL(i) != 0 && piececol[piece[(i + 9) as usize] as usize] == BLACK {
                        add_white_pawn_cap_move(i.try_into().unwrap(), (i + 9).try_into().unwrap(), piece[i+9], list);
                    } 
                    if COL(i) != 7 && piececol[piece[(i + 11) as usize] as usize] == BLACK {
                        add_white_pawn_cap_move(i.try_into().unwrap(), (i + 11).try_into().unwrap(), piece[i+11], list);
                    } 
                    if piece[i + 8] == EMPTY {
                        add_white_pawn_move(i.try_into().unwrap(), (i + 16).try_into().unwrap(), list);
                        if ranksbrd[i] == RANK_2 && (piece[(i + 16) as usize] == EMPTY) {
                            println!("TEST");
                            add_quiet_move(move_bytes(i.try_into().unwrap(), (i + 16).try_into().unwrap(), EMPTY, EMPTY, MFLAGPS), list);
                        }    
                    }
                }

                /* generate castle moves */
                if castle & 1 == 1 {
                    if piece[F1 as usize] == EMPTY && piece[G1 as usize] == EMPTY {
                        add_quiet_move(move_bytes(E1, G1, EMPTY, EMPTY, MFLAGCA), list);
                    }
                }
                if castle & 2 == 1 {
                    add_quiet_move(move_bytes(E1, C1, EMPTY, EMPTY, MFLAGCA), list);
                }
                // generate en passent moves
                if ep != -1 {
                    if COL(ep.try_into().unwrap()) != 0 && piece[(ep + 7) as usize] == WHITE && piece[(ep + 7) as usize] == wP {
                        add_ep_move(move_bytes((ep + 7).try_into().unwrap(), ep.try_into().unwrap(), EMPTY, EMPTY, MFLAGEP), list);
                    }  
                    if COL(ep.try_into().unwrap()) != 7 && piece[(ep + 9) as usize] == WHITE && piece[(ep + 9) as usize] == wP {
                        add_ep_move(move_bytes((ep + 9).try_into().unwrap(), ep.try_into().unwrap(), EMPTY, EMPTY, MFLAGEP), list);
                    }
                }
            }
        } 
        else {
            if piece[i] == bP {
                println!("i {}", i);
                if COL(i) != 0 && piece[(i + 7) as usize] == WHITE {
                    add_black_pawn_cap_move(i.try_into().unwrap(), (i + 7).try_into().unwrap(), piece[i+7], list);
                }
                if COL(i) != 7 && piece[(i + 9) as usize] == WHITE {
                    add_black_pawn_cap_move(i.try_into().unwrap(), (i + 9).try_into().unwrap(), piece[i+9], list);
                }  
                if piece[i + 8] == EMPTY {
                    add_black_pawn_move(i.try_into().unwrap(), (i + 8).try_into().unwrap(), list);
                    if i <= 15 && piece[(i + 16) as usize] == EMPTY {
                        add_quiet_move(move_bytes(i.try_into().unwrap(), (i + 16).try_into().unwrap(), EMPTY, EMPTY, MFLAGPS), list);
                    }
                }
            }
            
            if castle & 4 == 1 {
                add_quiet_move(move_bytes(E8, G8, EMPTY, EMPTY, MFLAGCA), list);
            }
                        
            if castle & 8 == 1 {
                add_quiet_move(move_bytes(E8, C8, EMPTY, EMPTY, MFLAGCA), list);
            } 
                
                /* generate en passant moves */
                
            if ep != -1 {
                if COL(ep.try_into().unwrap()) != 0 && piece[(ep - 9) as usize] == BLACK && piece[(ep - 9) as usize] == bP {
                    add_ep_move(move_bytes((ep - 9).try_into().unwrap(), ep.try_into().unwrap(), EMPTY, EMPTY, MFLAGEP), list);
                }
                if COL(ep.try_into().unwrap()) != 7 && piece[(ep - 7) as usize] == BLACK && piece[(ep - 7) as usize] == bP {
                    add_ep_move(move_bytes((ep - 7).try_into().unwrap(), ep.try_into().unwrap(), EMPTY, EMPTY, MFLAGEP), list);
                }
            }
        }

        let mut pceIndex: i32 = loopSlideIndex[side as usize];
        let mut pce: i32 = 0;
        pceIndex += 1;
        let mut dir_i: i32 = 0;
        let mut dir: i32 = 0;
        let mut t_sq: i32 = 0;

        if loopSlidePce.contains(&piece[i]) && piece[i] != EMPTY {
            for j in 0..3 {
                pce = loopSlidePce[j as usize];
                while pce < 4 {
                    for k in 0..=numDir[pce as usize] {
                        println!("pce {}", pce);
                        //println!("k {}", k);
                        dir = pceDir[k as usize][pce as usize];
                        //dir = dir_i as usize;
                        t_sq = (i as i32) + dir;
                        
                        println!("i {}", i);
                        // println!("dir_i {}", dir_i);
                        // println!("dir {}", dir);
                        
                        
                        if t_sq >= 0 {
                            while(sq_on_board(t_sq)) {
                                println!("t_sq {}", t_sq);
                                if piece[t_sq as usize] != EMPTY {
                                    if piececol[piece[t_sq as usize] as usize] == side ^ 1 {
                                        println!("Added capture move");
                                        add_capture_move(move_bytes(i.try_into().unwrap(), t_sq.try_into().unwrap(), piece[t_sq as usize], EMPTY, 0), list);
                                    }
                                    break;
                                }
                                else {
                                    println!("Added quiet move");
                                    add_quiet_move(move_bytes(i.try_into().unwrap(), t_sq.try_into().unwrap(), EMPTY, EMPTY, 0), list);
                                t_sq += dir;
                                }
                            }
                        }
                    }
                    pce = loopSlidePce[pceIndex as usize];
                    pce += 1;
                }
            }  
        }  
        
        if loopNonSlidePce.contains(&piece[i]) && piece[i] != EMPTY {
            for j in 0..3 {
                pce = loopNonSlidePce[j as usize];
                while pce < 4 {
                    for k in 0..=numDir[pce as usize] {
                        println!("pce {}", pce);
                        //println!("k {}", k);
                        dir = pceDir[k as usize][pce as usize];
                        //dir = dir_i as usize;
                        t_sq = (i as i32) + dir;
                        
                        println!("i {}", i);
                        // println!("dir_i {}", dir_i);
                        // println!("dir {}", dir);
                        
                        
                        if t_sq >= 0 {
                            while(sq_on_board(t_sq)) {
                                println!("t_sq {}", t_sq);
                                if piece[t_sq as usize] != EMPTY {
                                    if piececol[piece[t_sq as usize] as usize] == side ^ 1 {
                                        println!("Added capture move");
                                        add_capture_move(move_bytes(i.try_into().unwrap(), t_sq.try_into().unwrap(), piece[t_sq as usize], EMPTY, 0), list);
                                    }
                                    break;
                                }
                                else {
                                    println!("Added quiet move");
                                    add_quiet_move(move_bytes(i.try_into().unwrap(), t_sq.try_into().unwrap(), EMPTY, EMPTY, 0), list);
                                t_sq += dir;
                                }
                            }
                        }
                    }
                    pce = loopNonSlidePce[pceIndex as usize];
                    pce += 1;
                }
            }  
        }
        


    }
    println!("Total moves {}", &list.count);
}
		
	
