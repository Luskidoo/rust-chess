pub enum Colour {
    White,
    Black
}

impl Colour {
    pub fn index(self) -> usize {
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

pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
    pub fn index(self) -> usize {
        match self {
            Piece::Pawn => 0,
            Piece::Knight => 1,
            Piece::Bishop => 2,
            Piece::Rook => 3,
            Piece::Queen => 4,
            Piece::King => 5,
        }
    }
    
}

pub enum Square {
    Empty,
    Full(Colour,Piece),
    Offboard,
}

impl Square {
    pub fn return_piece(self) {
        match self {
            Square::Empty => println!("Empty square"),
            Square::Full(colour,piece) => println!("Piece: {} {}", colour.as_string(), piece.index()),
            Square::Offboard => println!("Square offboard")
        }
    }
}

pub fn main() {
    let e = Square::Empty;
    let t = Square::Full(Colour::White, Piece::Bishop);
    t.return_piece();
    e.return_piece();
}



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

