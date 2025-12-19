fn main() {
    use chess::board::Board;

    use chess::board::rook_magic::init_rook_magics;
    use chess::board::bishop_magic::init_bishop_magics;

    init_rook_magics();
    init_bishop_magics();


    let mut board = Board::new();
    // board.load_from_fen("2kr3r/1pp3pp/p7/2b1np2/P3p1nq/4P3/1P1PBP1P/RNBQK2R w");
    let start = std::time::Instant::now();
    dbg!(board.perft(0,6));
    dbg!(start.elapsed());
}
