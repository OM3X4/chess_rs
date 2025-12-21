fn main() {
    use bishop_magic::init_bishop_magics;
    use chess::board::*;
    use rook_magic::init_rook_magics;

    init_bishop_magics();
    init_rook_magics();

    let start = std::time::Instant::now();

    let mut board = Board::new();
    board.load_from_fen("2b2r2/rp1nb1p1/1q1p1n1k/p1p1Np2/1PQPp3/P1N1P3/2P2PPP/2RK1B1R w");
    // let best_move = board.engine_multithreaded();
    let best_move = board.engine(8 , 1 , false , false);
    // for _ in 0..20_000_000 {
    //     // let mut moves: Vec<Move> = Vec::with_capacity(256);
    //     let mut moves: SmallVec<[Move; 256]> = SmallVec::new();
    //     board.generate_pesudo_moves(&mut moves);
    // }
    let end = std::time::Instant::now();
    let duration = end.duration_since(start);
    println!("Time took: {:?} seconds", duration.as_secs_f64(),);
    println!("{:?}", best_move.to_uci());
}
