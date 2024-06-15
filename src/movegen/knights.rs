use crate::{BitBoard, BitMove, MoveList};

use super::{MoveGenerator, SQ};

impl MoveGenerator {
    pub fn w_knight_moves(mut bb: BitBoard, w_empty: BitBoard, list: &mut MoveList) {
        //println!("Initial bitboard {:?}", bb);
        while bb > BitBoard(0) {
            let from = BitBoard::next(&mut bb);
            //println!("From {}", from);
            let from_bb: BitBoard = BitBoard(1) << BitBoard(from);
            //println!("From bb {:?}", from_bb);
            let mut to_bb: BitBoard = from_bb.knight_attacks() & w_empty;
            //println!("To bb {:?}", to_bb);
            while to_bb > BitBoard(0) {
                let to = BitBoard::next(&mut to_bb);
                list.push(BitMove::new(0, SQ(from as u8), SQ(to as u8)));
                //println!("Knight move from {} to {}", from, to);
            }
        }
        //bb.knight_attacks() & w_empty 
    }
}