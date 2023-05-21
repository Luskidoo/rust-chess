pub mod data;
mod board;

pub fn main() {
    let mut list = board::MoveList {
        moves: [board::Move {m: 0, score: 0}; data::MAX_POSITION_MOVES],
        count: 0
    };
    unsafe {
        board::init_hash();
        board::init_board();
        print_board();
        board::gen(&mut list);
    }
}

unsafe fn print_board()
{
	let i: usize;
	
	print!("\n8");
	for i in 0..64 {
		match board::color[i] {
			data::EMPTY => print!(" ."),
			data::LIGHT => print!(" {0}", data::piece_char[board::piece[i] as usize]),
			data::DARK => print!(" {0}", data::piece_char[board::piece[i] as usize].to_lowercase()),
            _ => print!("Undefined colour on board")
        }
		if (i + 1) % 8 == 0 && i != 63 {
            print!("\n{0}", 7 - data::ROW(i));
        }
			
	}
	print!("\n  a b c d e f g h\n\n");
}