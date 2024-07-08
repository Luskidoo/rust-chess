use crate::Board;

use super::{Ranks, Side, Sides, Square};

impl Board {
    pub fn promotion_rank(side: Side) -> usize {
        match side {
            Sides::WHITE => Ranks::R8,
            Sides::BLACK => Ranks::R1,
            _ => panic!()
        }
    }

    // Compute if a given square is or isn't on the given rank.
    pub fn square_on_rank(square: &Square, rank: Square) -> bool {
        let start = (rank.0) * 8;
        let end = start + 7;
        (start..=end).contains(&square.0)
    }
}