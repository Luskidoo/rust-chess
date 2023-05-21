use crate::data::*;
use crate::board;

pub unsafe fn sq_on_board(sq: i32) -> bool {
    if board::filesbrd[board::sq64tosq120[sq as usize] as usize] == OFFBOARD {
        return false
    }
    else {
        return true
    }
}