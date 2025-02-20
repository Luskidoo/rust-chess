use std::{collections::HashMap, hash::Hash, sync::{Arc, Mutex}, time::Instant};

use crate::{defs::{Pieces, SQUARE_NAME}, movegen::MoveGenerator, Board, MoveList};

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
        // Clear divide_count for each depth
        divide_count.clear();

        // Current time
        let now = Instant::now();
        let mut leaf_nodes = 0;

        leaf_nodes += perft(&mut board, d, d, &mg, &mut divide_count);

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
        if d == depth {
            println!("Divided perft for depth {d}:");
            // 1. Convert HashMap to Vec of (String, u64)
            let mut sorted_moves: Vec<_> = divide_count.iter().collect();
            // 2. Sort the Vec by move string (key)
            sorted_moves.sort_by_key(|pair| pair.0);
            // 3. Iterate over the sorted Vec and print
            for (m, count) in sorted_moves {
                println!("{m}: {count}");
            }
        }
    }

    // Final calculation of the entire time taken, and average speed of leaves/second.
    let final_lnps = ((total_nodes * 1000) as f64 / total_time as f64).floor();
    println!("Total time spent: {total_time} ms");
    println!("Execution speed: {final_lnps} leaves/second");
}


pub fn perft(
    board: &mut Board,
    depth: u8,
    max_depth: u8,
    mg: &MoveGenerator,
    divide_count: &mut HashMap<String, u64>
) -> u64 {
    let mut leaf_nodes: u64 = 0;
    let mut move_list: MoveList = MoveList::new();

    // Count each visited leaf node.
    if depth == 0 {
        return 1;
    }

    mg.generate_all_moves(board, &mut move_list);
    // Run perft for each of the moves.
    for i in 0..move_list.len() {
        // Get the move to be executed and counted.
        let m = move_list.get_move(i);
        let move_string = String::from(format!("{}{}", SQUARE_NAME[m.from().0], SQUARE_NAME[m.to().0]));
        // If the move is legal...
        let legal = board.make(m, mg);
        //board.print_board();
        if legal {
            // Then count the number of leaf nodes it generates...
            let nodes= perft(board, depth - 1, max_depth, mg, divide_count);
            //println!("Move: {}, Nodes: {}", m.as_string(), nodes);
            leaf_nodes += nodes;
            if depth == max_depth {
                divide_count.entry(move_string).and_modify(|counter| *counter += nodes).or_insert(nodes);
            }
            // Then unmake the move so the next one can be counted.
            board.unmake();
            //board.print_board();
        }

        else {
            continue;
        }
    }
    // Return the number of leaf nodes for the given position and depth.
    leaf_nodes
}