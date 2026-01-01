use rand::seq::IteratorRandom;
use rand::{Rng, rng};
use shakmaty::fen::Fen;
use shakmaty::{Chess, Position};

pub fn random_fen(min_moves: usize, max_moves: usize) -> Fen {
    let mut rng = rng();
    let mut board = Chess::default();

    let moves_to_play = rng.random_range(min_moves..=max_moves);
    for _ in 0..moves_to_play {
        if board.is_game_over() {
            break;
        }

        let mv = board.legal_moves().into_iter().choose(&mut rng).unwrap();

        board = board.play(mv).unwrap();
    }

    Fen::from_position(&board, shakmaty::EnPassantMode::Legal)
} //

#[cfg(test)]
mod test {
    use std::{collections::HashSet, time::Duration};

    use crate::board::Board as MyEngine;
    use crate::board::bishop_magic::init_bishop_magics;
    use crate::board::rook_magic::init_rook_magics;
    use rand::Rng;
    use shakmaty::{Chess, Position};
    use stockfish::Stockfish;

    #[test]
    fn move_generation() {
        init_rook_magics();
        init_bishop_magics();

        let mut counter = 0;

        loop {
            println!("Counter: {}", counter);
            let fen = super::random_fen(0, 10);
            let fen_string = fen.to_string();
            // .split_whitespace()
            // .take(2)
            // .fold(String::new(), |mut acc, w| {
            //     if !acc.is_empty() {
            //         acc.push(' ');
            //     }
            //     acc.push_str(w);
            //     acc
            // });

            let shakmaty_board: Chess =
                fen.into_position(shakmaty::CastlingMode::Standard).unwrap();

            let mut board = MyEngine::new();
            board.load_from_fen(&fen_string);

            let mut missed = vec![];

            let moves = shakmaty_board.legal_moves();
            let mut my_moves: HashSet<(u8, u8)> = board
                .generate_moves()
                .iter()
                .map(|mv| (mv.from(), mv.to()))
                .collect();

            for mv in moves {
                match mv {
                    shakmaty::Move::Normal {
                        role,
                        from,
                        capture,
                        to,
                        promotion,
                    } => {
                        // println!("{} {}" , mv.from().unwrap() as usize, mv.to() as usize);
                        let mv = (mv.from().unwrap() as u8, mv.to() as u8);
                        if let Some(promotion) = promotion {
                            if promotion != shakmaty::Role::Queen {
                                continue;
                            }
                        }
                        if my_moves.contains(&mv) {
                            my_moves.remove(&mv);
                            continue;
                        } else {
                            missed.push(mv);
                        }
                    },
                    shakmaty::Move::EnPassant { from, to } => {
                        let mv = (from as u8, to as u8);
                        if my_moves.remove(&mv) {
                            continue;
                        } else {
                            missed.push(mv);
                        }
                    }
                    _ => continue,
                }
            }

            if missed.len() > 0 || my_moves.len() > 0 {
                println!("❌ {} {}", counter, fen_string);
                println!("Missed : {:?}", missed);
                println!("Extra : {:?}", my_moves);
                println!(
                    "Moves : {:#?}",
                    board
                        .generate_moves()
                        .iter()
                        .map(|mv| (mv.from(), mv.to()))
                        .collect::<Vec<(u8, u8)>>()
                );
                break;
            }
            counter += 1;
        }
    } //

    #[test]
    fn best_move_test() {
        init_rook_magics();
        init_bishop_magics();

        let mut counter = 0;
        let mut missed = 0;

        while counter < 100 {
            let mut sf =
                Stockfish::new("C:/Program Files/stockfish/stockfish-windows-x86-64-avx2.exe")
                    .unwrap();

            let fen = super::random_fen(4, 20);
            let fen_string =
                fen.to_string()
                    .split_whitespace()
                    .take(2)
                    .fold(String::new(), |mut acc, w| {
                        if !acc.is_empty() {
                            acc.push(' ');
                        }
                        acc.push_str(w);
                        acc
                    });

            let mut board = MyEngine::new();
            board.load_from_fen(&fen_string);

            sf.set_fen_position(&fen.to_string()).unwrap();

            sf.set_depth(22);
            let output = sf.go().unwrap();
            let best_move_sf = output.best_move();
            sf.play_move(&best_move_sf).unwrap();
            let eval = sf.go().unwrap().eval().value();

            let start = std::time::Instant::now();
            let best_move_by_engine = board
                .engine(16, 1, false, true,false , false,  Duration::from_secs(6))
                .to_uci();
            dbg!(start.elapsed());

            sf.set_fen_position(&fen.to_string()).unwrap();
            sf.play_move(&best_move_by_engine).unwrap();
            let engine_eval = sf.go().unwrap().eval().value();

            dbg!(&fen_string);
            dbg!(best_move_sf);
            dbg!(&best_move_by_engine);

            counter += 1;
            if (eval - engine_eval).abs() > 100 {
                missed += 1;
                println!("❌ {}", &fen_string);
                println!("Expected : {}", best_move_sf);
                println!("Got : {}", best_move_by_engine);
                println!("Engine eval : {}", engine_eval);
                println!("Stockfish eval : {}", eval);
                println!("\n\n");
            } else {
                println!("Expected : {}", best_move_sf);
                println!("Got : {}", best_move_by_engine);
                println!("Engine eval : {}", engine_eval);
                println!("Stockfish eval : {}", eval);
                println!("\n\n");
            }
        }

        println!("Report: {} games tested, {} games missed", counter, missed);
    } //

    #[test]
    fn evaluating_sanity_check() {
        let mut board = MyEngine::new();

        // Starting Position eval must be zero
        board.load_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w");
        assert_eq!(board.eval, 0);

        // White up a pawn (white to move)
        board.load_from_fen("rnbqkbnr/pppp1ppp/8/8/8/8/PPPPPPPP/RNBQKBNR w");
        assert_eq!(board.eval, 100);

        // White up a pawn (black to move)
        board.load_from_fen("rnbqkbnr/pppp1ppp/8/8/8/8/PPPPPPPP/RNBQKBNR b");
        assert_eq!(board.eval, -100);
    } //

    #[test]
    fn make_unmake() {
        init_rook_magics();
        init_bishop_magics();

        let mut counter = 0;

        while counter < 100000 {
            println!("Counter: {}", counter);
            let mut board = MyEngine::new();
            let fen = super::random_fen(4, 20);
            board.load_from_fen(&fen.to_string());

            let moves = board.generate_moves();

            if moves.is_empty() || moves.len() == 1 {
                counter -= 1;
                continue;
            }

            let rng = rand::rng().random_range(0..(moves.len() - 1));

            let old_fen = board.to_fen();

            let old_board = board.clone();
            let unmake = board.make_move(moves[rng]);

            let mid_fen = board.to_fen();

            // board = old_board.clone();
            board.unmake_move(unmake);

            let new_fen = board.to_fen();

            if new_fen != old_fen {
                dbg!(fen.to_string());
                dbg!(&old_fen);
                dbg!(&mid_fen);
                dbg!(&new_fen);
                break;
            }
            counter += 1;
        }
    }
}
