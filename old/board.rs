pub enum Colour {
    White,
    Black
}

impl Colour {
    pub fn index(self) -> i32 {
        match self {
            Colour::White => 0,
            Colour::Black => 1,
        }
    }

    pub fn as_string(self) -> String {
        match self {
            Colour::White => String::from("White"),
            Colour::Black => String::from("Black"),
        }
    }
}

pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    pub fn index(self) -> usize {
        match self {
            PieceType::Pawn => 0,
            PieceType::Knight => 1,
            PieceType::Bishop => 2,
            PieceType::Rook => 3,
            PieceType::Queen => 4,
            PieceType::King => 5,
        }
    }

    pub fn string(self) -> &'static str {
        match self {
            PieceType::Pawn => "p",
            PieceType::Knight => "n",
            PieceType::Bishop => "b",
            PieceType::Rook => "r",
            PieceType::Queen => "q",
            PieceType::King => "k",
        }
    }    
}

pub enum Square {
    Empty,
    Piece(Colour, PieceType),
    Offboard,
}

impl Square {
    pub fn return_piece(self) {
        match self {
            Square::Empty => println!("Empty square"),
            Square::Piece(colour,piece) => println!("Piece: {} {}", colour.as_string(), piece.index()),
            Square::Offboard => println!("Square offboard")
        }
    }

    pub fn print(self) -> String  {
        match self {
            Square::Empty => String::from("."),
            Square::Piece(c,p) => {
                if &c.index() == 0 {
                    format!("{} {}", c.as_string(), p.string().to_uppercase())
                }
                else {
                    format!("{} {}", c.as_string(), p.string())
                }
                },
            Square::Offboard => String::from(".")
        }
    }
}
    use crate::board::Square::Piece;
    use crate::board::Square::*;
    use crate::board::PieceType::*;
    use crate::board::Colour::*;

pub fn main() {
    let e = Empty;
    let t = Piece(White, Bishop);
    t.return_piece();
    e.return_piece();
    let board: [Square; 3] = [Square::Empty, Square::Empty, Square::Empty];
}

pub fn init_board() {
    let init_pieces: [i32; 64] = [
        3, 1, 2, 4, 5, 2, 1, 3,
        0, 0, 0, 0, 0, 0, 0, 0,
        6, 6, 6, 6, 6, 6, 6, 6,
        6, 6, 6, 6, 6, 6, 6, 6,
        6, 6, 6, 6, 6, 6, 6, 6,
        6, 6, 6, 6, 6, 6, 6, 6,
        0, 0, 0, 0, 0, 0, 0, 0,
        3, 1, 2, 4, 5, 2, 1, 3
    ];

    use crate::board::Square::Piece;
    use crate::board::Square::*;
    use crate::board::PieceType::*;
    use crate::board::Colour::*;

    let mut board: [Square; 64] = [
        Piece(White, Rook), Piece(White, Knight), Piece(White, Bishop), Piece(White, Queen), Piece(White, King),  Piece(White, Bishop),  Piece(White, Knight),  Piece(White, Rook),
        Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,
        Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,
        Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,
        Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,
        Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,
        Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,
        Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty,  Empty
    ];

    for sq in board {
        sq.return_piece()
    }
    
}

// fn pieces (pc: &str) -> Piece {
//     use crate::board::PieceType::*;
//     use crate::board::Colour::*;
//     match pc {
//         "p" => Piece{pctype: Bishop, colour: White},
//         &_ => Square::Empty,
//     }
// }

pub fn mailbox120(index: usize) -> i32 {
    let board120: [i32; 120] = [
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1,  0,  1,  2,  3,  4,  5,  6,  7, -1,
        -1,  8,  9, 10, 11, 12, 13, 14, 15, -1,
        -1, 16, 17, 18, 19, 20, 21, 22, 23, -1,
        -1, 24, 25, 26, 27, 28, 29, 30, 31, -1,
        -1, 32, 33, 34, 35, 36, 37, 38, 39, -1,
        -1, 40, 41, 42, 43, 44, 45, 46, 47, -1,
        -1, 48, 49, 50, 51, 52, 53, 54, 55, -1,
        -1, 56, 57, 58, 59, 60, 61, 62, 63, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1
    ];

    board120[index]
}

pub fn mailbox64(index: usize) -> i32 {
    let board64: [i32; 64] = [
        21, 22, 23, 24, 25, 26, 27, 28,
        31, 32, 33, 34, 35, 36, 37, 38,
        41, 42, 43, 44, 45, 46, 47, 48,
        51, 52, 53, 54, 55, 56, 57, 58,
        61, 62, 63, 64, 65, 66, 67, 68,
        71, 72, 73, 74, 75, 76, 77, 78,
        81, 82, 83, 84, 85, 86, 87, 88,
        91, 92, 93, 94, 95, 96, 97, 98
    ];
    board64[index]
}

/* in_check() returns TRUE if side s is in check and FALSE
   otherwise. It just scans the board to find side s's king
   and calls attack() to see if it's being attacked. */

// unsafe fn in_check(s: i32) -> usize { 
//     for i in 0..64 {
//         if piece[i] == KING && color[i] == s {
//             return attack(i.try_into().unwrap(), s ^ 1);
//         }
//     }
//     return TRUE;  /* shouldn't get here */
// }

// unsafe fn attack(sq: i32, s: i32) -> usize {
//     let mut n: i32 = 0;
// 	for i in 0..64 {
//         if color[i] == s {
// 			if piece[i] == PAWN {
// 				if s == LIGHT {
// 					if COL(i) != 0 && i - 9 == sq.try_into().unwrap() {
//                         return TRUE;
//                     }
						
// 					if COL(i) != 7 && i - 7 == sq.try_into().unwrap() {
//                         return TRUE;
//                     }
						
// 				}
// 				else {
// 					if COL(i) != 0 && i + 7 == sq.try_into().unwrap() {
//                         return TRUE;
//                     }
						
// 					if COL(i) != 7 && i + 9 == sq.try_into().unwrap() {
//                         return TRUE;
//                     }	
// 				}
// 			}
// 			else {
//                 for j in 0..offsets[piece[i] as usize] {
//                     n = mailbox[mailbox64[n as usize] + (offset[piece[i] as usize][j as usize] as usize)];
//                     if n == -1 {
//                         break;
//                     }
                        
//                     if n == sq {
//                         return TRUE;
//                     }
                        
//                     if color[n as usize] != EMPTY {
//                         break;
//                     }
                        
//                     if slide[piece[i] as usize] == FALSE {
//                         break;
//                     }
//                 }
//             }
				
//         }
//     }
//     return FALSE;
// }

// /* gen_push() puts a move on the move stack, unless it's a
//    pawn promotion that needs to be handled by gen_promote().
//    It also assigns a score to the move for alpha-beta move
//    ordering. If the move is a capture, it uses MVV/LVA
//    (Most Valuable Victim/Least Valuable Attacker). Otherwise,
//    it uses the move's history heuristic value. Note that
//    1,000,000 is added to a capture move's score, so it
//    always gets ordered above a "normal" move. */

// unsafe fn gen_push(from: i32, to: i32, bits: i32)
// {
  
//     let mut g = empty_gent;
	
// 	if bits & 16 == 1 {
// 		if side == LIGHT {
// 			if to <= H8 {
// 				gen_promote(from, to, bits);
// 				return;
// 			}
// 		}
// 		else {
// 			if to >= A1 {
// 				gen_promote(from, to, bits);
// 				return;
// 			}
// 		}
// 	}

//     // check this line
// 	g = gen_dat[first_move[ply + 1] as usize];

//     println!("ply {0}", ply);
//     println!("from {}", from);
//     println!("from {0}", from_digit(from.try_into().unwrap(), 2).unwrap());
//     println!("to {0}", to);
//     println!("bits {0}", bits);
//     println!("g {:?}", g);

// 	g.m.b.from = from_digit(from.try_into().unwrap(), 10).unwrap();
// 	g.m.b.to = from_digit(to.try_into().unwrap(), 10).unwrap();
// 	g.m.b.promote = from_digit(0, 10).unwrap();
// 	g.m.b.bits = from_digit(bits.try_into().unwrap(), 10).unwrap();

// 	if color[to as usize] != EMPTY {
//         g.score = 1000000 + (piece[to as usize] * 10) - piece[from as usize];
//     }
// 	else {
//         g.score = history[[from as usize, to as usize]];
//     }
//     count_moves = count_moves + 1;
//     println!("Moves: {}", count_moves);
// }

// /* gen_promote() is just like gen_push(), only it puts 4 moves
//    on the move stack, one for each possible promotion piece */

// unsafe fn gen_promote(from: i32, to: i32, bits: i32) {
//     let mut g = empty_gent;
    
//     for i in KNIGHT..QUEEN {
//         g = gen_dat[first_move[ply + 1] as usize];
//         g.m.b.from = from_digit(from.try_into().unwrap(), 10).unwrap();
//         g.m.b.to = from_digit(to.try_into().unwrap(), 10).unwrap();
//         g.m.b.promote = from_digit(i.try_into().unwrap(), 10).unwrap();
//         g.m.b.bits = from_digit((bits | 32).try_into().unwrap(), 10).unwrap();
//         g.score = 1000000 + (i * 10);
//     }
// }