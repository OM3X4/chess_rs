fn main() {
    use chess::board::Board;

    use chess::board::bishop_magic::init_bishop_magics;
    use chess::board::rook_magic::init_rook_magics;

    init_rook_magics();
    init_bishop_magics();

    let mut board = Board::new();
    // board.load_from_fen("1qr1k2r/1p2bp2/pBn1p3/P2pPbpp/5P2/2P1QBPP/1P1N3R/R4K2 b k -");
    let start = std::time::Instant::now();
    dbg!(board.perft(0,5));
    dbg!(start.elapsed());

    // let args: Vec<String> = std::env::args().collect();

    // if args[1] == "perft" {
    //     let fen = &args[2];
    //     let depth: u32 = args[3].parse().unwrap();

    //     let mut board = Board::new();
    //     board.load_from_fen(fen);

    //     let nodes = board.perft(0 , depth as i32);

    //     println!("{}", nodes);
    // }
}
