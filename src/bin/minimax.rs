fn main() {
    use chess::board::*;

    let start = std::time::Instant::now();

    let mut board = Board::new();
    board.engine();
    let end = std::time::Instant::now();
    let duration = end.duration_since(start);
    println!("Time took: {:?} seconds", duration.as_secs_f64(),);
}
