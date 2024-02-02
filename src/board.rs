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

pub const knight_dir: [i32; 8] = [
    -17, -15, -10, -6, 6, 10, 15, 17
];

pub const bishop_dir: [i32; 4] = [
    -9, -7, 7, 9
];

pub const rook_dir: [i32; 4] = [
    -8, -1, 1, 8
];

pub const queen_dir: [i32; 8] = [
    -9, -8, -7, -1, 1, 7, 8, 9
];

pub const king_dir: [i32; 8] = [
    -9, -8, -7, -1, 1, 7, 8, 9
];

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

fn add_capture_move(m: i32, mut list: &mut MoveList) {

    list.moves[list.count].m = m;
    // Add MvvLVA
    list.count += 1;
}

fn add_ep_move(m: i32, mut list: &mut MoveList) {

    list.moves[list.count].m = m;
    list.moves[list.count].score = 105 + 100000;
    list.count += 1;
}

fn add_white_pawn_move(from: i32, to: i32, list: &mut MoveList) {

    if rank(from) == RANK_7 {
        for i in wN..=wQ {
            add_quiet_move(move_bytes(from, to, EMPTY, i, 0), list);
        }
    } 
    else {
        add_quiet_move(move_bytes(from, to, EMPTY, EMPTY, 0), list);
    }    
}

fn add_white_pawn_cap_move(from: i32, to: i32, cap: i32, list: &mut MoveList) {

    if rank(from) == RANK_7 {
        for i in wN..=wQ {
            add_capture_move(move_bytes(from, to, EMPTY, i, 0), list);
        }
    } 
    else {
        add_capture_move(move_bytes(from, to, EMPTY, EMPTY, 0), list);
    }   
}

fn add_black_pawn_move(from: i32, to: i32, list: &mut MoveList) {

    if rank(from) == RANK_2 {
        for i in bN..=bQ {
            add_quiet_move(move_bytes(from, to, EMPTY, i, 0), list);
        }
    } 
    else {
        add_quiet_move(move_bytes(from, to, EMPTY, EMPTY, 0), list);
    }    
}

fn add_black_pawn_cap_move(from: i32, to: i32, cap: i32, list: &mut MoveList) {

    if rank(from) == RANK_2 {
        for i in bN..=bQ {
            add_capture_move(move_bytes(from, to, EMPTY, i, 0), list);
        }
    } 
    else {
        add_capture_move(move_bytes(from, to, EMPTY, EMPTY, 0), list);
    }   
}

pub fn generate_moves(pos: &mut Board, list: &mut MoveList) {
    let mut t_sq: i32 = 0;
    for sq in 0..63 {
        if pos.pieces[sq] == wP {
            if file(sq.try_into().unwrap()) != FILE_H && piece_col[pos.pieces[sq + 9] as usize] == BLACK {
                add_white_pawn_cap_move(sq.try_into().unwrap(), (sq + 9).try_into().unwrap(), pos.pieces[sq+9], list);
            }
            if file(sq.try_into().unwrap()) != FILE_A && piece_col[pos.pieces[sq + 7] as usize] == BLACK {
                add_white_pawn_cap_move(sq.try_into().unwrap(), (sq + 7).try_into().unwrap(), pos.pieces[sq+7], list);
            }  
            if pos.pieces[sq + 8] == EMPTY {
                add_white_pawn_move(sq.try_into().unwrap(), (sq + 8).try_into().unwrap(), list);
                if rank(sq.try_into().unwrap()) == RANK_2 && (pos.pieces[(sq + 16) as usize] == EMPTY) {
                    add_quiet_move(move_bytes(sq.try_into().unwrap(), (sq + 16).try_into().unwrap(), EMPTY, EMPTY, MFLAGPS), list);
                }    
            }
        }

        if pos.pieces[sq] == wN {
            for wn_dir in knight_dir {
                t_sq = sq as i32;
                if mailbox[(mailbox64[sq] + wn_dir) as usize] == -1 {
                    continue
                }
                else if pos.pieces[(sq as i32 + wn_dir) as usize] != EMPTY {
                    if piece_col[pos.pieces[(sq as i32 + wn_dir) as usize] as usize] == BLACK {
                        add_capture_move(move_bytes(sq.try_into().unwrap(), (sq as i32 + wn_dir).try_into().unwrap(), 0, 0, 0), list);
                    }
                }
                else {
                    add_quiet_move(move_bytes(sq.try_into().unwrap(), (sq as i32 + wn_dir).try_into().unwrap(), 0, 0, 0), list);
                }
            }
        }

        if pos.pieces[sq] == wB {
            for wb_dir in bishop_dir {
                t_sq = sq as i32;
                while mailbox[(mailbox64[t_sq as usize] + wb_dir) as usize] != -1 {
                    if (t_sq as i32) + wb_dir < 0 || (t_sq as i32) + wb_dir > 63 {
                        break 
                    }
                    if pos.pieces[(t_sq as i32 + wb_dir) as usize] != EMPTY {
                        if piece_col[pos.pieces[(t_sq as i32 + wb_dir) as usize] as usize] == BLACK {
                            add_capture_move(move_bytes(t_sq.try_into().unwrap(), (t_sq as i32 + wb_dir).try_into().unwrap(), 0, 0, 0), list);
                        }
                        else {
                            break
                        }
                    }
                    else {
                        add_quiet_move(move_bytes(t_sq.try_into().unwrap(), (t_sq as i32 + wb_dir).try_into().unwrap(), 0, 0, 0), list);
                    }
                    t_sq += wb_dir;
                }
            }
        }

        if pos.pieces[sq] == wR {
            for wr_dir in rook_dir {
                t_sq = sq as i32;
                while mailbox[(mailbox64[t_sq as usize] + wr_dir) as usize] != -1 {
                    if (t_sq as i32) + wr_dir < 0 || (t_sq as i32) + wr_dir > 63 {
                        break 
                    }
                    if pos.pieces[(t_sq as i32 + wr_dir) as usize] != EMPTY {
                        if piece_col[pos.pieces[(t_sq as i32 + wr_dir) as usize] as usize] == BLACK {
                            add_capture_move(move_bytes(t_sq.try_into().unwrap(), (t_sq as i32 + wr_dir).try_into().unwrap(), 0, 0, 0), list);
                        }
                        else {
                            break
                        }
                    }
                    else {
                        add_quiet_move(move_bytes(t_sq.try_into().unwrap(), (t_sq as i32 + wr_dir).try_into().unwrap(), 0, 0, 0), list);
                    }
                    t_sq += wr_dir;
                }
            }
        }

        if pos.pieces[sq] == wQ {
            for wq_dir in queen_dir {
                t_sq = sq as i32;
                while mailbox[(mailbox64[t_sq as usize] + wq_dir) as usize] != -1 {
                    if (t_sq as i32) + wq_dir < 0 || (t_sq as i32) + wq_dir > 63 {
                        break 
                    }
                    println!("{}", t_sq as i32);
                    //println!("{}", mailbox[(mailbox64[t_sq as usize] + wq_dir) as usize]);
                    if pos.pieces[(t_sq as i32 + wq_dir) as usize] != EMPTY {
                        
                        if piece_col[pos.pieces[(t_sq as i32 + wq_dir) as usize] as usize] == BLACK {
                            add_capture_move(move_bytes(t_sq.try_into().unwrap(), (t_sq as i32 + wq_dir).try_into().unwrap(), 0, 0, 0), list);
                        }
                        else {
                            break
                        }
                    }
                    else {
                        add_quiet_move(move_bytes(t_sq.try_into().unwrap(), (t_sq as i32 + wq_dir).try_into().unwrap(), 0, 0, 0), list);
                    }
                    t_sq += wq_dir;
                }
            }
        }

        if pos.pieces[sq] == wK {
            for wk_dir in king_dir {
                t_sq = sq as i32;
                if mailbox[(mailbox64[sq] + wk_dir) as usize] == -1 {
                    continue
                }
                else if pos.pieces[(sq as i32 + wk_dir) as usize] != EMPTY {
                    if piece_col[pos.pieces[(sq as i32 + wk_dir) as usize] as usize] == BLACK {
                        add_capture_move(move_bytes(sq.try_into().unwrap(), (sq as i32 + wk_dir).try_into().unwrap(), 0, 0, 0), list);
                    }
                }
                else {
                    add_quiet_move(move_bytes(sq.try_into().unwrap(), (sq as i32 + wk_dir).try_into().unwrap(), 0, 0, 0), list);
                }
            }
        }

        if pos.pieces[sq] == bP {
            if file(sq.try_into().unwrap()) != FILE_H && piece_col[pos.pieces[sq - 7] as usize] == BLACK {
                add_black_pawn_cap_move(sq.try_into().unwrap(), (sq - 7).try_into().unwrap(), pos.pieces[sq-7], list);
            }
            if file(sq.try_into().unwrap()) != FILE_A && piece_col[pos.pieces[sq - 9] as usize] == BLACK {
                add_black_pawn_cap_move(sq.try_into().unwrap(), (sq - 9).try_into().unwrap(), pos.pieces[sq-9], list);
            }  
            if pos.pieces[sq - 8] == EMPTY {
                add_black_pawn_move(sq.try_into().unwrap(), (sq - 8).try_into().unwrap(), list);
                if rank(sq.try_into().unwrap()) == RANK_7 && (pos.pieces[(sq - 16) as usize] == EMPTY) {
                    add_quiet_move(move_bytes(sq.try_into().unwrap(), (sq - 16).try_into().unwrap(), EMPTY, EMPTY, MFLAGPS), list);
                }    
            }
        }

        if pos.pieces[sq] == bN {
            for bn_dir in knight_dir {
                t_sq = sq as i32;
                if mailbox[(mailbox64[sq] + bn_dir) as usize] == -1 {
                    continue
                }
                else if pos.pieces[(sq as i32 + bn_dir) as usize] != EMPTY {
                    if piece_col[pos.pieces[(sq as i32 + bn_dir) as usize] as usize] == WHITE {
                        add_capture_move(move_bytes(sq.try_into().unwrap(), (sq as i32 + bn_dir).try_into().unwrap(), 0, 0, 0), list);
                    }
                }
                else {
                    add_quiet_move(move_bytes(sq.try_into().unwrap(), (sq as i32 + bn_dir).try_into().unwrap(), 0, 0, 0), list);
                }
            }
        }

        if pos.pieces[sq] == bB {
            for bb_dir in bishop_dir {
                t_sq = sq as i32;
                while mailbox[(mailbox64[t_sq as usize] + bb_dir) as usize] != -1 {
                    if (t_sq as i32) + bb_dir < 0 || (t_sq as i32) + bb_dir > 63 {
                        break 
                    }
                    if pos.pieces[(t_sq as i32 + bb_dir) as usize] != EMPTY {
                        if piece_col[pos.pieces[(t_sq as i32 + bb_dir) as usize] as usize] == WHITE {
                            add_capture_move(move_bytes(t_sq.try_into().unwrap(), (t_sq as i32 + bb_dir).try_into().unwrap(), 0, 0, 0), list);
                        }
                        else {
                            break
                        }
                    }
                    else {
                        add_quiet_move(move_bytes(t_sq.try_into().unwrap(), (t_sq as i32 + bb_dir).try_into().unwrap(), 0, 0, 0), list);
                    }
                    t_sq += bb_dir;
                }
            }
        }

        if pos.pieces[sq] == bR {
            for br_dir in rook_dir {
                t_sq = sq as i32;
                while mailbox[(mailbox64[t_sq as usize] + br_dir) as usize] != -1 {
                    if (t_sq as i32) + br_dir < 0 || (t_sq as i32) + br_dir > 63 {
                        break 
                    }
                    if pos.pieces[(t_sq as i32 + br_dir) as usize] != EMPTY {
                        if piece_col[pos.pieces[(t_sq as i32 + br_dir) as usize] as usize] == WHITE {
                            add_capture_move(move_bytes(t_sq.try_into().unwrap(), (t_sq as i32 + br_dir).try_into().unwrap(), 0, 0, 0), list);
                        }
                        else {
                            break
                        }
                    }
                    else {
                        add_quiet_move(move_bytes(t_sq.try_into().unwrap(), (t_sq as i32 + br_dir).try_into().unwrap(), 0, 0, 0), list);
                    }
                    t_sq += br_dir;
                }
            }
        }

        if pos.pieces[sq] == bQ {
            for bq_dir in queen_dir {
                t_sq = sq as i32;
                while mailbox[(mailbox64[t_sq as usize] + bq_dir) as usize] != -1 {
                    if (t_sq as i32) + bq_dir < 0 || (t_sq as i32) + bq_dir > 63 {
                        break 
                    }
                    if pos.pieces[(t_sq as i32 + bq_dir) as usize] != EMPTY {
                        if piece_col[pos.pieces[(t_sq as i32 + bq_dir) as usize] as usize] == WHITE {
                            add_capture_move(move_bytes(t_sq.try_into().unwrap(), (t_sq as i32 + bq_dir).try_into().unwrap(), 0, 0, 0), list);
                        }
                        else {
                            break
                        }
                    }
                    else {
                        add_quiet_move(move_bytes(t_sq.try_into().unwrap(), (t_sq as i32 + bq_dir).try_into().unwrap(), 0, 0, 0), list);
                    }
                    t_sq += bq_dir;
                }
            }
        }

        if pos.pieces[sq] == bK {
            for dir in king_dir {
                t_sq = sq as i32;
                if mailbox[(mailbox64[sq] + dir) as usize] == -1 {
                    continue
                }
                else if pos.pieces[(sq as i32 + dir) as usize] != EMPTY {
                    if piece_col[pos.pieces[(sq as i32 + dir) as usize] as usize] == WHITE {
                        add_capture_move(move_bytes(sq.try_into().unwrap(), (sq as i32 + dir).try_into().unwrap(), 0, 0, 0), list);
                    }
                }
                else {
                    add_quiet_move(move_bytes(sq.try_into().unwrap(), (sq as i32 + dir).try_into().unwrap(), 0, 0, 0), list);
                }
            }
        }
    }
    println!("Total moves = {}", list.count);
}