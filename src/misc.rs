use crate::defs::{Square, SQUARE_NAME};

pub fn algebraic_square_to_number(algebraic_square: &str) -> Option<Square> {
    SQUARE_NAME
        .iter()
        .position(|&element| element == algebraic_square)
}