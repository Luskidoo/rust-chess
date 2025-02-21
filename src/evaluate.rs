use crate::board::Board;
use crate::defs::{Piece, Pieces, Sides};
pub fn evaluate_position(board: &mut Board) -> i32 {
    count_material(board)
}

fn count_material(board: &mut Board) -> i32{

    let mut material_score: i32 = 0;
    material_score += 100 * (board.pieces[Sides::WHITE][Pieces::PAWN].pop_count() as i32 - board.pieces[Sides::BLACK][Pieces::PAWN].pop_count() as i32);
    material_score += 300 * (board.pieces[Sides::WHITE][Pieces::KNIGHT].pop_count() as i32 - board.pieces[Sides::BLACK][Pieces::KNIGHT].pop_count() as i32);
    material_score += 300 * (board.pieces[Sides::WHITE][Pieces::BISHOP].pop_count() as i32 - board.pieces[Sides::BLACK][Pieces::BISHOP].pop_count() as i32);
    material_score += 500 * (board.pieces[Sides::WHITE][Pieces::ROOK].pop_count() as i32 - board.pieces[Sides::BLACK][Pieces::ROOK].pop_count() as i32);
    material_score += 900 * (board.pieces[Sides::WHITE][Pieces::QUEEN].pop_count() as i32 - board.pieces[Sides::BLACK][Pieces::QUEEN].pop_count() as i32);
    material_score += 20000 * (board.pieces[Sides::WHITE][Pieces::KING].pop_count() as i32 - board.pieces[Sides::BLACK][Pieces::KING].pop_count() as i32);
    match board.game_state.side_to_move {
        Sides::WHITE => material_score,
        Sides::BLACK => -material_score,
        _ => 0
    }
}

// fn mobility(board: &mut Board) -> i32 {
//     let mut mobility_score: i32 = 0;
//     move_gen.generate_all_moves(board, &mut move_list);
//     mobility_score += move_list.len() as i32;
//     match board.game_state.side_to_move {
//         Sides::WHITE => mobility_score,
//         Sides::BLACK => -mobility_score,
//         _ => 0
//     }
// }