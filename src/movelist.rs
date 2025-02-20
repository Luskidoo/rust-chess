use crate::defs::MAX_POSITION_MOVES;
use crate::movegen::bit_move::Move;
use std::mem;

#[derive(Copy, Clone)]
pub struct MoveList {
    list: [Move; MAX_POSITION_MOVES as usize],
    pub count: u8,
}

impl MoveList {

    pub fn new() -> Self {
        Self {
            list: unsafe {
                let block = mem::MaybeUninit::uninit();
                block.assume_init()
            },
            count: 0,
        }
    }
    // Used to store a move in the move list.
    pub fn push(&mut self, m: Move) {
        self.list[self.count as usize] = m;
        self.count += 1;
    }

    // Returns the number of moves in the move list.
    pub fn len(&self) -> u8 {
        self.count
    }

    // Return the move at the given index. If out of bounds, the program crashes.
    pub fn get_move(&self, index: u8) -> Move {
        self.list[index as usize]
    }

    pub fn move_list_ok(&self) -> bool {
        let mut result: bool = true;
        if self.count > MAX_POSITION_MOVES.try_into().unwrap() {
            result = false;
        }

        for i in 0..self.count {
            let to = self.list[i as usize].to();
            let from = self.list[i as usize].from();
            if to.0 > 63 || from.0 > 63 {
                result = false;
            }
        };
        result
    }
}

impl Default for MoveList {
    #[inline]
    fn default() -> Self {
        MoveList {
            list: [Move::null(); MAX_POSITION_MOVES],
            count: 0,
        }
    }
}