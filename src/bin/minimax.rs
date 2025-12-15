fn main() {
    use chess::board::*;

    let start = std::time::Instant::now();

    let mut board = Board::new();
    // let mut moves_map: HashMap<u64, (i32 , i32)> = HashMap::new();
    let mut tt = TranspositionTable::new(20);
    board.alpha_beta(0 , i32::MIN + 1, i32::MAX - 1, &mut tt);
    let end = std::time::Instant::now();
    let duration = end.duration_since(start);
    println!(
        "Time took: {:?} seconds",
        duration.as_secs_f64(),
    );
}