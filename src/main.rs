pub mod data;
pub mod board;
pub mod macros;
pub mod bitboards;
mod validate;
pub mod movegen;
mod init;
use crate::init::*;
use crate::data::*;
use crate::board::*;
use crate::bitboards::*;
use ndarray::*;
use once_cell::sync::Lazy;
use fen::*;

macro_rules! fr2sq {
    ($file:expr, $rank:expr) => {
        (21 + $file) + <u32 as TryInto<usize>>::try_into(($rank * 10)).unwrap()
    };
}

fn piece_string(piece: &Option<Piece>) -> String {
    match piece {
        Some(piece) => piece.to_string(),
        None    => String::from("."),
    }
}

// fn get_piece_type(piece: &Option<Piece>) -> PieceKind {
//     match piece {
//         Some(piece) => piece.kind,
//         None        => piece.kind,
//     }
// }

fn piece_index(piece_string: String) -> usize {
    let index = piece_char.iter().position(|&x| x == piece_string.chars().next().unwrap());
    match index {
        Some(index) => index,
        None => 0,
    }
}

fn from_fen(fen: String, mut pos: Board) -> Board {
    let board = fen::BoardState::from_fen(&fen).unwrap();
    let masks = BitMasks::init_masks();
    for sq64 in 0..64 {
        let piece = piece_index(piece_string(&board.pieces[sq64])) as i32;
        match piece {

            wN => {
                pos.pieces[sq64_to_sq120(sq64) as usize] = wN;
                pos.pList[wN as usize][pos.pceNum[wN as usize] as usize] = sq64_to_sq120(sq64);
                pos.pceNum[wN as usize] += 1;
            }

            wB => {
                pos.pieces[sq64_to_sq120(sq64) as usize] = wB;
                pos.pList[wB as usize][pos.pceNum[wB as usize] as usize] = sq64_to_sq120(sq64);
                pos.pceNum[wB as usize] += 1;
            }

            wR => {
                pos.pieces[sq64_to_sq120(sq64) as usize] = wR;
                pos.pList[wR as usize][pos.pceNum[wR as usize] as usize] = sq64_to_sq120(sq64);
                pos.pceNum[wR as usize] += 1;
            }

            wQ => {
                pos.pieces[sq64_to_sq120(sq64) as usize] = wQ;
                pos.pList[wQ as usize][pos.pceNum[wQ as usize] as usize] = sq64_to_sq120(sq64);
                pos.pceNum[wQ as usize] += 1;
            }

            wK => {
                pos.pieces[sq64_to_sq120(sq64) as usize] = wK;
                pos.pList[wK as usize][pos.pceNum[wK as usize] as usize] = sq64_to_sq120(sq64);
                pos.pceNum[wK as usize] += 1;
            }

            wR => {
                pos.pieces[sq64_to_sq120(sq64) as usize] = wR;
                pos.pList[wR as usize][pos.pceNum[wR as usize] as usize] = sq64_to_sq120(sq64);
                pos.pceNum[wR as usize] += 1;
            }

            bN => {
                pos.pieces[sq64_to_sq120(sq64) as usize] = bN;
                pos.pList[bN as usize][pos.pceNum[bN as usize] as usize] = sq64_to_sq120(sq64);
                pos.pceNum[bN as usize] += 1;
            }

            bB => {
                pos.pieces[sq64_to_sq120(sq64) as usize] = bB;
                pos.pList[bB as usize][pos.pceNum[bB as usize] as usize] = sq64_to_sq120(sq64);
                pos.pceNum[bB as usize] += 1;
            }

            bR => {
                pos.pieces[sq64_to_sq120(sq64) as usize] = bR;
                pos.pList[bR as usize][pos.pceNum[bR as usize] as usize] = sq64_to_sq120(sq64);
                pos.pceNum[bR as usize] += 1;
            }

            bQ => {
                pos.pieces[sq64_to_sq120(sq64) as usize] = bQ;
                pos.pList[bQ as usize][pos.pceNum[bQ as usize] as usize] = sq64_to_sq120(sq64);
                pos.pceNum[bQ as usize] += 1;
            }

            bK => {
                pos.pieces[sq64_to_sq120(sq64) as usize] = bK;
                pos.pList[bK as usize][pos.pceNum[bK as usize] as usize] = sq64_to_sq120(sq64);
                pos.pceNum[bK as usize] += 1;
            }

            wP => {
                pos.pieces[sq64_to_sq120(sq64) as usize] = wP;
                set_bit(pos.pawns[WHITE as usize], sq64, masks);
                set_bit(pos.pawns[BOTH as usize], sq64, masks);
            }

            bP => {
                pos.pieces[sq64_to_sq120(sq64) as usize] = bP;
                set_bit(pos.pawns[BLACK as usize], sq64, masks);
                set_bit(pos.pawns[BOTH as usize], sq64, masks);
            }

            _ => (),
        }
    }
    return pos
}

pub fn main() {
    init_all();
    let masks = BitMasks::init_masks();
    let set_mask = masks.SetMask;
    let undo = Undo {
        m: 0,
        castlePerm: 0,
        enPas: NO_SQ,
    };

    let mut pos = Board::default();
    

    let mut list = board::MoveList {
        moves: [board::Move {m: 0, score: 0}; data::MAX_POSITION_MOVES],
        count: 0
    };
    
    let fen = String::from("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
    let board = fen::BoardState::from_fen(&fen).unwrap();
    print_board(board);
        
    pos = from_fen(fen, pos);
    
    println!("My board");
    print_my_board(pos);
    //board::init_board();
    //board::init_sq120_to_sq64();
    //board::init_files_ranks_board();
    //print_board();
    //board::generate_all_moves(&mut pos, &mut list);
}

fn print_board(board: BoardState)
{
	for rank in (0..=7).rev() {
        print!("{} ", char::from_digit(rank + 1, 10).unwrap());
        for file in 0..=7 {
            let sq = mailbox[fr2sq!(file, rank) as usize];
            let piece_string = piece_string(&board.pieces[sq as usize]);
            print!("{} ", piece_string)
        }
		println!(""); 
	}
	print!("  a b c d e f g h\n\n");
}

fn print_my_board(pos: Board)
{
	for rank in (0..=7).rev() {
        print!("{} ", char::from_digit(rank + 1, 10).unwrap());
        for file in 0..=7 {
            let sq = fr2sq!(file, rank);
            //println!("{}", sq);
            //println!("{}", pos.pieces[sq as usize]);
            let piece_string = piece_char[pos.pieces[sq as usize] as usize];
            print!("{} ", piece_string)
        }
		println!(""); 
	}
	print!("  a b c d e f g h\n\n");
}