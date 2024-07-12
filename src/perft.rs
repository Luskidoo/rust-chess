use std::{sync::{Arc, Mutex}, time::Instant};

use crate::{movegen::MoveGenerator, Board, MoveList};

pub fn run(
    board: Board,
    depth: i8,
    mg: MoveGenerator,
    //tt: Arc<Mutex<TT<PerftData>>>,
    //tt_enabled: bool,
) {
    let mut total_time: u128 = 0;
    let mut total_nodes: u64 = 0;
    let mut hash_full = String::from("");

    // Create a mutex guard for the board, so it can be safely cloned.
    // Panic if the guard can't be created, because something is wrong with
    // the main engine thread.
    let mtx_board = board;//.lock().expect("Board lock failed");

    // Clone the locked board for local use.
    let mut local_board = mtx_board.clone();

    // The function now has its own local board. Drop the guard. It is not
    // necessary to keep the lock until perft runs out.
    std::mem::drop(mtx_board);

    println!("Benchmarking perft 1-{depth}:");

    //print::position(&local_board, None);

    // Perform all perfts for depths 1 up to and including "depth"
    for d in 1..=depth {
        // Current time
        let now = Instant::now();
        let mut leaf_nodes = 0;

        leaf_nodes += perft(&mut local_board, d, &mg);

        // Measure time and speed
        let elapsed = now.elapsed().as_millis();
        let leaves_per_second = ((leaf_nodes * 1000) as f64 / elapsed as f64).floor();

        // Add tot totals for final calculation at the very end.
        total_time += elapsed;
        total_nodes += leaf_nodes;

        // Request TT usage. (This is provided permille as per UCI
        // spec, so divide by 10 to get the usage in percents.)
        // if tt_enabled {
        //     hash_full = format!(
        //         ", hash full: {}%",
        //         tt.lock().expect(ErrFatal::LOCK).hash_full() as f64 / 10f64
        //     );
        // }

        // Print the results.
        println!(
            "Perft {d}: {leaf_nodes} ({elapsed} ms, {leaves_per_second} leaves/sec{hash_full})"
        );
    }

    // Final calculation of the entire time taken, and average speed of leaves/second.
    let final_lnps = ((total_nodes * 1000) as f64 / total_time as f64).floor();
    println!("Total time spent: {total_time} ms");
    println!("Execution speed: {final_lnps} leaves/second");
}

// This is the actual Perft function. It is public, because it is used by
// the "testsuite" module.
pub fn perft(
    board: &mut Board,
    depth: i8,
    mg: &MoveGenerator,
    //tt: &Mutex<TT<PerftData>>,
    //tt_enabled: bool,
) -> u64 {
    let mut leaf_nodes: u64 = 0;
    let mut move_list: MoveList = MoveList::default();

    // Count each visited leaf node.
    if depth == 0 {
        return 1;
    }

    // See if the current position is in the TT, and if so, get the
    // number of leaf nodes that were previously calculated for it.
    // let mut leaf_nodes_tt: Option<u64> = None;
    // if tt_enabled {
    //     if let Some(data) = tt
    //         .lock()
    //         .expect(ErrFatal::LOCK)
    //         .probe(board.game_state.zobrist_key)
    //     {
    //         leaf_nodes_tt = data.get(depth);
    //     };
    // }

    // If we found a leaf node count, return it immediately.
    // if let Some(leaf_nodes) = leaf_nodes_tt {
    //     return leaf_nodes;
    // }

    mg.generate_all_moves(board, &mut move_list);
    println!("Depth: {}, Moves generated: {}", depth, move_list.len());

    // Run perft for each of the moves.
    for i in 0..move_list.len() {
        // Get the move to be executed and counted.
        let m = move_list.get_move(i);
        println!("Trying move: {}", m.as_string());
        // If the move is legal...
        if board.make(m, mg) {
            // Then count the number of leaf nodes it generates...
            let nodes = perft(board, depth - 1, mg);
            println!("Move: {}, Nodes: {}", m.as_string(), nodes);
            leaf_nodes += nodes;
            // Then unmake the move so the next one can be counted.
            board.unmake();
        } else {
            println!("Illegal move: {}", m.as_string());
        }
    }

    // We have calculated the number of leaf nodes for this position.
    // Store this in the TT for later use.
    // if tt_enabled {
    //     tt.lock().expect(ErrFatal::LOCK).insert(
    //         board.game_state.zobrist_key,
    //         PerftData::create(depth, leaf_nodes),
    //     )
    // }

    // Return the number of leaf nodes for the given position and depth.
    leaf_nodes
}