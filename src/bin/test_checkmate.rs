use chess::board::*;

fn main() {
    let mut fen = String::new();
    std::io::stdin().read_line(&mut fen).unwrap();

    let mut board = Board::new();

    board.load_from_fen(fen.trim());

    // println!("{}", board.to_fen());
    let game_state = board.get_game_state();
    match game_state {
        GameState::CheckMate => println!("CHECKMATE"),
        GameState::StaleMate => println!("STALEMATE"),
        GameState::InProgress => println!("IN_PROGRESS")
    }
}