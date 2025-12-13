fn main() {
    use chess::chess::Board;
    let mut board = Board::new();

    let start = std::time::Instant::now();
    board.load_from_fen("r2q1rk1/pp1b1ppp/2np1n2/2p1p3/2P1P3/2NP1N2/PP1B1PPP/R2Q1RK1 w");
    let mut count = 0;
    for _ in 0..1_000_000 {
        let _ = board.generate_moves();
        count += 1;
    }
    let end = std::time::Instant::now();
    let duration = end.duration_since(start);
    println!(
        "Time took: {:?} seconds for {} moves",
        duration.as_secs_f64(),
        count
    );
}
