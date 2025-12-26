use rand::prelude::IndexedRandom;
use smallvec::SmallVec;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread; // Import the trait for random selection
use std::time::Duration;

use crate::board::constants::{RANK_1, RANK_8};

use super::constants::{MVV_LVA, get_book_moves};
use super::zobrist::{Z_PIECE, Z_SIDE};
use super::{Board, Bound, Move, PieceType, TTEntry, TranspositionTable, Turn};

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
} //

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
        let black_undeveloped = (self.bitboards.0[PieceType::BlackBishop.piece_index()].0
            | self.bitboards.0[PieceType::BlackKnight.piece_index()].0)
            & RANK_8;

        let white_undeveloped = (self.bitboards.0[PieceType::WhiteBishop.piece_index()].0
            | self.bitboards.0[PieceType::WhiteKnight.piece_index()].0)
            & RANK_1;

        let black_penalty = black_undeveloped.count_ones() as i32 * 10;
        let white_penalty = white_undeveloped.count_ones() as i32 * 10;

        white_penalty - black_penalty
    }

    pub fn evaluate(&mut self) -> i32 {
        let mut score = self.pieces_score();
        score += self.development_score();

        return score;
    } //

    #[inline(always)]
    pub fn mvv_lva(&self, mv: Move) -> i32 {
        if !mv.is_capture() || mv.is_en_passant() {
            return 0;
        }
        let mut victim;

        victim = self.piece_at[mv.to() as usize].unwrap();
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
            if mv.is_castling() {
                match mv.to() {
                    6 => {
                        if self.is_square_attacked(6, self.opposite_turn()) {
                            continue;
                        } else if self.is_square_attacked(5, self.opposite_turn()) {
                            continue;
                        } else if self.is_square_attacked(4, self.opposite_turn()) {
                            continue;
                        }
                    }
                    2 => {
                        if self.is_square_attacked(2, self.opposite_turn()) {
                            continue;
                        } else if self.is_square_attacked(3, self.opposite_turn()) {
                            continue;
                        } else if self.is_square_attacked(4, self.opposite_turn()) {
                            continue;
                        }
                    }
                    58 => {
                        if self.is_square_attacked(58, self.opposite_turn()) {
                            continue;
                        } else if self.is_square_attacked(59, self.opposite_turn()) {
                            continue;
                        } else if self.is_square_attacked(60, self.opposite_turn()) {
                            continue;
                        }
                    }
                    62 => {
                        if self.is_square_attacked(62, self.opposite_turn()) {
                            continue;
                        } else if self.is_square_attacked(61, self.opposite_turn()) {
                            continue;
                        } else if self.is_square_attacked(60, self.opposite_turn()) {
                            continue;
                        }
                    }
                    _ => (),
                }
            };

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

    pub fn engine(
        &mut self,
        max_depth: i32,
        threads: i32,
        is_tt: bool,
        is_null_move_pruning: bool,
        maximum_time: std::time::Duration,
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

            return Move::new(from, to, piece, capture, false, false, false);
        };

        self.engine_singlethread(max_depth, is_tt, is_null_move_pruning, maximum_time)
    } //

    pub fn perft(&mut self, depth: i32, max_depth: i32) -> i64 {
        if depth == max_depth {
            return 1;
        }
        let mut moves = SmallVec::new();
        self.generate_pesudo_moves(&mut moves);

        let mut nodes = 0;

        for mv in moves {
            let unmake = self.make_move(mv);

            if self.is_king_in_check(self.opposite_turn()) {
                self.unmake_move(unmake);
                continue;
            }

            let inner_nodes = self.perft(depth + 1, max_depth);

            self.unmake_move(unmake);

            if depth == 0 {
                println!("{} {}", mv.to_uci(), inner_nodes);
            }

            nodes += inner_nodes;
        }

        nodes
    } //
}

mod test {
    use std::collections::HashMap;

    use crate::board::board;

    #[test]
    fn test() {
        use crate::board::bishop_magic::init_bishop_magics;
        use crate::board::rook_magic::init_rook_magics;

        init_rook_magics();
        init_bishop_magics();

        let mut board = board::Board::new();
        board.load_from_fen("5r1k/p5p1/1pp2r1p/4N3/P7/RQ1PpP1P/1P5q/5K2 w");

        let start = std::time::Instant::now();
        // dbg!(
        //     board
        //         .generate_moves()
        //         .iter()
        //         .map(|mv| mv.to_uci())
        //         .collect::<Vec<String>>()
        // );
        dbg!(
            board
                .engine(8, 1, false, false, std::time::Duration::from_secs(10))
                .to_uci()
        );
        dbg!(start.elapsed());
    } //

    #[test]
    fn fens() {
        use crate::board::bishop_magic::init_bishop_magics;
        use crate::board::rook_magic::init_rook_magics;

        init_rook_magics();
        init_bishop_magics();

        let fens: HashMap<&str, Vec<[u8; 2]>> = HashMap::from([
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w",
                vec![[12, 28], [11, 27], [6, 21], [10, 26], [12, 20]],
            ),
            (
                "r1bqkbnr/pppppppp/2n5/8/4P3/8/PPPP1PPP/RNBQKBNR w",
                vec![[11, 27], [6, 21], [1, 18]],
            ),
            (
                "r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w",
                vec![[5, 26], [1, 18], [11, 27], [5, 33], [15, 23]],
            ),
            (
                "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w",
                vec![[21, 38], [3, 12], [11, 19]],
            ),
            (
                "r1bqkb1r/ppp2ppp/2n2n2/3pp1N1/2B1P3/8/PPPP1PPP/RNBQK2R w",
                vec![[28, 35]],
            ),
            (
                "r1bqkb1r/ppp2ppp/5n2/3Pp1N1/2Bn4/8/PPPP1PPP/RNBQK2R w",
                vec![[10, 18], [35, 43], [1, 18]],
            ),
            (
                "r1b1kb1r/ppp2ppp/3q1n2/4p1N1/2Bn4/8/PPPP1PPP/RNBQK2R w",
                vec![[11, 19], [10, 18]],
            ),
            (
                "r1b2b1r/ppp1kBpp/3q1n2/4p1N1/3n4/8/PPPP1PPP/RNBQK2R w",
                vec![[53, 17]],
            ),
            (
                "r1b2b1r/ppp1kBp1/3q1n1p/4p1N1/3n4/2P5/PP1P1PPP/RNBQK2R w",
                vec![[18, 27], [53, 26]],
            ),
            (
                "r1b2b1r/ppp2kp1/3q1n1p/4p3/3n4/2P4N/PP1P1PPP/RNBQK2R w",
                vec![[18, 27]],
            ),
            (
                "r1b2b1r/ppp2kp1/3q1n1p/8/3p4/7N/PP1P1PPP/RNBQK2R w",
                vec![[11, 19], [4, 5]],
            ),
            (
                "r4b1r/ppp2kp1/3qbn1p/8/3p4/1Q5N/PP1P1PPP/RNB1K2R w",
                vec![[17, 3]],
            ),
            (
                "4rb1r/ppp2kp1/3qbn1p/8/3p4/7N/PP1P1PPP/RNBQK2R w",
                vec![[4, 5]],
            ),
            (
                "4rb1r/ppp2kp1/4bn1p/4q3/3p4/7N/PP1P1PPP/RNBQ1K1R w",
                vec![[11, 19]],
            ),
            (
                "4rb1r/ppp2kp1/5n1p/4q3/3p2b1/3P3N/PP3PPP/RNBQ1K1R w",
                vec![[13, 21]],
            ),
            (
                "4r2r/ppp2kp1/5n1p/4q3/1b1p2b1/3P1P1N/PP4PP/RNBQ1K1R w",
                vec![[1, 11]],
            ),
            (
                "4r2r/ppp2kp1/5n1p/4q3/3p2b1/3P1P1N/PP1b2PP/RN1Q1K1R w",
                vec![[1, 11]],
            ),
            (
                "4r2r/ppp2kp1/5n1p/4q3/3p4/3P1P1b/PP1N2PP/R2Q1K1R w",
                vec![[14, 23]],
            ),
            (
                "4rr2/ppp2kp1/5n1p/4q3/3p4/3P1P1P/PP1N3P/R2Q1K1R w",
                vec![[11, 28], [3, 17]],
            ),
            (
                "4rr2/ppp2kp1/5n1p/8/2Np1q2/3P1P1P/PP5P/R2Q1K1R w",
                vec![[7, 6]],
            ),
            (
                "4rr2/pp3kp1/2p2n1p/8/2Np1q2/3P1P1P/PP5P/2RQ1K1R w",
                vec![[7, 6]],
            ),
            (
                "5r2/pp3kp1/2p1rn1p/8/P1Np1q2/3P1P1P/1P5P/2RQ1K1R w",
                vec![[24, 32], [7, 6]],
            ),
            (
                "5r2/pp3kp1/2p1r2p/3n4/P1Np1q2/3P1P1P/1P5P/R2Q1K1R w",
                vec![[7, 6]],
            ),
            (
                "5rk1/pp4p1/2p1r2p/3n4/P1Np1q2/R2P1P1P/1P5P/3Q1K1R w",
                vec![[26, 43]],
            ),
            (
                "5rk1/p5p1/1pp1r2p/3n4/P1Np1q2/R2P1P1P/1P5P/3Q1KR1 w",
                vec![[6, 22]],
            ),
            (
                "5rk1/p5p1/1pp1r2p/3n4/P1Np4/R2P1P1P/1P5q/3Q1K2 w",
                vec![[30, 6], [9, 17], [16, 0], [23, 31], [24, 32]],
            ),
            (
                "5rk1/p5p1/1pp2r1p/3n4/P1NpR3/R2P1P1P/1P5q/3Q1K2 w",
                vec![[5, 4]],
            ),
            (
                "5rk1/p5p1/1pp2r1p/4N3/P2pR3/R2PnP1P/1P5q/3Q1K2 w",
                vec![[28, 20], [5, 4]],
            ),
            (
                "5rk1/p5p1/1pp2r1p/4N3/P7/R2PpP1P/1P5q/3Q1K2 w",
                vec![[3, 17], [3, 10], [36, 30], [3, 4], [3, 11]],
            ),
            (
                "5r1k/p5p1/1pp2r1p/4N3/P7/RQ1PpP1P/1P5q/5K2 w",
                vec![[17, 10], [17, 62], [36, 46], [36, 53], [5, 4]],
            ),
            (
                "5r1k/p5p1/1pp4p/4N3/P7/R2Ppr1P/1PQ4q/5K2 w",
                vec![[36, 21], [10, 13], [5, 4]],
            ),
            (
                "7k/p5p1/1pp4p/8/P7/R2Ppr1P/1PQ4q/5K2 w",
                vec![[5, 4], [10, 13]],
            ),
            (
                "7k/p5p1/1pp4p/8/P7/R2Ppr1P/1Pq5/4K3 w",
                vec![[9, 17], [19, 27], [23, 31], [24, 32], [9, 25]],
            ),
        ]);

        for (i, (fen, moves)) in fens.iter().enumerate() {
            println!("test {}/{}", i + 1, fens.len());
            println!("fen: {}", fen);
            println!("moves: {:?}\n", moves);
            let mut board = board::Board::new();
            board.load_from_fen(fen);

            let best_move = board.engine(64, 1, true, true, std::time::Duration::from_secs(20));
            dbg!((best_move.from(), best_move.to()));

            if moves.contains(&[best_move.from() as u8, best_move.to() as u8]) {
                println!("✅ ok");
            } else {
                println!("❌ fail");
            }

            println!("\n\n\n");
        }
    } //
}
