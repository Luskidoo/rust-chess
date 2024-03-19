mod bitboard;
use crate::bitboard::*;
use fen::*;

const index64: [u64; 64] = [
    0, 47,  1, 56, 48, 27,  2, 60,
   57, 49, 41, 37, 28, 16,  3, 61,
   54, 58, 35, 52, 50, 42, 21, 44,
   38, 32, 29, 23, 17, 11,  4, 62,
   46, 55, 26, 59, 40, 36, 15, 53,
   34, 51, 20, 43, 31, 22, 10, 45,
   25, 39, 14, 33, 19, 30,  9, 24,
   13, 18,  8, 12,  7,  6,  5, 63
];


fn bitScanForwardWithReset(mut bb: u64) -> u64 { // also called dropForward
    let idx = bitScanForward(bb);
    bb &= bb - 1; // reset bit outside
    return idx;
}

fn bitScanForward(bb: u64) -> u64 {
    let debruijn64: u64 = 0x03f79d71b4cb0a89u64;
    return index64[(((bb ^ (bb.wrapping_sub(1))).wrapping_mul(debruijn64).wrapping_shr(58))) as usize].try_into().unwrap();
}

fn from_fen(fen: String, bb: BitBoard) -> BitBoard {
    let board = fen::BoardState::from_fen(&fen).unwrap();
    for sq in 0..64 {
        let piece = board.pieces[sq].as_ref().unwrap();
        match piece.kind {
            fen::PieceKind::Pawn => println!("PAWN"),
            _ => (),
        }
    }
    //if piece.kind == fen::PieceKind::Pawn {
    bb.set_bit(bitboard::BitBoard(3));
    //}
    bb
}

fn main() {
    let pawns: [BitBoard; 3] = [BitBoard::empty; 3];
    let mut bb : BitBoard = BitBoard::empty;
    bb = bitboard::BitBoard(4);
    let bb2 = bitboard::BitBoard(3);
    let fen = String::from("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
    from_fen(fen, bb);
    println!("{:?}", bb.set_bit(BitBoard(3)));
}