use crate::BitBoard;

struct Board {
    pawns: [BitBoard; 3],
    bishops: [BitBoard; 3],
    knights: [BitBoard; 3],
    rooks: [BitBoard; 3],
    queens: [BitBoard; 3],
    king: [BitBoard; 3],
}

impl Board {
    pub const fn default() -> Board { 
        Board {
            pawns: [BitBoard::empty; 3],
            bishops: [BitBoard::empty; 3],
            knights: [BitBoard::empty; 3],
            rooks: [BitBoard::empty; 3],
            queens: [BitBoard::empty; 3],
            king: [BitBoard::empty; 3],
        }
    }

}