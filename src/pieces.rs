use crate::board;

// enum RFSquare {A1 = 21, B1, C1, D1, E1, F1, G1, H1,
//             A2 = 31, B2, C2, D2, E2, F2, G2, H2,
//             A3 = 41, B3, C3, D3, E3, F3, G3, H3,
//             A4 = 51, B4, C4, D4, E4, F4, G4, H4,
//             A5 = 61, B5, C5, D5, E5, F5, G5, H5,
//             A6 = 71, B6, C6, D6, E6, F6, G6, H6,
//             A7 = 81, B7, C7, D7, E7, F7, G7, H7,
//             A8 = 91, B8, C8, D8, E8, F8, G8, H8}

pub fn main() {
    const MAX_ROOKS: usize = 10;
    const MAX_KNIGHTS: usize = 10;
    const MAX_BISHOPS: usize = 10;
    const MAX_QUEENS: usize = 9;
    const MAX_PAWNS: usize = 8;

    let w_rooks: [i32; MAX_ROOKS] = [21, 28, -1, -1, -1, -1, -1, -1, -1, -1];
    let w_knights: [i32; MAX_ROOKS] = [22, 27, -1, -1, -1, -1, -1, -1, -1, -1];
    let w_bishops: [i32; MAX_ROOKS] = [23, 26, -1, -1, -1, -1, -1, -1, -1, -1];
    let w_queens: [i32; MAX_ROOKS] = [24, -1, -1, -1, -1, -1, -1, -1, -1, -1];
    let w_pawns: [i32; MAX_ROOKS] = [31, 32, 33, 34, 35, 36, 37, 38, -1, -1];
    let w_king: [i32; MAX_ROOKS] = [25, -1, -1, -1, -1, -1, -1, -1, -1, -1];

    let b_rooks: [i32; MAX_ROOKS] = [91, 98, -1, -1, -1, -1, -1, -1, -1, -1];
    let b_knights: [i32; MAX_KNIGHTS] = [92, 97, -1, -1, -1, -1, -1, -1, -1, -1];
    let b_bishops: [i32; MAX_BISHOPS] = [93, 96, -1, -1, -1, -1, -1, -1, -1, -1];
    let b_queens: [i32; MAX_QUEENS] = [94, -1, -1, -1, -1, -1, -1, -1, -1];
    let b_pawns: [i32; MAX_PAWNS] = [81, 82, 83, 84, 85, 86, 87, 88];
    let b_king: i32 = 4;

    let num_w_rooks = 2;
    let num_w_knights = 2;
    let num_w_bishops = 2;
    let num_w_queens = 1;
    let num_w_pawns = 8;
    let num_w_king = 1;

    let rooks = 0;
    let knights = 1;
    let bishops = 2;

    let pieces: [[i32; 10]; 6] = [w_rooks, w_knights, w_bishops, w_queens, w_pawns, w_king];
    let num_pieces: [i32; 6] = [num_w_rooks, num_w_knights, num_w_bishops, num_w_queens, num_w_pawns, num_w_king];
    
    let piece_names: [&str; 6] = ["rook", "knight", "bishop", "queen", "pawn", "king"];

    for wrs in pieces[0] {
        if wrs != -1 {
            println!("{}", wrs)
        }
        
    }
}

    

//     let num_b_rooks = 2;
//     let num_b_knights = 2;
//     let num_b_bishops = 2;
//     let num_b_queens = 1;
//     let num_b_pawns = 8;
//     //let a1 = MyEnum::A1 as i32;
//     //let a5 = MyEnum::A5 as i32;
//     for x in 0..num_w_rooks {
//         println!("White rook is on square {}", board::mailbox120(w_rooks[x] as usize));
//     }  
        

// fn sq64 (sq: RFSquare) -> u32 {
//     match sq {
//         RFSquare::A1 => 21,
//         todo!()
//     }
// }

