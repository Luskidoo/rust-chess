use std::{collections::HashMap, hash::Hash, sync::{Arc, Mutex}, time::Instant};

use crate::{defs::SQUARE_NAME, movegen::MoveGenerator, Board, MoveList};

pub fn run(
    mut board: Board,
    depth: u8,
    mg: MoveGenerator,
) {
    let mut total_time: u128 = 0;
    let mut total_nodes: u64 = 0;
    let mut divide_count: HashMap<String, u64> = HashMap::new();

    println!("Benchmarking perft 1-{depth}:");

    // Perform all perfts for depths 1 up to and including "depth"
    for d in 1..=depth {
        // Current time
        let now = Instant::now();
        let mut leaf_nodes = 0;

        leaf_nodes += perft(&mut board, d, depth, &mg, &mut divide_count);

        // Measure time and speed
        let elapsed = now.elapsed().as_millis();
        let leaves_per_second = ((leaf_nodes * 1000) as f64 / elapsed as f64).floor();

        // Add tot totals for final calculation at the very end.
        total_time += elapsed;
        total_nodes += leaf_nodes;

        // Print the results.
        println!(
            "Perft {d}: {leaf_nodes} ({elapsed} ms, {leaves_per_second} leaves/sec)"
        );
    }

    // Final calculation of the entire time taken, and average speed of leaves/second.
    let final_lnps = ((total_nodes * 1000) as f64 / total_time as f64).floor();
    println!("Total time spent: {total_time} ms");
    println!("Execution speed: {final_lnps} leaves/second");
    for (m, count) in divide_count {
        println!("{m}: {count}");
    }
}


pub fn perft(
    board: &mut Board,
    depth: u8,
    max_depth: u8,
    mg: &MoveGenerator,
    divide_count: &mut HashMap<String, u64>
) -> u64 {
    let mut leaf_nodes: u64 = 0;
    let mut move_list: MoveList = MoveList::default();

    // Count each visited leaf node.
    if depth == 0 {
        return 1;
    }

    let leaf = (depth == 2);

    mg.generate_all_moves(board, &mut move_list);

    // Run perft for each of the moves.
    for i in 0..move_list.len() {
        // Get the move to be executed and counted.
        let m = move_list.get_move(i);
        // If the move is legal...
        if board.make(m, mg) {
            // Then count the number of leaf nodes it generates...
            let nodes= perft(board, depth - 1, depth, mg, divide_count);
            //println!("Move: {}, Nodes: {}", m.as_string(), nodes);
            leaf_nodes += nodes;
            let move_string = String::from(format!("{}{}", SQUARE_NAME[m.from().0], SQUARE_NAME[m.to().0]));
            divide_count.entry(move_string).and_modify(|counter| *counter += nodes).or_insert(nodes);
            // Then unmake the move so the next one can be counted.
            board.unmake();
        }
    }
    // Return the number of leaf nodes for the given position and depth.
    leaf_nodes
}