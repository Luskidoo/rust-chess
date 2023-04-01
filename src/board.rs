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

