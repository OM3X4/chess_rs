use chess::board::{TranspositionTable, bishop_magic::init_bishop_magics};
use chess::board::rook_magic::init_rook_magics;
use chess::board::Move;
use std::io::{self, Write};

fn main() {
    init_bishop_magics();
    init_rook_magics();

    let mut board = chess::board::Board::new();

    let mut tt = TranspositionTable::new(20);

    let mut depth = 64;
    let mut time = std::time::Duration::from_secs(30000);

    let mut use_tt = true;
    let mut use_lmr = true;
    let mut use_null_move = true;
    let mut use_q = true;
    let mut use_move_ordering = true;

    loop {
        io::stdout().flush().unwrap();

        // Read Input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read prompt");

        let input = input.trim();

        if input == "uci" {
            // Initialization
            println!("id name QueenFish 2.0 [Egypt]");
            println!("id author Omar Emad (om3x4)");
            println!("option name UseTT type check default true");
            println!("option name UseLMR type check default true");
            println!("option name UseNullMove type check default true");
            println!("option name UseQuiesense type check default true");
            println!("option name UseMoveOrder type check default true");
            println!("uciok");
            io::stdout().flush().unwrap();
        } else if input == "isready" {
            println!("readyok");
            io::stdout().flush().unwrap();
        } else if input == "ucinewgame" {
            // println!("ok");
            board.reset_to_default();
        } else if input.starts_with("position") {
            let tokens: Vec<&str> = input.split_whitespace().collect();

            board.reset_to_default();

            let mut idx = 1;

            if tokens[idx] == "startpos" {
                idx += 1;
            } else if tokens[idx] == "fen" {
                let fen = tokens[idx + 1..idx + 7].join(" ");
                board.load_from_fen(&fen);
                idx += 7;
            }

            if idx < tokens.len() && tokens[idx] == "moves" {
                idx += 1;
                while idx < tokens.len() {
                    let mv = Move::from_uci(tokens[idx], &board);
                    board.make_move(mv);
                    idx += 1;
                }
            }
            // dbg!(board.hash);
            // dbg!(board.to_fen());
            io::stdout().flush().unwrap();
        } else if input.starts_with("setoption") {
            if input.contains("UseTT") {
                use_tt = input.contains("true");
            } else if input.contains("UseLMR") {
                use_lmr = input.contains("true");
            } else if input.contains("UseNullMove") {
                use_null_move = input.contains("true");
            } else if input.contains("UseQuiesense") {
                use_q = input.contains("true");
            } else if input.contains("UseMoveOrder") {
                use_move_ordering = input.contains("true");
            }
        } else if input.starts_with("go") {
            let args = input.split(' ').collect::<Vec<&str>>();

            let depth_parsed = args
                .iter()
                .position(|&x| x == "depth")
                .and_then(|i| args.get(i + 1));

            let time_parsed = args
                .iter()
                .position(|&x| x == "movetime")
                .and_then(|i| args.get(i + 1));

            if let Some(depth_str) = depth_parsed {
                depth = depth_str.parse::<i32>().unwrap();
            }

            if let Some(time_str) = time_parsed {
                time = std::time::Duration::from_millis(time_str.parse::<u64>().unwrap());
            }

            dbg!(board.to_fen());

            println!(
                "bestmove {}",
                board
                    .engine(
                        depth,
                        true,
                        use_tt,
                        use_null_move,
                        use_lmr,
                        use_q,
                        use_move_ordering,
                        time,
                        None
                    )
                    .to_uci()
            );
            io::stdout().flush().unwrap();
        } else if input == "quit" {
            break;
        }
    }
}
