use std::time::Duration;

fn main() {
    use bishop_magic::init_bishop_magics;
    use chess::board::*;
    use rook_magic::init_rook_magics;

    init_rook_magics();
    init_bishop_magics();

    let mut fen = String::new();
    std::io::stdin().read_line(&mut fen).unwrap();

    let mut board = Board::new();

    board.load_from_fen(fen.trim());
    // board.load_from_fen("8/7n/3r1B1P/4Nk2/b7/5QB1/pKN1q1Pb/8 b");

    // println!("{}", board.to_fen());

    // dbg!(board.turn);
    let start = std::time::Instant::now();
    let best_move = board.engine(64 , 1 , false , true , Duration::from_secs(3));
    dbg!(start.elapsed());
    // let best_move_1 = board.engine(6 , 1 , true , false);
    // dbg!(best_move_1.to_uci());
    // assert_eq!(best_move , best_move_1);



    dbg!(best_move.to_uci());
    println!("{:?} {:?}", best_move.from() , best_move.to());
}
