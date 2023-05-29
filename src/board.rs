use crate::data::*;

pub struct Board {
    pub pieces: [i32; 64],
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

pub fn move_bytes(from: i32, to: i32, capture: i32, promote: i32, fl: i32) -> i32 {
    (from) | ((to) << 7) | ( (capture) << 14 ) | ( (promote) << 20 ) | (fl)
}

pub fn from_square(m: i32) -> usize {
    (m & 0x7F).try_into().unwrap()
}

pub fn to_square(m: i32) -> usize {
    (m >> 7 & 0x7F).try_into().unwrap()
}

fn add_quiet_move(m: i32, list: &mut MoveList) {

    list.moves[list.count].m = m;
    list.count += 1;
}

pub fn generate_moves(pos: &mut Board, list: &mut MoveList) {
    for sq in 0..63 {
        if pos.pieces[sq] == wP {
            if rank(sq.try_into().unwrap()) == RANK_2 {
                add_quiet_move(move_bytes(sq.try_into().unwrap(), (sq+16).try_into().unwrap(),0,0,0), list);
                println!("{}", sq);
            }
            
        }
    }
    println!("{}", list.count);
}