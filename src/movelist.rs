use crate::defs::MAX_LEGAL_MOVES;
use crate::movegen::bit_move::Move;

#[derive(Copy, Clone)]
pub struct MoveList {
    list: [Move; MAX_LEGAL_MOVES as usize],
    pub count: u8,
}

impl MoveList {
    // Used to store a move in the move list.
    pub fn push(&mut self, m: Move) {
        self.list[self.count as usize] = m;
        self.count += 1;
    }

    // Returns the number of moves in the move list.
    pub fn len(&self) -> u8 {
        self.count
    }
}

impl Default for MoveList {
    #[inline]
    fn default() -> Self {
        MoveList {
            list: [Move::null(); MAX_LEGAL_MOVES],
            count: 0,
        }
    }
}