use rand::prelude::IndexedRandom;
use smallvec::SmallVec;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread; // Import the trait for random selection
use std::time::Duration;

use crate::board::constants::{RANK_1, RANK_8};

use super::constants::{FILES, MVV_LVA, get_book_moves};
use super::zobrist::{Z_PIECE, Z_SIDE};
use super::{Board, Bound, GameState, Move, PieceType, TTEntry, TranspositionTable, Turn};

static NODE_COUNT: AtomicU64 = AtomicU64::new(0);
static SKIP_COUNT: AtomicU64 = AtomicU64::new(0);

impl TranspositionTable {
    pub fn new(size_pow2: usize) -> Self {
        let size = 1usize << size_pow2;
        Self {
            table: vec![None; size],
            mask: size - 1,
        }
    } //

    #[inline(always)]
    fn index(&self, key: u64) -> usize {
        (key as usize) & self.mask
    } //

    #[inline(always)]
    pub fn probe(&self, key: u64, depth: i8, alpha: i32, beta: i32) -> Option<i32> {
        let entry = self.table[self.index(key)]?;

        if entry.key != key || entry.depth < depth {
            return None;
        }

        match entry.bound {
            Bound::Exact => Some(entry.score),
            Bound::Lower if entry.score >= beta => Some(entry.score),
            Bound::Upper if entry.score <= alpha => Some(entry.score),
            _ => None,
        }
    } //

    #[inline(always)]
    pub fn store(&mut self, key: u64, depth: i8, score: i32, alpha: i32, beta: i32) {
        let bound = if score <= alpha {
            Bound::Upper
        } else if score >= beta {
            Bound::Lower
        } else {
            Bound::Exact
        };

        let idx = self.index(key);

        match self.table[idx] {
            None => {
                self.table[idx] = Some(TTEntry {
                    key,
                    depth,
                    score,
                    bound,
                });
            }
            Some(old) if depth >= old.depth => {
                self.table[idx] = Some(TTEntry {
                    key,
                    depth,
                    score,
                    bound,
                });
            }
            _ => {}
        }
    } //
}

fn partition_by_bool<T>(v: &mut [T], pred: impl Fn(&T) -> bool) {
    let mut left = 0;
    let mut right = v.len();

    while left < right {
        if pred(&v[left]) {
            left += 1;
        } else {
            right -= 1;
            v.swap(left, right);
        }
    }
}

#[inline(always)]
fn partition_captures<T>(v: &mut [T], mut pred: impl FnMut(&T) -> bool) -> usize {
    let mut left = 0;
    let mut right = v.len();

    while left < right {
        if pred(&v[left]) {
            left += 1;
        } else {
            right -= 1;
            v.swap(left, right);
        }
    }

    left // number of elements matching pred
}

impl Board {
    #[inline]
    pub fn splitmix64(seed: &mut u64) -> u64 {
        *seed = seed.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = *seed;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    } //

    pub fn compute_hash(&self) -> u64 {
        let mut h = 0u64;

        for piece in [
            PieceType::WhitePawn,
            PieceType::WhiteKnight,
            PieceType::WhiteBishop,
            PieceType::WhiteRook,
            PieceType::WhiteQueen,
            PieceType::WhiteKing,
            PieceType::BlackPawn,
            PieceType::BlackKnight,
            PieceType::BlackBishop,
            PieceType::BlackRook,
            PieceType::BlackQueen,
            PieceType::BlackKing,
        ] {
            let mut bb = self.bitboards.0[piece.piece_index()].0;
            let p = piece.piece_index();

            while bb != 0 {
                let sq = bb.trailing_zeros() as usize;
                bb &= bb - 1;
                h ^= Z_PIECE[p][sq];
            }
        }

        if self.turn == Turn::BLACK {
            h ^= *Z_SIDE;
        }

        h
    } //

    /// Returns a random move from the opening book for a given FEN string.
    /// Returns None if the position is not in the book.
    pub fn get_random_opening_move(&self) -> Option<&'static str> {
        // We fetch the slice of available moves
        let moves = get_book_moves(&self.to_fen())?;

        // We choose a random one
        let mut rng = rand::rng();
        moves.choose(&mut rng).copied()
    } //

    /// The score is turn agnostic , it always returns the score of the white player
    pub fn pieces_score_old(&self) -> f32 {
        let mut score: f32 = 0.0;
        let num_of_knights = (self.bitboards.0[PieceType::WhiteKnight.piece_index()])
            .0
            .count_ones();
        let num_of_pawns = self.bitboards.0[PieceType::WhitePawn.piece_index()]
            .0
            .count_ones();
        let num_of_bishops = self.bitboards.0[PieceType::WhiteBishop.piece_index()]
            .0
            .count_ones();
        let num_of_rooks = self.bitboards.0[PieceType::WhiteRook.piece_index()]
            .0
            .count_ones();
        let num_of_queens = self.bitboards.0[PieceType::WhiteQueen.piece_index()]
            .0
            .count_ones();

        let num_of_enemy_knights = self.bitboards.0[PieceType::BlackKnight.piece_index()]
            .0
            .count_ones();
        let num_of_enemy_pawns = self.bitboards.0[PieceType::BlackPawn.piece_index()]
            .0
            .count_ones();
        let num_of_enemy_bishops = self.bitboards.0[PieceType::BlackBishop.piece_index()]
            .0
            .count_ones();
        let num_of_enemy_rooks = self.bitboards.0[PieceType::BlackRook.piece_index()]
            .0
            .count_ones();
        let num_of_enemy_queens = self.bitboards.0[PieceType::BlackQueen.piece_index()]
            .0
            .count_ones();

        score += (num_of_knights * 3) as f32;
        score += (num_of_pawns * 1) as f32;
        score += (num_of_bishops * 3) as f32;
        score += (num_of_rooks * 5) as f32;
        score += (num_of_queens * 9) as f32;

        score -= (num_of_enemy_knights * 3) as f32;
        score -= (num_of_enemy_pawns * 1) as f32;
        score -= (num_of_enemy_bishops * 3) as f32;
        score -= (num_of_enemy_rooks * 5) as f32;
        score -= (num_of_enemy_queens * 9) as f32;
        score
    } //

    #[inline(always)]
    pub fn pieces_score(&self) -> i32 {
        let bbs = &self.bitboards.0;

        let white = bbs[0].0.count_ones() as i32 * 100   // pawn
        + bbs[1].0.count_ones() as i32 * 300   // knight
        + bbs[2].0.count_ones() as i32 * 300   // bishop
        + bbs[3].0.count_ones() as i32 * 500   // rook
        + bbs[4].0.count_ones() as i32 * 900; // queen

        let black = bbs[6].0.count_ones() as i32 * 100
            + bbs[7].0.count_ones() as i32 * 300
            + bbs[8].0.count_ones() as i32 * 300
            + bbs[9].0.count_ones() as i32 * 500
            + bbs[10].0.count_ones() as i32 * 900;

        white - black
    } //

    #[inline(always)]
    pub fn development_score(&self) -> i32 {
        let black = (self.bitboards.0[PieceType::BlackBishop.piece_index()].0
            | self.bitboards.0[PieceType::BlackKnight.piece_index()].0)
            & RANK_8;
        let white = (self.bitboards.0[PieceType::WhiteBishop.piece_index()].0
            | self.bitboards.0[PieceType::WhiteKnight.piece_index()].0)
            & RANK_1;

        (black.count_ones() as i32 - white.count_ones() as i32) * 10
    } //

    pub fn double_rook_bonus(&self) -> f32 {
        let mut bonus: f32 = 0.0;

        for file in FILES {
            let white_rooks_on_file =
                ((self.bitboards.0[PieceType::WhiteRook.piece_index()]).0 & file).count_ones();
            let black_rooks_on_file =
                ((self.bitboards.0[PieceType::BlackRook.piece_index()]).0 & file).count_ones();

            if white_rooks_on_file >= 2 {
                bonus += 0.5;
            }
            if black_rooks_on_file >= 2 {
                bonus -= 0.5;
            }
        }

        bonus
    } //

    pub fn evaluate(&mut self) -> i32 {
        let score = self.pieces_score();
        // score += self.development_score();

        // It returns score relative to the side to play (by default it is white)
        return if self.turn == Turn::WHITE {
            score
        } else {
            -score
        };
    } //

    #[inline(always)]
    pub fn mvv_lva(&self, mv: Move) -> i32 {
        if !mv.is_capture() {
            return 0;
        }

        let victim = self.piece_at[mv.to() as usize].unwrap();
        let attacker = mv.piece();

        MVV_LVA[(victim.piece_index() % 6) as usize][(attacker.piece_index() % 6) as usize]
    } //

    pub fn sort_by_mvv_lva(&mut self, moves: &mut SmallVec<[Move; 256]>) {
        let split = partition_captures(moves, |mv| mv.is_capture());

        // Sort captures only
        moves[..split].sort_unstable_by(|a, b| {
            let va = self.mvv_lva(*a);
            let vb = self.mvv_lva(*b);
            vb.cmp(&va)
        });

        // Quiet moves stay untouched
    } //

    //currently only play for white
    pub fn minimax(&mut self, depth: i32, moves_map: &mut HashMap<u64, (i32, i32)>) -> i32 {
        let game_state = self.get_game_state();
        if game_state == GameState::CheckMate {
            match self.turn {
                Turn::WHITE => return i32::MIN + depth,
                Turn::BLACK => return i32::MAX - depth,
            }
        } else if game_state == GameState::StaleMate {
            return 0;
        }

        if depth >= 4 {
            return self.evaluate();
        }

        let moves: Vec<Move>;
        if let Some((score, stored_depth)) = moves_map.get(&self.hash) {
            if *stored_depth > depth {
                return *score;
            }
        }
        let moves = self.generate_moves();
        match self.turn {
            Turn::WHITE => {
                let mut best_score = i32::MIN;
                for mv in moves {
                    let unmake_move = self.make_move(mv);

                    let score = self.minimax(depth + 1, moves_map);

                    self.unmake_move(unmake_move);

                    if score > best_score {
                        best_score = score;
                    }
                }
                moves_map.insert(self.hash, (best_score, depth));
                return best_score;
            } //
            Turn::BLACK => {
                let mut best_score = i32::MAX;
                for mv in moves {
                    let unmake_move = self.make_move(mv);

                    let score = self.minimax(depth + 1, moves_map);

                    self.unmake_move(unmake_move);

                    if score < best_score {
                        best_score = score;
                    }
                }
                moves_map.insert(self.hash, (best_score, depth));
                return best_score;
            } //
        } //
    } //

    pub fn alpha_beta(
        &mut self,
        depth: i32,
        max_depth: i32,
        mut alpha: i32,
        beta: i32,
        tt: &mut TranspositionTable,
        is_tt: bool,
        is_null_move_pruning: bool,
    ) -> i32 {
        NODE_COUNT.fetch_add(1, Ordering::Relaxed);

        let depth_remaining = max_depth - depth;
        let orig_alpha = alpha;

        // if NODE_COUNT.load(Ordering::Relaxed) % 10_000_000 == 0 {
        //     dbg!("Nodes: {}", NODE_COUNT.load(Ordering::Relaxed));
        // }

        // 1. TT LOOKUP
        if is_tt {
            if let Some(score) = tt.probe(self.hash, depth_remaining as i8, alpha, beta) {
                // SKIP_COUNT.fetch_add(1, Ordering::Relaxed);
                return score;
            }
        }

        // 2. BASE CASE (Optimized)
        if depth >= max_depth {
            let score = self.evaluate();
            return score;
        }

        // 3. NULL MOVE PRUNING
        if depth_remaining >= 3 && !self.is_king_in_check(self.turn) && is_null_move_pruning {
            let r = 2;
            self.switch_turn();
            let score =
                -self.alpha_beta(depth + r + 1, max_depth, -beta + 1, -beta, tt, false, false);
            self.switch_turn();
            if score >= beta {
                return beta;
            }
        };

        // 3. MOVE GENERATION (Only for internal nodes)
        let mut moves = SmallVec::new();
        self.generate_pesudo_moves(&mut moves);

        self.sort_by_mvv_lva(&mut moves);

        let iter = moves.iter();

        let mut found_legal = false;

        let mut best_score = -30_000;

        for mv in iter {
            let unmake_move = self.make_move(*mv);

            // Filter illegal moves
            if self.is_king_in_check(self.opposite_turn()) {
                self.unmake_move(unmake_move);
                continue;
            }

            found_legal = true;

            let score = -self.alpha_beta(
                depth + 1,
                max_depth,
                -beta,
                -alpha,
                tt,
                is_tt,
                is_null_move_pruning,
            );

            self.unmake_move(unmake_move);

            best_score = best_score.max(score);
            alpha = alpha.max(best_score);

            if alpha >= beta {
                break; // Alpha Cutoff
            }
        }

        if !found_legal {
            if self.is_king_in_check(self.turn) {
                let mate = 30_000;
                return -mate + depth;
            } else {
                return 0; // Stalemate
            }
        };

        if is_tt {
            tt.store(
                self.hash,
                depth_remaining as i8,
                best_score,
                orig_alpha,
                beta,
            );
        };

        return best_score;
    } //

    pub fn engine_singlethread(
        &mut self,
        max_depth: i32,
        is_tt: bool,
        is_null_move_pruning: bool,
        maximum_time: Duration,
    ) -> Move {
        let mut moves = self.generate_moves();
        partition_by_bool(&mut moves, |mv| mv.is_capture());

        let start_time = std::time::Instant::now();

        let mut searched_depth = 0;
        let mut best_stable_move = moves[0];
        let mut best_move = moves[0];
        let mut tt = TranspositionTable::new(20);

        for current_depth in 1..=max_depth {
            // dbg!(current_depth);
            let mut alpha = -30_000;
            let beta = 30_000;
            let mut best_score = -30_000;

            for mv in &moves {
                if start_time.elapsed() > maximum_time {
                    dbg!(searched_depth);
                    return best_stable_move;
                }

                let unmake_move = self.make_move(*mv);

                let score = -self.alpha_beta(
                    0,
                    current_depth,
                    -beta,
                    -alpha,
                    &mut tt,
                    is_tt,
                    is_null_move_pruning,
                );

                if score > best_score {
                    best_score = score;
                    best_move = *mv;
                }

                alpha = alpha.max(score);

                self.unmake_move(unmake_move);
            }

            if let Some(idx) = moves.iter().position(|m| *m == best_move) {
                moves.swap(0, idx);
            };

            searched_depth = current_depth;
            best_stable_move = best_move;
        }

        dbg!(searched_depth);
        dbg!(NODE_COUNT.load(Ordering::Relaxed));
        return best_stable_move;
    } //

    pub fn engine_multithreaded(&mut self, max_depth: i32, number_of_threads: i32) -> Move {
        let start_time = std::time::Instant::now();
        let mut moves = self.generate_moves();
        partition_by_bool(&mut moves, |mv| mv.is_capture());

        let mut scored: Vec<(i32, Move)> = Vec::new();

        for mv in &moves {
            let unmake_move = self.make_move(*mv);

            let mut score = -self.alpha_beta(
                0,
                4,
                -30_000,
                30_000,
                &mut TranspositionTable::new(20),
                false,
                false,
            );

            if self.turn == Turn::WHITE {
                score = -score;
            };

            scored.push((score, *mv));

            self.unmake_move(unmake_move);
        }

        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        moves = scored.iter().map(|mv| mv.1).collect();

        let best = Arc::new(Mutex::new((-30_000, moves[0])));

        let threads = number_of_threads as usize;
        let chunk_size = (moves.len() + threads - 1) / threads;

        let mut handles = Vec::new();

        for chunck in moves.chunks(chunk_size) {
            let mut board = self.clone();
            let best = Arc::clone(&best);
            let mut chunck = chunck.to_vec();
            partition_by_bool(&mut chunck, |mv| mv.is_capture());

            handles.push(thread::spawn(move || {
                let mut tt = TranspositionTable::new(20);

                for mv in chunck {
                    let unmake_move = board.make_move(mv);

                    let mut score =
                        -board.alpha_beta(0, max_depth, -30_000, 30_000, &mut tt, false, false);

                    if board.turn == Turn::WHITE {
                        score = -score;
                    }

                    let mut best = best.lock().unwrap();
                    if score > best.0 {
                        *best = (score, mv);
                    }

                    board.unmake_move(unmake_move);
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        dbg!(start_time.elapsed().as_millis());
        dbg!(NODE_COUNT.load(Ordering::Relaxed));

        return best.lock().unwrap().1.clone();
    } //

    pub fn engine(
        &mut self,
        max_depth: i32,
        threads: i32,
        is_tt: bool,
        is_null_move_pruning: bool,
    ) -> Move {
        if let Some(uci) = self.get_random_opening_move() {
            let bytes = uci.as_bytes();

            // Decode squares
            let file_from = bytes[0] - b'a';
            let rank_from = bytes[1] - b'1';
            let from = (rank_from << 3) | file_from;

            let file_to = bytes[2] - b'a';
            let rank_to = bytes[3] - b'1';
            let to = (rank_to << 3) | file_to;

            // Get moving piece from board
            let piece = self.piece_at[from as usize]
                .expect("Opening book move refers to empty from-square");

            // Detect capture
            let capture = self.piece_at[to as usize].is_some();

            dbg!("Opening book move: {}", uci);

            return Move::new(from, to, piece, capture);
        };

        if threads > 1 {
            self.engine_multithreaded(max_depth, threads)
        } else {
            self.engine_singlethread(
                max_depth,
                is_tt,
                is_null_move_pruning,
                Duration::from_secs(15),
            )
        }
    } //

    pub fn perft(&mut self, depth: i32, max_depth: i32) -> i64 {
        if depth == max_depth {
            return 1;
        }

        let mut moves = SmallVec::new();
        self.generate_pesudo_moves(&mut moves);

        // println!("{} || {} , {}", self.to_fen() , depth, moves.len());

        if moves.is_empty() {
            return 1;
        }

        let mut nodes = 0;

        for mv in moves {
            // let before = self.clone();
            let unmake = self.make_move(mv);

            if self.is_king_in_check(self.opposite_turn()) {
                self.unmake_move(unmake);
                continue;
            }

            nodes += self.perft(depth + 1, max_depth);

            self.unmake_move(unmake);
            // assert_eq!(*self, before);
        }

        nodes
    } //
}

mod test {

    #[test]
    fn is_position_equal() {
        use super::Board;
        let mut board = Board::new();
        assert_eq!(board.evaluate(), 0);
    }

    #[test]
    fn generate_move() {
        use super::Board;
        use crate::board::bishop_magic::init_bishop_magics;
        use crate::board::rook_magic::init_rook_magics;

        init_bishop_magics();
        init_rook_magics();

        let mut board = Board::new();
        board.load_from_fen("r1b1k2r/ppq1bppp/2n2n2/2p1p3/8/P1N2N2/1PPPBPPP/R1BQ1K1R w");
        let start_time = std::time::Instant::now();
        let best_move = board.engine(8, 1, true, true);
        dbg!(start_time.elapsed());
        println!("{:?}", best_move.to_uci());
        // println!("{:?} {:?}", best_move.from(), best_move.to());

        // let moves = board.generate_moves();
        // for mv in moves {
        //     println!("{:?} {:?} {:?}", mv.to_uci() ,mv.from(), mv.to());
        // }
    }

    #[test]
    fn perft() {
        use super::Board;
        let mut board = Board::new();
        // board.load_from_fen("2kr3r/1pp3pp/p7/2b1np2/P3p1nq/4P3/1P1PBP1P/RNBQK2R w");
        dbg!(board.perft(0, 4));
    }
}
