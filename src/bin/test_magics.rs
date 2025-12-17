use chess::board::Board;

fn main() {
    use chess::board::rook_magic::init_rook_magics;

    init_rook_magics();

    let mut board = Board::new();
    board.load_from_fen("1rbk1bnr/pp3ppp/1Pp1p3/3p1P2/5N1q/2NQ2P1/1PP1P2P/R1B1KB1R w");


    let start = std::time::Instant::now();

    for _ in 0..10_000_000 {
        // board.generate_rook_moves_magics();
        // let mut moves = Vec::new();
        // board.generate_pesudo_moves(&mut moves);
        // board.generate_moves();
        // board.generate_rook_moves(&mut moves);
    }
    let end = std::time::Instant::now();
    let duration = end.duration_since(start);
    println!("Time took: {:?} seconds", duration.as_secs_f64(),);

    // dbg!(moves);
}
