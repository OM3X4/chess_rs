fn main() {

    use chess::board::*;

    let mut fen = String::new();
    std::io::stdin().read_line(&mut fen).unwrap();

    let mut board = Board::new();

    board.load_from_fen(fen.trim());

    // println!("{}", board.to_fen());

    let moves = board.generate_moves();

    for m in moves {
        println!("{} {}", m.from(), m.to());
    }
}