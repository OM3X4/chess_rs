use std::time::Duration;

fn main() {
    use chess::board::*;
    let depth = 12;
    let is_tt = false;
    let is_lmr = false;
    let is_null_move_pruning = false;
    let is_quiesense = false;
    use bishop_magic::init_bishop_magics;
    use rook_magic::init_rook_magics;

    init_rook_magics();
    init_bishop_magics();

    // Starting Position
    {
        // let mut board = Board::new();
        // println!("################################################");
        // println!("################################################");
        // println!("Starting Position:");
        // println!("################################################");
        // println!("################################################\n\n");

        // println!("-------------------------------------------------");
        // println!("Single Threaded Engine:");
        // let start = std::time::Instant::now();
        // let best_move = board.engine(depth , 1 , is_tt , is_null_move_pruning);
        // println!("Time taken: {:?}", start.elapsed());
        // println!("Best Move: {}", best_move.to_uci());
        // println!("-------------------------------------------------\n");

        // println!("-------------------------------------------------");
        // println!("8 Threads Engine:");
        // let start = std::time::Instant::now();
        // let best_move = board.engine(depth , 8);
        // println!("Time taken: {:?}", start.elapsed());
        // println!("Best Move: {}", best_move.to_uci());
        // println!("-------------------------------------------------\n");

        // println!("-------------------------------------------------");
        // println!("16 Threads Engine:");
        // let start = std::time::Instant::now();
        // let best_move = board.engine(depth , 16);
        // println!("Time taken: {:?}", start.elapsed());
        // println!("Best Move: {}", best_move.to_uci());
        // println!("-------------------------------------------------\n");
    }//


    // Complex Position
    {
        let mut board = Board::new();
        board.load_from_fen("2b2r2/rp1nb1p1/1q1p1n1k/p1p1Np2/1PQPp3/P1N1P3/2P2PPP/2RK1B1R w");
        println!("################################################");
        println!("################################################");
        println!("Complex Position:");
        println!("################################################");
        println!("################################################\n\n");

        println!("-------------------------------------------------");
        println!("Single Threaded Engine:");
        let start = std::time::Instant::now();
        let best_move = board.engine(depth , 1 , is_tt , is_null_move_pruning , is_lmr , is_quiesense , Duration::from_secs(15));
        println!("Time taken: {:?}", start.elapsed());
        println!("Best Move: {}", best_move.to_uci());
        println!("-------------------------------------------------\n");

        // println!("-------------------------------------------------");
        // println!("8 Threads Engine:");
        // let start = std::time::Instant::now();
        // let best_move = board.engine(depth , 8);
        // println!("Time taken: {:?}", start.elapsed());
        // println!("Best Move: {}", best_move.to_uci());
        // println!("-------------------------------------------------\n");

        // println!("-------------------------------------------------");
        // println!("16 Threads Engine:");
        // let start = std::time::Instant::now();
        // let best_move = board.engine(depth , 16);
        // println!("Time taken: {:?}", start.elapsed());
        // println!("Best Move: {}", best_move.to_uci());
        // println!("-------------------------------------------------\n");
    }//


}
