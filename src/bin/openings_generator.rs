use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let file = File::open("opening_6_10_2_moves.txt").unwrap();
    let reader = BufReader::new(file);

    println!("pub struct BookEntry {{ pub hash: u64, pub moves: &'static [u16], }}");
    println!("pub static OPENING_BOOK: &[BookEntry] = &[");

    let mut entries = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let (fen, moves) = parse_line(&line);

        // let hash = compute_zobrist(&fen);
        // let encoded = encode_moves(&moves);

        let (hash , encoded) = compute(&fen , &moves);

        entries.push((hash, encoded));
    }

    entries.sort_by_key(|e| e.0);

    for (hash, moves) in entries {
        println!(
            "    BookEntry {{ hash: 0x{:016x}, moves: &{:?} }},",
            hash, moves
        );
    }

    println!("];");
}


fn parse_line(line: &str) -> (String , Vec<String>) {
    let mut parts = line.split("||").collect::<Vec<_>>();
    (parts.remove(0).trim().to_string(), parts.remove(0).trim().split_ascii_whitespace().map(|s| s.to_string()).collect())
}

fn compute(fen: &str , moves: &Vec<String>) -> (u64 , Vec<u32>) {
    let mut board = chess::board::Board::new();
    board.load_from_fen(fen);
    let mut valid_moves = Vec::new();
    board.generate_moves().iter().for_each(|mv| {
        if moves.contains(&mv.to_uci()) {
            valid_moves.push(mv.move_encoded());
        }
    });
    (board.compute_hash() , valid_moves)
}
