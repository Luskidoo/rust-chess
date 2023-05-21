pub use crate::data;
use rand::*;
use ndarray::*;
use once_cell::sync::Lazy;
use std::char::from_digit;

#[derive(Copy, Clone, Debug)]
pub struct MoveBytes{
	from: char,
	to: char,
	promote: char,
	bits: char,
}
#[derive(Copy, Clone, Debug)]
pub struct Move {
	b: MoveBytes,
	u: i32,
}

/* an element of the move stack. it's just a move with a
   score, so it can be sorted by the search functions. */
#[derive(Copy, Clone, Debug)]
pub struct GenT{
	m: Move,
	score: i32,
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

const empty_gent: GenT = GenT {
    m: empty_move,
    score: 0,
};

const empty_move: Move = Move {
    b: empty_move_bytes,
	u: 0,
};

const empty_move_bytes: MoveBytes = MoveBytes{
	from: 'a',
	to: 'a',
	promote: 'a',
	bits: 'a',
};

/* gen_dat is some memory for move lists that are created by the move
   generators. The move list for ply n starts at first_move[n] and ends
   at first_move[n + 1]. */

static mut gen_dat: [GenT; data::GEN_STACK] = [empty_gent; data::GEN_STACK];

static init_color: [i32; 64] = [
	1, 1, 1, 1, 1, 1, 1, 1,
	1, 1, 1, 1, 1, 1, 1, 1,
	6, 6, 6, 6, 6, 6, 6, 6,
	6, 6, 6, 6, 6, 6, 6, 6,
	6, 6, 6, 6, 6, 6, 6, 6,
	6, 6, 6, 6, 6, 6, 6, 6,
	0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0
];

static init_piece: [i32; 64] = [	
    3, 1, 2, 4, 5, 2, 1, 3,
	0, 0, 0, 0, 0, 0, 0, 0,
	6, 6, 6, 6, 6, 6, 6, 6,
	6, 6, 6, 6, 6, 6, 6, 6,
	6, 6, 6, 6, 6, 6, 6, 6,
	6, 6, 6, 6, 6, 6, 6, 6,
	0, 0, 0, 0, 0, 0, 0, 0,
	3, 1, 2, 4, 5, 2, 1, 3
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

static mut first_move: [i32; data::MAX_PLY] = [0; data::MAX_PLY];

/* slide, offsets, and offset are basically the vectors that
  pieces can move in. If slide for the piece is FALSE, it can
  only move one square in any one direction. offsets is the
  number of directions it can move in, and offset is an array
  of the actual directions. */

static slide: [usize; 6] = [
   data::FALSE, data::FALSE, data::TRUE, data::TRUE, data::TRUE, data::FALSE
];

static offsets: [i32; 6] = [
   0, 8, 4, 4, 8, 8
];

static offset: [[i32; 8]; 6] = [
   [ 0, 0, 0, 0, 0, 0, 0, 0 ],
   [ -21, -19, -12, -8, 8, 12, 19, 21 ],
   [ -11, -9, 9, 11, 0, 0, 0, 0 ],
   [ -10, -1, 1, 10, 0, 0, 0, 0 ],
   [ -11, -10, -9, -1, 1, 9, 10, 11 ],
   [ -11, -10, -9, -1, 1, 9, 10, 11 ]
];

static mut history: Lazy<Array2<i32>> = Lazy::new(|| Array2::<i32>::zeros((64, 64)));

/* the board representation */
pub static mut color: [i32; 64] = [0; 64];  /* LIGHT, DARK, or EMPTY */
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
static mut hash_piece: Lazy<Array3<i32>> = Lazy::new(|| Array3::<i32>::zeros((2, 6, 64)));  /* indexed by piece [color][type][square] */
static mut hash_side: i32 = 0;
static mut hash_ep: [i32; 64] = [0; 64];

static mut count_moves: i32 = 0;

pub fn init_board() {
    unsafe {
        for i in 0..64 {
            color[i] = init_color[i];
            piece[i] = init_piece[i];
        }
        side = data::LIGHT;
        xside = data::DARK;
        castle = 15;
        ep = -1;
        fifty = 0;
        ply = 0;
        hply = 0;
        set_hash();  /* init_hash() must be called before this function */
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
	for i in 0..64{
        if color[i] != data::EMPTY {
            hp = hash_piece[[color[i] as usize, piece[i] as usize, i]];
            local_hash ^= hp;
        }		
        if side == data::DARK {
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

fn Move(from: i32, to: i32, capture: i32, promote: i32, fl: i32) -> i32 {
    (from) | ((to) << 7) | ( (capture) << 14 ) | ( (promote) << 20 ) | (fl)
}

/* in_check() returns TRUE if side s is in check and FALSE
   otherwise. It just scans the board to find side s's king
   and calls attack() to see if it's being attacked. */

unsafe fn in_check(s: i32) -> usize { 
    for i in 0..64 {
        if piece[i] == data::KING && color[i] == s {
            return attack(i.try_into().unwrap(), s ^ 1);
        }
    }
    return data::TRUE;  /* shouldn't get here */
}

unsafe fn attack(sq: i32, s: i32) -> usize {
    let mut n: i32 = 0;
	for i in 0..64 {
        if color[i] == s {
			if piece[i] == data::PAWN {
				if s == data::LIGHT {
					if data::COL(i) != 0 && i - 9 == sq.try_into().unwrap() {
                        return data::TRUE;
                    }
						
					if data::COL(i) != 7 && i - 7 == sq.try_into().unwrap() {
                        return data::TRUE;
                    }
						
				}
				else {
					if data::COL(i) != 0 && i + 7 == sq.try_into().unwrap() {
                        return data::TRUE;
                    }
						
					if data::COL(i) != 7 && i + 9 == sq.try_into().unwrap() {
                        return data::TRUE;
                    }	
				}
			}
			else {
                for j in 0..offsets[piece[i] as usize] {
                    n = mailbox[mailbox64[n as usize] + (offset[piece[i] as usize][j as usize] as usize)];
                    if n == -1 {
                        break;
                    }
                        
                    if n == sq {
                        return data::TRUE;
                    }
                        
                    if color[n as usize] != data::EMPTY {
                        break;
                    }
                        
                    if slide[piece[i] as usize] == data::FALSE {
                        break;
                    }
                }
            }
				
        }
    }
    return data::FALSE;
}

/* gen_push() puts a move on the move stack, unless it's a
   pawn promotion that needs to be handled by gen_promote().
   It also assigns a score to the move for alpha-beta move
   ordering. If the move is a capture, it uses MVV/LVA
   (Most Valuable Victim/Least Valuable Attacker). Otherwise,
   it uses the move's history heuristic value. Note that
   1,000,000 is added to a capture move's score, so it
   always gets ordered above a "normal" move. */

unsafe fn gen_push(from: i32, to: i32, bits: i32)
{
  
    let mut g = empty_gent;
	
	if bits & 16 == 1 {
		if side == data::LIGHT {
			if to <= data::H8 {
				gen_promote(from, to, bits);
				return;
			}
		}
		else {
			if to >= data::A1 {
				gen_promote(from, to, bits);
				return;
			}
		}
	}

    // check this line
	g = gen_dat[first_move[ply + 1] as usize];

    println!("ply {0}", ply);
    println!("from {}", from);
    println!("from {0}", from_digit(from.try_into().unwrap(), 2).unwrap());
    println!("to {0}", to);
    println!("bits {0}", bits);
    println!("g {:?}", g);

	g.m.b.from = from_digit(from.try_into().unwrap(), 10).unwrap();
	g.m.b.to = from_digit(to.try_into().unwrap(), 10).unwrap();
	g.m.b.promote = from_digit(0, 10).unwrap();
	g.m.b.bits = from_digit(bits.try_into().unwrap(), 10).unwrap();

	if color[to as usize] != data::EMPTY {
        g.score = 1000000 + (piece[to as usize] * 10) - piece[from as usize];
    }
	else {
        g.score = history[[from as usize, to as usize]];
    }
    count_moves = count_moves + 1;
    println!("Moves: {}", count_moves);
}

/* gen_promote() is just like gen_push(), only it puts 4 moves
   on the move stack, one for each possible promotion piece */

unsafe fn gen_promote(from: i32, to: i32, bits: i32) {
    let mut g = empty_gent;
    
    for i in data::KNIGHT..data::QUEEN {
        g = gen_dat[first_move[ply + 1] as usize];
        g.m.b.from = from_digit(from.try_into().unwrap(), 10).unwrap();
        g.m.b.to = from_digit(to.try_into().unwrap(), 10).unwrap();
        g.m.b.promote = from_digit(i.try_into().unwrap(), 10).unwrap();
        g.m.b.bits = from_digit((bits | 32).try_into().unwrap(), 10).unwrap();
        g.score = 1000000 + (i * 10);
    }
}

/* gen() generates pseudo-legal moves for the current position.
   It scans the board to find friendly pieces and then determines
   what squares they attack. When it finds a piece/square
   combination, it calls gen_push to put the move on the "move
   stack." */

pub unsafe fn gen()
{
    /* so far, we have no moves for the current ply */
    first_move[ply + 1] = first_move[ply];
    let mut n: i32 = 0;
    for i in 0..64 {
        println!("i = {}", i);
        if color[i] == side {
            if piece[i] == data::PAWN {
                if side == data::LIGHT {
                    if data::COL(i) != 0 && color[(i - 9) as usize] == data::DARK {
                        gen_push(i.try_into().unwrap(), (i - 9).try_into().unwrap(), 17);
                    } 
                    if data::COL(i) != 7 && color[(i - 7) as usize] == data::DARK {
                        gen_push(i.try_into().unwrap(), (i - 7).try_into().unwrap(), 17);
                    } 
                    if color[i - 8] == data::EMPTY {
                        println!("color[(i - 16) as usize] {}", color[(i - 16) as usize]);
                        gen_push(i.try_into().unwrap(), (i - 8).try_into().unwrap(), 16);
                        if (i >= 48) && (color[(i - 16) as usize] == data::EMPTY) {
                            gen_push(i.try_into().unwrap(), (i - 16).try_into().unwrap(), 24);
                        }    
                    }
                }
                else {
                    if data::COL(i) != 0 && color[(i + 7) as usize] == data::LIGHT {
                        gen_push(i.try_into().unwrap(), (i + 7).try_into().unwrap(), 17);
                    }
                    if data::COL(i) != 7 && color[(i + 9) as usize] == data::LIGHT {
                        gen_push(i.try_into().unwrap(), (i + 9).try_into().unwrap(), 17);
                    }  
                    if color[i + 8] == data::EMPTY {
                        gen_push(i.try_into().unwrap(), (i + 8).try_into().unwrap(), 16);
                        if i <= 15 && color[(i + 16) as usize] == data::EMPTY {
                            gen_push(i.try_into().unwrap(), (i + 16).try_into().unwrap(), 24);
                        }
                    }
                }
            }
            else {
                for j in 0..offsets[piece[i as usize] as usize] {
                    n = mailbox[mailbox64[n as usize] + (offset[piece[i] as usize][j as usize] as usize)];
                        if n == -1 {
                            break;
                        }
                        if color[n as usize] != data::EMPTY {
                            if color[n as usize] == xside {
                                gen_push(i.try_into().unwrap(), n.try_into().unwrap(), 1);
                            }
                            break;
                        }
                        gen_push(i.try_into().unwrap(), n.try_into().unwrap(), 0);
                        if slide[piece[i as usize]  as usize] == data::FALSE {
                            break;
                        }  
                    }
                }  
            }      
        }
        

    /* generate castle moves */
    if side == data::LIGHT {
        if castle & 1 == 1 {
            gen_push(data::E1, data::G1, 2);
        }
        if castle & 2 == 1 {
            gen_push(data::E1, data::C1, 2);
        }
    }
    else {
        if castle & 4 == 1 {
            gen_push(data::E8, data::G8, 2);
        }
            
        if castle & 8 == 1 {
            gen_push(data::E8, data::C8, 2);
        } 
    }
    
    /* generate en passant moves */
    if ep != -1 {
        if side == data::LIGHT {
            if data::COL(ep.try_into().unwrap()) != 0 && color[(ep + 7) as usize] == data::LIGHT && piece[(ep + 7) as usize] == data::PAWN {
                gen_push((ep + 7).try_into().unwrap(), ep.try_into().unwrap(), 21);
            }  
            if data::COL(ep.try_into().unwrap()) != 7 && color[(ep + 9) as usize] == data::LIGHT && piece[(ep + 9) as usize] == data::PAWN {
                gen_push((ep + 9).try_into().unwrap(), ep.try_into().unwrap(), 21);
            }
        }
        else {
            if data::COL(ep.try_into().unwrap()) != 0 && color[(ep - 9) as usize] == data::DARK && piece[(ep - 9) as usize] == data::PAWN {
                gen_push((ep - 9).try_into().unwrap(), ep.try_into().unwrap(), 21);
            }
            if data::COL(ep.try_into().unwrap()) != 7 && color[(ep - 7) as usize] == data::DARK && piece[(ep - 7) as usize] == data::PAWN {
                gen_push((ep - 7).try_into().unwrap(), ep.try_into().unwrap(), 21);
            }
        }
    }
}
		
	
