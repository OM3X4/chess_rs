use rand::prelude::IndexedRandom;
use smallvec::SmallVec;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, current}; // Import the trait for random selection
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
        // score += self.development_score();

        if self.turn == Turn::BLACK {
            return -score;
        }
        score
    } //

    #[inline(always)]
    pub fn mvv_lva(&self, mv: Move) -> i32 {
        if !mv.is_capture() || mv.is_en_passant() {
            return 0;
        }
        let mut victim;

        victim = self.piece_at[mv.to() as usize].unwrap_or_else(|| {
            println!("{}", mv.to());
            println!("{}", mv.from());
            println!("{}", self.to_fen());
            panic!()
        });
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

    pub fn quiescence(&mut self, alpha: i32, beta: i32) -> i32 {
        let stand_pat = self.eval;

        let margin = 900; // queen value

        if stand_pat + margin < alpha {
            return alpha;
        }


        if stand_pat >= beta {
            return beta;
        }
        let mut alpha = alpha.max(stand_pat);

        let mut moves = SmallVec::new();
        self.generate_pesudo_moves(&mut moves);

        let mut iter = moves.iter().filter(|mv| mv.is_capture());

        for mv in iter {
            let undo = self.make_move(*mv);

            // after make_move, side-to-move is the opponent
            // ensure the player who just moved is not in check
            if self.is_king_in_check(self.opposite_turn()) {
                self.unmake_move(undo);
                continue;
            }

            let score = -self.quiescence(-beta, -alpha);
            self.unmake_move(undo);

            if score >= beta {
                return beta;
            }
            alpha = alpha.max(score);
        }

        alpha
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
        is_lmr: bool,
        is_quiesense: bool,
    ) -> i32 {
        NODE_COUNT.fetch_add(1, Ordering::Relaxed);

        let depth_remaining = max_depth - depth;
        let orig_alpha = alpha;

        // 1. TT LOOKUP
        if is_tt {
            if let Some(score) = tt.probe(self.hash, depth_remaining as i8, alpha, beta) {
                // SKIP_COUNT.fetch_add(1, Ordering::Relaxed);
                return score;
            }
        };

        // 2. BASE CASE (Optimized)
        if depth >= max_depth {
            if is_quiesense {
                return self.quiescence(alpha, beta);
            }
            // return self.evaluate();
            match self.turn {
                Turn::BLACK => return -self.eval,
                Turn::WHITE => return self.eval,
            }
        };

        // 3. NULL MOVE PRUNING
        if depth_remaining >= 3 && !self.is_king_in_check(self.turn) && is_null_move_pruning {
            let r = 2;
            self.switch_turn();
            let score = -self.alpha_beta(
                depth + r + 1,
                max_depth,
                -beta + 1,
                -beta,
                tt,
                false,
                false,
                false,
                false,
            );
            self.switch_turn();
            if score >= beta {
                return beta;
            }
        };

        // 4. MOVE GENERATION (Only for internal nodes)
        let mut moves = SmallVec::new();
        self.generate_pesudo_moves(&mut moves);

        self.sort_by_mvv_lva(&mut moves);

        let iter = moves.iter();

        let mut found_legal = false;

        let mut best_score = -30_000;

        let opposite_turn = self.opposite_turn();

        let remaining_depth = max_depth - depth;
        let remaining_depth_next = remaining_depth - 1;

        for (index, mv) in iter.enumerate() {
            if mv.is_castling() {
                match mv.to() {
                    6 => {
                        if self.is_square_attacked(6, opposite_turn) {
                            continue;
                        } else if self.is_square_attacked(5, opposite_turn) {
                            continue;
                        } else if self.is_square_attacked(4, opposite_turn) {
                            continue;
                        }
                    }
                    2 => {
                        if self.is_square_attacked(2, opposite_turn) {
                            continue;
                        } else if self.is_square_attacked(3, opposite_turn) {
                            continue;
                        } else if self.is_square_attacked(4, opposite_turn) {
                            continue;
                        }
                    }
                    58 => {
                        if self.is_square_attacked(58, opposite_turn) {
                            continue;
                        } else if self.is_square_attacked(59, opposite_turn) {
                            continue;
                        } else if self.is_square_attacked(60, opposite_turn) {
                            continue;
                        }
                    }
                    62 => {
                        if self.is_square_attacked(62, opposite_turn) {
                            continue;
                        } else if self.is_square_attacked(61, opposite_turn) {
                            continue;
                        } else if self.is_square_attacked(60, opposite_turn) {
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
            };

            found_legal = true;

            let mut score: i32 = 0;

            let can_lmr = !mv.is_capture() && index >= 4 && remaining_depth_next >= 3 && is_lmr;

            if !can_lmr {
                score = -self.alpha_beta(
                    depth + 1,
                    max_depth,
                    -beta,
                    -alpha,
                    tt,
                    is_tt,
                    is_null_move_pruning,
                    is_lmr,
                    is_quiesense,
                );
            } else {
                // Reduction

                let reduction = 1;
                let reduced_remaining = remaining_depth_next - reduction;
                let reduced_max_depth = depth + 1 + reduced_remaining;

                let reduced_score = -self.alpha_beta(
                    depth + 1,
                    reduced_max_depth,
                    -beta,
                    -alpha,
                    tt,
                    is_tt,
                    is_null_move_pruning,
                    is_lmr,
                    false,
                );

                if reduced_score >= beta {
                    score = -self.alpha_beta(
                        depth + 1,
                        max_depth,
                        -beta,
                        -alpha,
                        tt,
                        is_tt,
                        is_null_move_pruning,
                        is_lmr,
                        is_quiesense,
                    );
                } else {
                    score = reduced_score;
                }
            }

            self.unmake_move(unmake_move);

            best_score = best_score.max(score);
            alpha = alpha.max(best_score);

            if alpha >= beta {
                break; // Alpha Cutoff
            }
        } //

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
        is_lmr: bool,
        is_quiesense: bool,
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
                    is_lmr,
                    is_quiesense,
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
        is_lmr: bool,
        is_quiesense: bool,
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

        self.engine_singlethread(
            max_depth,
            is_tt,
            is_null_move_pruning,
            is_lmr,
            is_quiesense,
            maximum_time,
        )
    } //

    pub fn perft(&mut self, depth: i32, max_depth: i32) -> i64 {
        // if self.eval != self.evaluate() && self.eval != -self.evaluate() {
        //     println!("Mismatched evaluation")
        // }
        if depth == max_depth {
            return 1;
        }
        let mut moves = SmallVec::new();
        self.generate_pesudo_moves(&mut moves);

        let mut nodes = 0;

        // if depth == 0 { println!("Number of moves : {}" , moves.len()); }

        // if depth == 1 {
        //     println!("The current castling {:b}", self.castling);
        // }

        let opposite_turn = self.opposite_turn();
        let current_turn = self.turn; // Will be the opposite one making the move

        for mv in moves {
            if mv.is_castling() {
                match mv.to() {
                    6 => {
                        if self.is_square_attacked(6, opposite_turn) {
                            continue;
                        } else if self.is_square_attacked(5, opposite_turn) {
                            continue;
                        } else if self.is_square_attacked(4, opposite_turn) {
                            continue;
                        }
                    }
                    2 => {
                        if self.is_square_attacked(2, opposite_turn) {
                            continue;
                        } else if self.is_square_attacked(3, opposite_turn) {
                            continue;
                        } else if self.is_square_attacked(4, opposite_turn) {
                            continue;
                        }
                    }
                    58 => {
                        if self.is_square_attacked(58, opposite_turn) {
                            continue;
                        } else if self.is_square_attacked(59, opposite_turn) {
                            continue;
                        } else if self.is_square_attacked(60, opposite_turn) {
                            continue;
                        }
                    }
                    62 => {
                        if self.is_square_attacked(62, opposite_turn) {
                            continue;
                        } else if self.is_square_attacked(61, opposite_turn) {
                            continue;
                        } else if self.is_square_attacked(60, opposite_turn) {
                            continue;
                        }
                    }
                    _ => (),
                }
            };

            // Turn switches here
            let unmake = self.make_move(mv);

            if self.is_king_in_check(current_turn) {
                self.unmake_move(unmake);
                continue;
            }

            let inner_nodes = self.perft(depth + 1, max_depth);

            // Turn switches back
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
        board.load_from_fen("1qr1k2r/1p2bp2/pBn1p3/P2pPbpp/5P2/2P1QBPP/1P1N3R/R4K2 b k -");

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
                .engine(
                    6,
                    1,
                    false,
                    false,
                    false,
                    false,
                    std::time::Duration::from_secs(300)
                )
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

        // let fens: HashMap<&str, Vec<[u8; 2]>> = HashMap::from([
        //     (
        //         "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w",
        //         vec![[21, 38], [3, 12], [11, 19]],
        //     ),
        //     (
        //         "r1bqkb1r/ppp2ppp/2n2n2/3pp1N1/2B1P3/8/PPPP1PPP/RNBQK2R w",
        //         vec![[28, 35]],
        //     ),
        //     (
        //         "r1bqkb1r/ppp2ppp/5n2/3Pp1N1/2Bn4/8/PPPP1PPP/RNBQK2R w",
        //         vec![[10, 18], [35, 43], [1, 18]],
        //     ),
        //     (
        //         "r1b1kb1r/ppp2ppp/3q1n2/4p1N1/2Bn4/8/PPPP1PPP/RNBQK2R w",
        //         vec![[11, 19], [10, 18]],
        //     ),
        //     (
        //         "r1b2b1r/ppp1kBpp/3q1n2/4p1N1/3n4/8/PPPP1PPP/RNBQK2R w",
        //         vec![[53, 17]],
        //     ),
        //     (
        //         "r1b2b1r/ppp1kBp1/3q1n1p/4p1N1/3n4/2P5/PP1P1PPP/RNBQK2R w",
        //         vec![[18, 27], [53, 26]],
        //     ),
        //     (
        //         "r1b2b1r/ppp2kp1/3q1n1p/4p3/3n4/2P4N/PP1P1PPP/RNBQK2R w",
        //         vec![[18, 27]],
        //     ),
        //     (
        //         "r1b2b1r/ppp2kp1/3q1n1p/8/3p4/7N/PP1P1PPP/RNBQK2R w",
        //         vec![[11, 19], [4, 5]],
        //     ),
        //     (
        //         "r4b1r/ppp2kp1/3qbn1p/8/3p4/1Q5N/PP1P1PPP/RNB1K2R w",
        //         vec![[17, 3]],
        //     ),
        //     (
        //         "4rb1r/ppp2kp1/3qbn1p/8/3p4/7N/PP1P1PPP/RNBQK2R w",
        //         vec![[4, 5]],
        //     ),
        //     (
        //         "4rb1r/ppp2kp1/4bn1p/4q3/3p4/7N/PP1P1PPP/RNBQ1K1R w",
        //         vec![[11, 19]],
        //     ),
        //     (
        //         "4rb1r/ppp2kp1/5n1p/4q3/3p2b1/3P3N/PP3PPP/RNBQ1K1R w",
        //         vec![[13, 21]],
        //     ),
        //     (
        //         "4r2r/ppp2kp1/5n1p/4q3/1b1p2b1/3P1P1N/PP4PP/RNBQ1K1R w",
        //         vec![[1, 11]],
        //     ),
        //     (
        //         "4r2r/ppp2kp1/5n1p/4q3/3p2b1/3P1P1N/PP1b2PP/RN1Q1K1R w",
        //         vec![[1, 11]],
        //     ),
        //     (
        //         "4r2r/ppp2kp1/5n1p/4q3/3p4/3P1P1b/PP1N2PP/R2Q1K1R w",
        //         vec![[14, 23]],
        //     ),
        //     (
        //         "4rr2/ppp2kp1/5n1p/4q3/3p4/3P1P1P/PP1N3P/R2Q1K1R w",
        //         vec![[11, 28], [3, 17]],
        //     ),
        //     (
        //         "4rr2/ppp2kp1/5n1p/8/2Np1q2/3P1P1P/PP5P/R2Q1K1R w",
        //         vec![[7, 6]],
        //     ),
        //     (
        //         "4rr2/pp3kp1/2p2n1p/8/2Np1q2/3P1P1P/PP5P/2RQ1K1R w",
        //         vec![[7, 6]],
        //     ),
        //     (
        //         "5r2/pp3kp1/2p1rn1p/8/P1Np1q2/3P1P1P/1P5P/2RQ1K1R w",
        //         vec![[24, 32], [7, 6]],
        //     ),
        //     (
        //         "5r2/pp3kp1/2p1r2p/3n4/P1Np1q2/3P1P1P/1P5P/R2Q1K1R w",
        //         vec![[7, 6]],
        //     ),
        //     (
        //         "5rk1/pp4p1/2p1r2p/3n4/P1Np1q2/R2P1P1P/1P5P/3Q1K1R w",
        //         vec![[26, 43]],
        //     ),
        //     (
        //         "5rk1/p5p1/1pp1r2p/3n4/P1Np1q2/R2P1P1P/1P5P/3Q1KR1 w",
        //         vec![[6, 22]],
        //     ),
        //     (
        //         "5rk1/p5p1/1pp1r2p/3n4/P1Np4/R2P1P1P/1P5q/3Q1K2 w",
        //         vec![[30, 6], [9, 17], [16, 0], [23, 31], [24, 32]],
        //     ),
        //     (
        //         "5rk1/p5p1/1pp2r1p/3n4/P1NpR3/R2P1P1P/1P5q/3Q1K2 w",
        //         vec![[5, 4]],
        //     ),
        //     (
        //         "5rk1/p5p1/1pp2r1p/4N3/P2pR3/R2PnP1P/1P5q/3Q1K2 w",
        //         vec![[28, 20], [5, 4]],
        //     ),
        //     (
        //         "5rk1/p5p1/1pp2r1p/4N3/P7/R2PpP1P/1P5q/3Q1K2 w",
        //         vec![[3, 17], [3, 10], [36, 30], [3, 4], [3, 11]],
        //     ),
        //     (
        //         "5r1k/p5p1/1pp2r1p/4N3/P7/RQ1PpP1P/1P5q/5K2 w",
        //         vec![[17, 10], [17, 62], [36, 46], [36, 53], [5, 4]],
        //     ),
        //     (
        //         "5r1k/p5p1/1pp4p/4N3/P7/R2Ppr1P/1PQ4q/5K2 w",
        //         vec![[36, 21], [10, 13], [5, 4]],
        //     ),
        //     (
        //         "7k/p5p1/1pp4p/8/P7/R2Ppr1P/1PQ4q/5K2 w",
        //         vec![[5, 4], [10, 13]],
        //     ),
        //     (
        //         "7k/p5p1/1pp4p/8/P7/R2Ppr1P/1Pq5/4K3 w",
        //         vec![[9, 17], [19, 27], [23, 31], [24, 32], [9, 25]],
        //     ),
        // ]);

        let fens: HashMap<&str, Vec<[u8; 2]>> = HashMap::from([
            (
                "1kr5/3n4/q3p2p/p2n2p1/PppB1P2/5BP1/1P2Q2P/3R2K1 w",
                vec![[29, 37]],
            ),
            (
                "1n5k/3q3p/pp1p2pB/5r2/1PP1Qp2/P6P/6P1/2R3K1 w",
                vec![[26, 34]],
            ),
            (
                "1n6/4bk1r/1p2rp2/pP2pN1p/K1P1N2P/8/P5R1/3R4 w",
                vec![[14, 6], [3, 35], [14, 22], [3, 11], [8, 16]],
            ),
            (
                "1nr5/1k5r/p3pqp1/3p4/1P1P1PP1/R4N2/3Q1PK1/R7 w",
                vec![[25, 33], [16, 17], [0, 1], [11, 20], [0, 4]],
            ),
            (
                "1q2r1k1/1b2bpp1/p2ppn1p/2p5/P3PP1B/2PB1RP1/2P1Q2P/2KR4 b",
                vec![[34, 26]],
            ),
            (
                "1q4k1/5p1p/p1rprnp1/3R4/N1P1P3/1P6/P5PP/3Q1R1K w",
                vec![[28, 36]],
            ),
            (
                "1qr1k2r/1p2bp2/pBn1p3/P2pPbpp/5P2/2P1QBPP/1P1N3R/R4K2 b k -",
                vec![[39, 31]],
            ),
            (
                "1r1b2k1/2r2ppp/p1qp4/3R1NPP/1pn1PQB1/8/PPP3R1/1K6 w",
                vec![[38, 46]],
            ),
            (
                "1r1qk1nr/p3ppbp/3p2p1/1pp5/2bPP3/4B1P1/2PQNPBP/R2R2K1 w k -",
                vec![[38, 46]],
            ),
            (
                "1r1r2k1/p3n2p/b1nqpbp1/2pp4/1p3PP1/2PP1N2/PPN3BP/R1BRQ2K w - -",
                vec![[19, 27], [15, 23], [14, 23]],
            ),
            (
                "1r2n1rk/pP2q2p/P2p4/4pQ2/2P2p2/5B1P/3R1P1K/3R4 w - -",
                vec![[26, 34], [21, 28], [3, 6], [21, 42], [21, 35], [3, 4]],
            ),
            (
                "1r3bk1/7p/pp1q2p1/P1pPp3/2P3b1/4B3/1P1Q2BP/R6K w - -",
                vec![[26, 34], [21, 28], [3, 6], [21, 42], [21, 35], [3, 4]],
            ),
        ]);

        let fens = vec![
            (
                "1kr5/3n4/q3p2p/p2n2p1/PppB1P2/5BP1/1P2Q2P/3R2K1 w - -",
                vec![[29, 37], [27, 36], [27, 13], [21, 30]],
            ),
            (
                "1n5k/3q3p/pp1p2pB/5r2/1PP1Qp2/P6P/6P1/2R3K1 w - -",
                vec![[26, 34], [28, 27], [25, 33], [14, 30]],
            ),
            (
                "1n6/4bk1r/1p2rp2/pP2pN1p/K1P1N2P/8/P5R1/3R4 w - -",
                vec![[26, 34], [3, 19], [3, 11], [14, 22], [3, 35]],
            ),
            (
                "1nr5/1k5r/p3pqp1/3p4/1P1P1PP1/R4N2/3Q1PK1/R7 w - -",
                vec![[25, 33], [14, 22], [21, 38], [11, 20]],
            ),
            (
                "1q2r1k1/1b2bpp1/p2ppn1p/2p5/P3PP1B/2PB1RP1/2P1Q2P/2KR4 b - -",
                vec![[34, 26], [49, 42], [57, 56], [57, 58]],
            ),
            (
                "1q4k1/5p1p/p1rprnp1/3R4/N1P1P3/1P6/P5PP/3Q1R1K w - -",
                vec![[28, 36], [24, 18], [3, 19], [3, 21]],
            ),
            (
                "1qr1k2r/1p2bp2/pBn1p3/P2pPbpp/5P2/2P1QBPP/1P1N3R/R4K2 b k -",
                vec![[39, 31], [52, 59], [52, 61], [63, 55]],
            ),
            (
                "1r1b2k1/2r2ppp/p1qp4/3R1NPP/1pn1PQB1/8/PPP3R1/1K6 w - -",
                vec![[38, 46], [1, 0], [37, 27], [35, 19]],
            ),
            (
                "1r1qk1nr/p3ppbp/3p2p1/1pp5/2bPP3/4B1P1/2PQNPBP/R2R2K1 w k -",
                vec![[28, 36], [14, 21], [12, 2], [0, 48]],
            ),
            (
                "1r1r2k1/p3n2p/b1nqpbp1/2pp4/1p3PP1/2PP1N2/PPN3BP/R1BRQ2K w - -",
                vec![[19, 27], [21, 38], [8, 24], [15, 23]],
            ),
            (
                "1r2n1rk/pP2q2p/P2p4/4pQ2/2P2p2/5B1P/3R1P1K/3R4 w - -",
                vec![[26, 34], [21, 42], [37, 10], [3, 6]],
            ),
            (
                "1r3bk1/7p/pp1q2p1/P1pPp3/2P3b1/4B3/1P1Q2BP/R6K w - -",
                vec![[9, 25], [20, 6], [32, 41], [15, 23]],
            ),
            (
                "1r3rk1/3n1pbp/1q1pp1p1/p1p5/2PnPP2/PPB1N1PP/6B1/1R1Q1RK1 b - -",
                vec![[32, 24], [27, 12], [61, 59], [55, 39]],
            ),
            (
                "1r3rk1/p5bp/6p1/q1pPppn1/7P/1B1PQ1P1/PB3P2/R4RK1 b - -",
                vec![[37, 29], [38, 53], [34, 26], [36, 28]],
            ),
            (
                "1r4k1/1rq2pp1/3b1nn1/pBpPp3/P1N4p/2PP1Q1P/6PB/2R2RK1 w - -",
                vec![[19, 27], [33, 42], [21, 37], [2, 4]],
            ),
            (
                "1r4k1/p1rqbp1p/b1p1p1p1/NpP1P3/3PB3/3Q2P1/P4P1P/3RR1K1 w - -",
                vec![[8, 24], [6, 14], [19, 21], [15, 31]],
            ),
            (
                "2r3k1/p2q1pp1/Pbrp3p/6n1/1BP1PpP1/R4P2/2QN2KP/1R6 b - -",
                vec![[47, 39], [41, 20], [38, 44], [51, 59]],
            ),
            (
                "1r6/2q2pk1/2n1p1pp/p1Pr4/P1RP4/1p1RQ2P/1N3PP1/7K b - -",
                vec![[44, 36], [54, 55], [50, 49], [57, 59]],
            ),
            (
                "1r6/R1nk1p2/1p4pp/pP1p1P2/P2P3P/5PN1/5K2/8 w - -",
                vec![[31, 39], [22, 12], [22, 5], [21, 29]],
            ),
            (
                "1rb3k1/2pn2pp/p2p4/4p3/1pP4q/1P1PBP1P/1PQ2P2/R3R1K1 w - -",
                vec![[26, 34], [6, 5], [6, 14], [0, 32]],
            ),
            (
                "1rbqnrk1/6bp/pp3np1/2pPp3/P1P1N3/2N1B3/1P2Q1BP/R4R1K w - -",
                vec![[24, 32], [20, 38], [7, 6], [28, 45]],
            ),
            (
                "1rr3k1/1q3pp1/pnbQp2p/1p2P3/3B1P2/2PB4/P1P2RPP/R5K1 w - -",
                vec![[29, 37], [43, 16], [0, 3], [15, 23]],
            ),
            (
                "2kr2r1/1bpnqp2/1p1ppn2/p5pp/P1PP4/4PP2/1P1NBBPP/R2Q1RK1 w - -",
                vec![[9, 25], [3, 1], [3, 10], [5, 4]],
            ),
            (
                "2b1k2r/5p2/pq1pNp1b/1p6/2r1PPBp/3Q4/PPP3PP/1K1RR3 w k -",
                vec![[28, 36], [19, 35], [19, 23], [29, 37]],
            ),
            (
                "2b1r1k1/1p6/pQ1p1q1p/P2P3P/2P1pPpN/6P1/4R1K1/8 w - -",
                vec![[26, 34], [14, 6], [14, 15], [12, 20]],
            ),
            (
                "2b2rk1/2qn1p2/p2p2pp/2pPP3/8/4NN1P/P1Q2PP1/bB2R1K1 w - -",
                vec![[36, 44], [20, 26], [20, 30], [36, 43]],
            ),
            (
                "2bq2k1/1pr3bp/1Qpr2p1/P2pNp2/3P1P1P/6P1/5PB1/1RR3K1 w - -",
                vec![[32, 40], [41, 34], [2, 34], [31, 39]],
            ),
            (
                "rr6/8/2pbkp2/ppp1p1p1/P3P3/1P1P1PB1/R1P2PK1/R7 b - -",
                vec![[34, 26], [43, 50], [57, 41], [33, 25]],
            ),
            (
                "2r2rk1/pb2q2p/1pn1p2p/5p1Q/3P4/P1NB4/1P3PPP/R4RK1 w - -",
                vec![[27, 35], [39, 47], [0, 2], [5, 3]],
            ),
            (
                "2kr4/ppqnbp1r/2n1p1p1/P2pP3/3P2P1/3BBN2/1P1Q1PP1/R4RK1 w - -",
                vec![[32, 40], [11, 10], [5, 2], [14, 22]],
            ),
            (
                "2q5/1pb2r1k/p1b3pB/P1Pp3p/3P4/3B1pPP/1R3P1K/2Q5 b - -",
                vec![[39, 31], [42, 33], [42, 51], [58, 59]],
            ),
            (
                "2r1kb1r/1bqn1pp1/p3p3/1p2P1P1/3Np3/P1N1B3/1PP1Q2P/R4RK1 w k -",
                vec![[38, 46], [20, 29], [0, 4], [5, 29]],
            ),
            (
                "2r1rb2/1bq2p1k/3p1np1/p1p5/1pP1P1P1/PP2BPN1/2Q3P1/R2R1BK1 b - -",
                vec![[43, 35], [61, 54], [55, 62], [45, 51]],
            ),
            (
                "2r2bk1/pq3r1p/6p1/2ppP1P1/P7/BP1Q4/2R3P1/3R3K b - -",
                vec![[35, 27], [49, 52], [58, 50], [34, 26]],
            ),
            (
                "2r2rk1/1bb2ppp/p2ppn2/1p4q1/1PnNP3/P1N4P/2P1QPPB/3RRBK1 w - -",
                vec![[16, 24], [27, 21], [3, 1], [3, 19]],
            ),
            (
                "2r2rk1/3q3p/p3pbp1/1p1pp3/4P3/2P5/PPN1QPPP/3R1RK1 b - -",
                vec![[35, 27], [45, 54], [51, 49], [51, 43]],
            ),
            (
                "2r4k/pp3q1b/5PpQ/3p4/3Bp3/1P6/P5RP/6K1 w - -",
                vec![[15, 31], [14, 22], [14, 30], [14, 38]],
            ),
            (
                "2r3k1/1b2b2p/r2p1pp1/pN1Pn3/1pPB2P1/1P5P/P3R1B1/5RK1 w - -",
                vec![[30, 38], [5, 3], [5, 13], [23, 31]],
            ),
            (
                "2r3k1/5pp1/1pq4p/p7/P1nR4/2P2P2/Q5PP/4B1K1 b - -",
                vec![[41, 33], [26, 43], [26, 20], [26, 36]],
            ),
            (
                "6k1/6pp/4r3/p1qpp3/Pp6/1n1P1B1P/1B2Q1P1/3R1K2 w - -",
                vec![[19, 27], [9, 36], [12, 13], [3, 4]],
            ),
            (
                "r2qkb1r/1b1n1ppp/p3pn2/1pp5/3PP3/2NB1N2/PP3PPP/R1BQ1RK1 w kq -",
                vec![[27, 35], [2, 20], [8, 16], [28, 36]],
            ),
            (
                "r3r1k1/pn1bnpp1/1p2p2p/1q1pPP2/1BpP3N/2P2BP1/2P3QP/R4RK1 w - -",
                vec![[37, 45], [25, 52], [0, 1], [22, 30]],
            ),
            (
                "2r5/p3kpp1/1pn1p2p/8/1PP2P2/PB1R1KP1/7P/8 b - -",
                vec![[48, 32], [48, 40], [44, 36], [53, 37]],
            ),
            (
                "2rq1rk1/1b2bppp/p2p1n2/1p1Pp3/1Pn1P3/5N1P/P1B2PP1/RNBQR1K1 w - -",
                vec![[8, 24], [10, 17], [1, 11], [1, 18]],
            ),
            (
                "2rqr1k1/1b2bp1p/ppn1p1pB/3n4/3P3P/P1NQ1N2/1PB2PP1/3RR1K1 w - -",
                vec![[31, 39], [47, 2], [18, 35], [3, 1]],
            ),
            (
                "3Rb3/5ppk/2r1r3/p5Pp/1pN2P1P/1P5q/P4Q2/K2R4 b - -",
                vec![[32, 24], [42, 50], [44, 52], [42, 26]],
            ),
            (
                "3Rbrk1/4Q2p/6q1/pp3p2/4p2P/1P4P1/8/5R1K w - -",
                vec![[22, 30], [7, 15], [59, 58], [5, 13]],
            ),
            (
                "3bn3/3r1p1k/3Pp1p1/1q6/Np2BP1P/3R2PK/8/3Q4 w - -",
                vec![[31, 39], [28, 21], [19, 27], [22, 30]],
            ),
            (
                "3k1r1r/p2n1p1p/q2p2pQ/1p2P3/2pP4/P4N2/5PPP/2R1R1K1 w - -",
                vec![[16, 24], [21, 38], [47, 31], [36, 43]],
            ),
            (
                "3r1bk1/1p2qp1p/p5p1/P1pPp3/2QnP3/3BB3/1P3PPP/2R3K1 w - -",
                vec![[13, 29], [2, 1], [2, 5], [15, 23]],
            ),
            (
                "3r1bkr/2q3pp/1p1Npp2/pPn1P3/5B2/1P6/2P2PPP/R2QR1K1 w - -",
                vec![[17, 25], [29, 11], [3, 21], [36, 45]],
            ),
            (
                "3r2k1/p2q1pp1/1p2n1p1/2p1P2n/P4P2/2B1Q1P1/7P/1R3BK1 w - -",
                vec![[24, 32], [5, 33], [20, 28], [1, 0]],
            ),
            (
                "3r4/8/pq3kr1/3Bp3/7p/1P3P2/P5PP/3RQ2K b - -",
                vec![[31, 23], [45, 54], [59, 51], [46, 47]],
            ),
            (
                "3r4/pk1p3p/1p2pp2/1N6/2P1KP2/6P1/3R3P/8 w - -",
                vec![[29, 37], [28, 27], [28, 20], [33, 18]],
            ),
            (
                "4k2r/1b2b3/p3pp1p/1p1p4/3BnpP1/P1P4R/1KP4P/5BR1 w k -",
                vec![[30, 38], [5, 12], [16, 24], [18, 26]],
            ),
            (
                "4k3/r2bbprp/3p1p1N/2qBpP2/ppP1P1P1/1P1R3P/P7/1KR1Q3 w - -",
                vec![[8, 16], [4, 11], [2, 10], [23, 31]],
            ),
            (
                "4q1k1/pb5p/Nbp1p1r1/3r1p2/PP1Pp1pP/4P1P1/1BR1QP2/2R3K1 w - -",
                vec![[25, 33], [9, 0], [6, 14], [24, 32]],
            ),
            (
                "4r1k1/1pb3qp/p1b1r1p1/P1Pp4/3P1p2/2BB4/1R1Q1PPP/1R4K1 b - -",
                vec![[29, 21], [54, 51], [54, 52], [54, 45]],
            ),
            (
                "4r1k1/5p1p/p2q2p1/3p4/3Qn3/2P1RN2/Pr3PPP/R5K1 w - -",
                vec![[18, 26], [20, 4], [8, 16], [14, 22]],
            ),
            (
                "4rr1k/pp1n2bp/7n/1Pp1pp1q/2Pp3N/1N1P1PP1/P5QP/2B1RR1K b - -",
                vec![[37, 29], [47, 53], [61, 53], [49, 41]],
            ),
            (
                "4rrk1/p6p/2q2pp1/1p6/2pP1BQP/5N2/P4PP1/2R3K1 w - -",
                vec![[31, 39], [29, 11], [30, 22], [8, 24]],
            ),
            (
                "5nk1/1bp1rnp1/pp1p4/4p1P1/2PPP3/NBP5/P2B4/4R1K1 w - -",
                vec![[26, 34], [6, 5], [6, 13], [27, 35]],
            ),
            (
                "5r2/1p1k4/2bp4/r3pp1p/PRP4P/2P2PP1/2B2K2/7R b - -",
                vec![[37, 29], [51, 50], [32, 56], [61, 53]],
            ),
            (
                "5r2/5p1Q/4pkp1/p7/1pb2q1P/5P2/P4RP1/3R2K1 w - -",
                vec![[31, 39], [13, 9], [3, 51], [3, 11]],
            ),
            (
                "5rk1/1Q3pp1/p2p3p/4p1b1/N3PqP1/1N1K4/PP6/3R4 b - -",
                vec![[43, 35], [29, 21], [29, 30], [47, 39]],
            ),
            (
                "7r/3nkpp1/4p3/p1pbP3/1r3P1p/1P2B2P/P2RBKP1/7R b - -",
                vec![[32, 24], [63, 58], [63, 59], [63, 57]],
            ),
            (
                "8/1r1rq2k/2p3p1/3b1p1p/4p2P/1N1nP1P1/2Q2PK1/RR3B2 b - -",
                vec![[37, 29], [49, 25], [42, 34], [46, 38]],
            ),
            (
                "8/1r2k3/4p2p/R3K2P/1p1P1P2/1P6/8/8 w - -",
                vec![[29, 37], [36, 28], [32, 34], [27, 35]],
            ),
            (
                "8/3r1pp1/p7/2k2PpP/rp1pB3/2pK1P2/P1R5/1R6 w - -",
                vec![[37, 45], [1, 6], [1, 7], [21, 29]],
            ),
            (
                "8/6k1/3P1bp1/2B1p3/1P6/1Q3P1q/7r/1K2R3 b - -",
                vec![[36, 28], [23, 58], [23, 37], [46, 38]],
            ),
            (
                "b2rrbk1/2q2p1p/pn1p2p1/1p4P1/2nNPB1P/P1N3Q1/1PP3B1/1K1RR3 w - -",
                vec![[31, 39], [29, 2], [18, 8], [22, 23]],
            ),
            (
                "b7/2pr1kp1/1p3p2/p2p3p/P1nP1N2/4P1P1/P1R2P1P/2R3K1 w - -",
                vec![[20, 28], [10, 18], [10, 12], [13, 21]],
            ),
            (
                "k1qbr1n1/1p4p1/p1p1p1Np/2P2p1P/3P4/R7/PP2Q1P1/1K1R4 w - -",
                vec![[27, 35], [16, 24], [3, 19], [14, 30]],
            ),
            (
                "r1b1rnk1/pp3pq1/2p3p1/6P1/2B2P1R/2P5/PP1Q2P1/2K4R w - -",
                vec![[29, 37], [26, 19], [31, 63], [14, 30]],
            ),
            (
                "r1bq1rk1/pp3pbp/3Pp1p1/2p5/4PP2/2P5/P2QB1PP/1RB1K2R b K -",
                vec![[44, 36], [58, 51], [59, 31], [49, 41]],
            ),
            (
                "r1bqr2k/pppn2bp/4n3/2P1p1p1/1P2Pp2/5NPB/PBQN1P1P/R4RK1 w - -",
                vec![[34, 42], [11, 17], [11, 26], [0, 2]],
            ),
            (
                "r1br1k2/1pq2pb1/1np1p1pp/2N1N3/p2P1P1P/P3P1R1/1PQ3P1/1BR3K1 w - -",
                vec![[31, 39], [1, 8], [2, 4], [2, 5]],
            ),
            (
                "r1n2k1r/5pp1/2R5/pB2pPq1/P2pP3/6Pp/1P2Q2P/5RK1 w - -",
                vec![[37, 45], [12, 19], [42, 34], [42, 46]],
            ),
            (
                "r1r2bk1/pp1n1p1p/2pqb1p1/3p4/1P1P4/1QN1PN2/P3BPPP/2RR2K1 w - -",
                vec![[25, 33], [17, 10], [2, 1], [8, 16]],
            ),
            (
                "r2q1r2/pp1b2kp/2n1p1p1/3p4/3P1P1P/2PB1N2/6P1/R3QRK1 w - -",
                vec![[31, 39], [4, 20], [0, 1], [14, 22]],
            ),
            (
                "r2q1rk1/pp2b1pp/1np1b3/4pp2/1P6/P1NP1BP1/2Q1PP1P/1RB2RK1 w - -",
                vec![[25, 33], [2, 20], [21, 14], [5, 4]],
            ),
            (
                "r2q4/6k1/r1p3p1/np1p1p2/3P4/4P1P1/R2QBPK1/7R w - -",
                vec![[20, 28], [11, 9], [11, 18], [8, 10]],
            ),
            (
                "r2qr1k1/pp3pbp/5np1/2p2b2/8/2PP1Q2/PPB3PP/RNB2RK1 b - -",
                vec![[34, 26], [37, 30], [45, 30], [59, 51]],
            ),
            (
                "r3k2r/1bq1bpp1/p4n2/2p1pP2/2NpP2p/3B4/PPP3PP/R1B1QR1K b k -",
                vec![[31, 23], [49, 42], [52, 59], [60, 61]],
            ),
            (
                "r3k2r/2q2p2/p2bpPpp/1b1p4/1p1B1PPP/8/PPPQ4/1K1R1B1R w kq -",
                vec![[29, 37], [27, 20], [5, 33], [31, 39]],
            ),
            (
                "r3k2r/ppq2p1p/2n1p1p1/3pP3/5PP1/2P1Q3/PP2N2P/3R1RK1 b k -",
                vec![[55, 39], [60, 62], [50, 41], [56, 58], [56, 58]],
            ),
            (
                "r3r1k1/1pp1np1p/1b1p1p2/pP2p3/2PP2b1/P3PN2/1B3PPP/R3KB1R w KQ -",
                vec![[26, 34], [5, 12], [0, 3], [27, 36]],
            ),
            (
                "r3r1k1/1pq2pbp/p1ppbnp1/4n3/2P1PB2/1NN2P2/PP1Q2PP/R3RBK1 w - -",
                vec![[26, 34], [29, 36], [18, 3], [4, 3]],
            ),
            (
                "r3r1k1/bpp1np1p/3p1p2/pPP1p3/3P2b1/P3PN2/1B3PPP/R3KB1R w KQ -",
                vec![[33, 41], [0, 2], [0, 3], [15, 23]],
            ),
            (
                "r3r1k1/pp2q3/2b1pp2/6pN/Pn1P4/6R1/1P3PP1/3QRBK1 w - -",
                vec![[13, 29], [3, 11], [9, 17], [13, 21]],
            ),
            (
                "r4r2/1p2pbk1/1np1qppp/p7/3PP2P/P1Q2NP1/1P3PB1/2R1R1K1 w - -",
                vec![[31, 39], [18, 34], [18, 20], [9, 25]],
            ),
            (
                "r4r2/2p2kb1/1p1p2p1/qPnPp2n/2B1PP2/pP6/P1Q1N2R/1KB4R w - -",
                vec![[29, 37], [2, 11], [7, 6], [15, 14]],
            ),
            (
                "r4rk1/2p5/p2p1n2/1p1P3p/2P1p1pP/1P4B1/1P3PP1/3RR1K1 w - -",
                vec![[26, 34], [3, 2], [3, 11], [4, 12]],
            ),
            (
                "r4rk1/2qnb1pp/4p3/ppPb1p2/3Pp3/1PB3P1/R1QNPPBP/R5K1 b - -",
                vec![[32, 24], [52, 45], [50, 42], [44, 36]],
            ),
            (
                "r4rk1/p5pp/1p2b3/2Pn1p2/P2Pp2P/4P1Pq/2Q1BP2/R1BR2K1 w - -",
                vec![[24, 32], [12, 26], [12, 5], [0, 1]],
            ),
            (
                "r4rk1/pbq2p2/2p2np1/1p2b2p/4P3/2N1BPP1/PPQ1B2P/R2R2K1 b - -",
                vec![[39, 31], [61, 59], [48, 32], [48, 40]],
            ),
            (
                "r4rk1/pp1b2b1/n2p1nq1/2pP1p1p/2P1pP2/PP4PP/1BQ1N1B1/R3RNK1 b - -",
                vec![[39, 31], [56, 57], [56, 60], [61, 53]],
            ),
            (
                "rn3rk1/p1p1qp2/1pbppn1p/6p1/P1PP4/2PBP1B1/3N1P1P/R2QK1R1 w Q -",
                vec![[15, 31], [3, 12], [24, 32], [13, 29]],
            ),
            (
                "rnbq1rk1/2p1p1bp/p3pnp1/1p6/3P4/1QN1BN2/PP3PPP/R3KB1R w KQ -",
                vec![[8, 24], [0, 2], [14, 22], [15, 23]],
            ),
            (
                "rr3n1k/q3bpn1/2p1p1p1/2PpP2p/pP1P1N1P/2BB1NP1/P2Q1P2/6RK w - -",
                vec![[22, 30], [7, 15], [11, 2], [8, 16]],
            ),
            (
                "1b1r4/3rkp2/p3p2p/4q3/P5P1/2RBP3/P1Q4P/1R3K2 b - -",
                vec![[57, 48], [36, 45], [40, 32], [47, 39]],
            ),
            (
                "1bq3rk/R6p/5n1P/1N1p4/1PnP2p1/6P1/5B2/2Q2BK1 w - -",
                vec![[48, 52], [48, 8], [48, 54]],
            ),
            (
                "1k1r3r/1p1b1Q1p/p7/q3p3/4p3/2P1N3/P4PPP/R4RK1 w - -",
                vec![[0, 3], [53, 45], [5, 3], [8, 24]],
            ),
            (
                "1Q6/1b4pk/2q2b1p/1p1ppP2/1Pp5/2P2P1P/2BB2P1/6K1 b - -",
                vec![[42, 40], [49, 58], [42, 51]],
            ),
            (
                "1qrr3k/1p2bp1p/1n2p1pP/p2pP3/P4B2/1PPB2P1/2R1QP2/3R2K1 w - -",
                vec![[19, 33], [12, 4], [12, 20]],
            ),
            (
                "1r1n1rk1/3qp2p/P2p2p1/1p6/5pP1/1p3P1P/5PB1/R1QR2K1 w - -",
                vec![[14, 5], [2, 9], [2, 18], [2, 11]],
            ),
            (
                "1r1n2k1/5r1p/P2qp1p1/3p4/1p3pP1/1Q3P1P/R4P2/2R2BK1 w - -",
                vec![[8, 10], [6, 14], [17, 24]],
            ),
            (
                "1r1q1nk1/p3bpp1/b1p1p2p/4P3/1P2NB2/P4N2/5PPP/2Q1R1K1 w - -",
                vec![[29, 20], [28, 43], [4, 3], [15, 23]],
            ),
            (
                "1r1q4/5p2/2p4k/1n2p1pP/4P3/P4BR1/3Q1PKP/8 w - -",
                vec![[11, 2], [11, 8], [11, 20]],
            ),
            (
                "1r1qbr1k/4bp1p/p3p2Q/3pP3/2pP4/P1N1PN2/1PR2RP1/6K1 b - -",
                vec![[61, 62], [60, 42], [60, 51]],
            ),
            (
                "1r3k2/8/R7/4pPP1/P1p5/1nP5/R5P1/3rB1K1 w - -",
                vec![[40, 47], [6, 5], [40, 45]],
            ),
            (
                "1r3rk1/2b3pp/p4p2/2p5/P1Np4/1P1R2P1/1P3P1P/2R3K1 b - -",
                vec![[61, 60], [61, 58], [61, 59]],
            ),
            (
                "1r5k/p7/3pQ1p1/1Np5/2P3P1/7P/1PK5/7q b - -",
                vec![[7, 5], [7, 14], [7, 15]],
            ),
            (
                "1r6/4q3/2p2p1k/4p1pP/P2nP2P/5BR1/5PK1/2Q5 w - -",
                vec![[21, 30], [21, 3], [24, 32]],
            ),
            (
                "1r6/7k/1p4r1/1P2p3/2P1p2P/2RbB3/6P1/2R4K w - -",
                vec![[2, 0], [7, 6], [7, 15]],
            ),
            (
                "1r6/8/1r5p/p2pk3/4p3/2P4P/RP2B1n1/1RK5 b - -",
                vec![[41, 45], [57, 61], [41, 46]],
            ),
            (
                "1rbr2k1/pp3pp1/q6p/4P3/3p1P2/6P1/P2QP1BP/R1R3K1 w - -",
                vec![[14, 28], [14, 21], [11, 19]],
            ),
            (
                "2b1r1k1/2r5/p3pbp1/5p2/PN5P/1Pp2BP1/4PP2/2R2RK1 w - -",
                vec![[5, 3]],
            ),
            (
                "2r1k3/p5pp/1p2p3/7P/P2B2Q1/1bP5/R4PPK/4q3 w - -",
                vec![[8, 12], [30, 54], [8, 9]],
            ),
            (
                "2r1q1k1/2p2rbp/p2p2p1/Q2P4/b3P3/3p2P1/3B1PBP/2R1R1K1 w - -",
                vec![[14, 23], [14, 5], [28, 36]],
            ),
            (
                "2r1q2k/7p/p1np1P1P/8/1pP2R2/8/PP1Q4/R1KN2r1 b - -",
                vec![[60, 46], [42, 36], [60, 36]],
            ),
            (
                "2r1q3/p1p1nk1p/4p2p/2R1Pp2/1P1Q4/P3N1P1/5P2/6K1 w - -",
                vec![[27, 31], [34, 2], [34, 10], [34, 26]],
            ),
            (
                "2r2k2/R4bpp/2p2p2/1pN5/1n3PP1/1P2P2P/8/5BK1 w - -",
                vec![[5, 14], [20, 28]],
            ),
            (
                "2r2rk1/1p1nbppp/p2p4/B2Pp1PP/5P2/1P6/P1P5/1K1R1B1R w - -",
                vec![[5, 23], [32, 11], [3, 4]],
            ),
            (
                "2r2rk1/4q1pp/p7/1pb1Pp2/5P2/1P1QB3/b3N1PP/2R1K2R b K -",
                vec![[61, 59], [34, 20], [62, 63]],
            ),
            (
                "2r3k1/1b1n1npp/1pq2p2/8/1P1QP3/6P1/1B1NBP1P/3R2K1 w - -",
                vec![[3, 2], [12, 30], [13, 21], [13, 29]],
            ),
            (
                "2r3n1/p1p3kp/1q2p2p/4Pp2/1P4N1/P5P1/1Q3P2/2R3K1 w - -",
                vec![[9, 11], [30, 20], [30, 45]],
            ),
            (
                "2rq2k1/1p3pb1/1n4pp/pP2p3/P1b1P3/2N4P/2B1NPP1/R1Q3K1 b - -",
                vec![[54, 61], [54, 45], [59, 43]],
            ),
            (
                "2rq2k1/4rpp1/p3p3/P2n2pP/2pPR3/2P3B1/2Q2PP1/R5K1 b - -",
                vec![[52, 49], [52, 48], [52, 51]],
            ),
            (
                "2rr1b2/3q3k/p4n1p/1p1p2p1/2nNp3/P1N2PQ1/1PP3PP/R1BR2K1 b - -",
                vec![[61, 43], [61, 34], [61, 54]],
            ),
            (
                "3b2k1/1p1b4/4p2B/1n1p1p2/pN6/P2P4/1PR3PP/7K w - -",
                vec![[47, 20], [47, 29], [10, 12]],
            ),
            (
                "3q2k1/5rpp/r1pPp3/1bQn4/p2B4/4P1P1/1P1R2BP/R5K1 w - -",
                vec![[14, 23], [0, 2], [11, 10]],
            ),
            (
                "3Q4/5pk1/4p1p1/3pb2p/P3q2P/1r4P1/2R2P1K/2R5 b - -",
                vec![[36, 29], [17, 16], [17, 19]],
            ),
            (
                "3q4/7k/3ppp2/p3n2B/P1r1P2P/8/1PPQ4/1K3R2 w - -",
                vec![[5, 6], [11, 20], [11, 14]],
            ),
            (
                "3r1k2/R4bpp/2p2p2/1pN5/1n3PP1/1P2P2P/6B1/6K1 w - -",
                vec![[14, 28], [14, 21], [30, 38]],
            ),
            (
                "3r1r2/p5k1/1pnpP2p/6p1/2P3P1/4P3/P1B1K1P1/3R3R w - -",
                vec![[10, 24], [3, 5], [7, 5]],
            ),
            (
                "3r2k1/4q1pp/p7/1p2Pp2/5P2/1P2Q3/b5PP/2N1K2R b K -",
                vec![[8, 1]],
            ),
            (
                "3r2k1/pp1q1ppp/4rnn1/3p4/P1pP4/2P1P2P/1BQN1PP1/RR4K1 w - -",
                vec![[9, 16], [10, 3], [10, 37]],
            ),
            (
                "3r2k1/R7/1p3np1/1P1b4/1p6/5PqP/2Q3P1/3R2K1 b - -",
                vec![[59, 60], [59, 61]],
            ),
            (
                "3r3k/1p2R2p/pP4p1/8/2rp1p2/5P2/2P3PP/4R1K1 w - -",
                vec![[52, 50]],
            ),
            (
                "3r4/3pkpp1/4p3/2p3q1/6r1/1P6/P5B1/2R1RQK1 b - -",
                vec![[59, 63], [59, 56], [53, 37]],
            ),
            (
                "3r4/3q2bk/3rp1p1/1p5p/4QP2/R1Pp2P1/P2B1R1P/6K1 b - -",
                vec![[59, 58], [43, 42], [59, 60]],
            ),
            (
                "3r4/p2rppk1/2R3p1/4q3/3b4/PP4P1/2QRNP2/5K2 b - -",
                vec![[36, 39], [27, 41], [36, 35]],
            ),
            (
                "3rr1k1/1p2bppp/p1p5/P2P3q/2B1PPb1/1P1R4/3B1QP1/4R1K1 w - -",
                vec![[11, 18], [17, 25], [29, 37]],
            ),
            (
                "3rr1k1/pp2q1bp/n1p1P1p1/5p2/P2N1Pb1/1BP5/6PP/R1B1QRK1 w - -",
                vec![[2, 16], [0, 8], [15, 23]],
            ),
            (
                "4b2k/1q2bprp/pr6/3pP3/2pP4/P4N2/1P3RPQ/3N1RK1 b - -",
                vec![[60, 51], [52, 59], [54, 62]],
            ),
            (
                "4k2r/p2n3p/4q1pQ/1p2b2P/8/P1P2R2/1P4P1/K4R2 w - -",
                vec![[5, 3], [5, 4], [21, 20]],
            ),
            (
                "4qrk1/pb4p1/2p1p2p/PpP1Pr2/1P2QP1R/2B3P1/6R1/6K1 w - -",
                vec![[14, 11], [18, 4], [14, 8]],
            ),
            (
                "4r1k1/3q1ppb/2p5/2Pp1P1p/p2P3P/P3P1PB/5Q1K/4R3 w - -",
                vec![[4, 1], [13, 29], [4, 12], [4, 12]],
            ),
            (
                "4r1k1/pbq2rpp/1p2p3/4P2P/P1p4Q/1nP1B3/R1B2PP1/1R4K1 w - -",
                vec![[10, 46], [31, 38], [1, 3], [24, 32]],
            ),
            (
                "4r1k1/R5p1/4b2p/1p2P3/7P/P1nP4/6PK/4R3 w - -",
                vec![[4, 2], [48, 40], [19, 27]],
            ),
            (
                "4r2k/1pp1n1pp/pb3r2/6Nq/P2P4/2PQ2P1/1R3RKP/2B5 w - -",
                vec![[9, 12], [13, 21], [13, 45], [24, 32], [15, 23], [15, 31]],
            ),
            (
                "4rrk1/1b3pp1/1q3b1p/p2p1P2/3N4/2P3N1/1P3QPP/1R3R1K b - -",
                vec![[49, 40]],
            ),
            (
                "5b2/1p1q3n/pB1p2k1/P4N1p/6pP/4R3/6P1/5RK1 w - -",
                vec![[41, 27]],
            ),
            (
                "5k2/ppQbbp1p/8/5p2/P7/1P6/4KPPP/q4B1R b - -",
                vec![[52, 25], [0, 8], [0, 9]],
            ),
            (
                "5r1k/6p1/p1n1pr1p/2P1p2q/P5RN/4Q2P/5PP1/2R4K w - -",
                vec![[2, 1], [20, 19], [2, 3]],
            ),
            (
                "5rk1/p3qpb1/1p5p/5Q2/P7/1B2P3/1p3PPP/5RK1 b - -",
                vec![[61, 59], [52, 25], [52, 36]],
            ),
            (
                "5rk1/p7/1n1q1p2/1Prp1pNp/8/5NPP/P2Q1PB1/5RK1 w - -",
                vec![[5, 4], [11, 20], [8, 24]],
            ),
            (
                "5rk1/pp4p1/5rnp/3p1b2/2p2P2/P1P3P1/2P1B2P/R1B1R1K1 w - -",
                vec![[12, 21], [12, 3], [2, 20]],
            ),
            (
                "5rrk/pR4p1/3p2q1/P6p/2Q1pn2/7P/1PP2PP1/R3N1K1 w - -",
                vec![[0, 16], [6, 15], [49, 17]],
            ),
            (
                "6k1/1b3p2/p2q2p1/P2Pp2p/1p2Pb1P/1Br5/Q4PP1/3RN1K1 b - -",
                vec![[49, 58]],
            ),
            (
                "6k1/1q3rpp/5n2/Rp2r3/4p3/1B5P/3BQPP1/6K1 w - -",
                vec![[11, 20], [11, 18], [14, 22]],
            ),
            (
                "6k1/1q5p/5np1/pb1pNp2/3P1B1P/1Q6/1P2PP2/6K1 w - -",
                vec![[17, 16], [17, 8], [17, 18]],
            ),
            (
                "6k1/2q4p/r3b1p1/2P1p3/r7/4QP2/p1RN2PP/R5K1 b - -",
                vec![[24, 25], [40, 32], [40, 56]],
            ),
            (
                "6k1/pb4p1/1p1b2q1/1P1p3p/3Pn3/P1r2N1P/4QPB1/2B1R1K1 b - -",
                vec![[49, 58], [43, 57], [18, 26]],
            ),
            (
                "6k1/q4p2/2b1p2p/4Pp2/pP3N2/Q4PP1/6KP/8 b - -",
                vec![[48, 27], [42, 33], [48, 57]],
            ),
            (
                "6r1/1R6/1bp1knp1/pp1n3p/4pP1P/1PN3B1/1P3PB1/5K2 w - -",
                vec![[14, 23], [18, 35], [18, 28]],
            ),
            (
                "6r1/4bbk1/p3p1p1/Pp1pPp2/3P1P2/2P2B2/3B2K1/1R6 b - -",
                vec![[62, 63], [52, 59], [53, 60], [46, 38]],
            ),
            (
                "6rk/1n4bp/p3R2r/1p1P1Pp1/1P6/P1pB2P1/5P2/2R3K1 b - -",
                vec![[62, 58]],
            ),
            (
                "8/2b1rk2/2b1p1p1/5n2/p1pP2Q1/2p3P1/P4P2/2B1RK2 w - -",
                vec![[2, 38], [2, 16], [2, 29]],
            ),
            (
                "8/3q1pk1/3p4/2pB2p1/P2n4/1P4BP/6P1/4R1K1 b - -",
                vec![[51, 37], [54, 47], [38, 30]],
            ),
            (
                "8/3r1k2/2p3p1/1pPb1p2/p7/P1B2P2/4BK2/3R4 w - -",
                vec![[3, 7], [12, 19], [3, 27]],
            ),
            (
                "8/5r1k/p3b1pp/4p1q1/P3Pn2/2B1N2P/3R1PPK/3Q4 w - -",
                vec![[3, 1], [18, 0], [18, 9]],
            ),
            (
                "8/5r2/1pp2p1R/p3n3/4Pp1p/2P2P1k/4K2P/R7 b - -",
                vec![[53, 54], [41, 33]],
            ),
            (
                "8/p1r3kp/r3p1p1/PRp1n1P1/7R/8/4K2P/7B b - -",
                vec![[40, 43], [36, 51], [36, 53]],
            ),
            (
                "8/p1rb3k/1p1b1p1p/3q1p2/1P1P4/P1n1PPP1/3N3P/R1R2Q1K b - -",
                vec![[51, 33], [55, 46], [55, 63], [47, 39]],
            ),
            (
                "b1r3k1/3n1pb1/p2q2p1/P2Pp2p/1p2P2P/1Rr1NN2/Q1B2PP1/3R2K1 b - -",
                vec![[54, 47]],
            ),
            (
                "b2r4/3qk3/1p1pp3/pN4b1/P3Pp1p/2P5/4BQPP/5R1K w - -",
                vec![[12, 30], [5, 3]],
            ),
            (
                "q1b2k2/5ppp/2n5/P1N1n3/8/2PpB1P1/3N1P1P/R4RK1 w - -",
                vec![[5, 1], [5, 4], [32, 40]],
            ),
            (
                "r1b2r1k/1p2qpb1/1np3pp/p7/3P4/PBN2N2/1PQ2PPP/3R1RK1 w - -",
                vec![[5, 4], [10, 2], [10, 11], [15, 23]],
            ),
            (
                "r1b2rk1/4bpp1/7p/1B1pP3/3B4/8/1PP3PP/2KR3R b - -",
                vec![[58, 30], [58, 37], [56, 32]],
            ),
            (
                "r1b2rk1/pp3pb1/1npN1qpp/2P5/1PBp4/P4NPP/5P2/2RQ1RK1 w - -",
                vec![[5, 4], [43, 58], [3, 19]],
            ),
            (
                "r1b3k1/4brp1/p6p/1p1p4/3B4/8/PPP3PP/2KR1B1R w - -",
                vec![[5, 19], [27, 20], [2, 1]],
            ),
            (
                "r1b3k1/pp2qrpp/1n6/1Bb1Pp2/3p1P2/7Q/PP1BN1PP/R3K2R b KQ -",
                vec![[58, 44], [48, 40]],
            ),
            (
                "r1b3kr/p3qpp1/1pn1p2p/3pP2P/7n/3B3Q/2P2PPN/1RB1R1K1 w - -",
                vec![[2, 16], [23, 22]],
            ),
            (
                "r1br2k1/p4p2/1qp4p/1p2P1p1/6n1/5NP1/PP2QN1P/R1B2BK1 b - -",
                vec![[58, 44], [58, 37], [33, 25]],
            ),
            (
                "r3r1k1/1b3ppp/2p2n2/1p2b1B1/4P1B1/P6P/1PQ1NPP1/6K1 w - -",
                vec![[30, 37], [30, 21], [38, 45], [13, 21]],
            ),
            (
                "r3r1k1/2q2pp1/1pp2np1/4p3/nP2P1P1/P3Q2P/3R1PB1/B1R3K1 w - -",
                vec![[14, 5], [14, 21], [11, 10]],
            ),
            (
                "r3r1k1/ppn2ppp/2p5/5P2/P7/2N4P/1PP5/R1B2RK1 w - -",
                vec![[2, 29], [6, 14], [5, 3], [37, 45]],
            ),
            (
                "r3r2k/2R4p/3B2b1/p7/1p2p3/6PP/PPP4K/4R3 w - -",
                vec![[43, 34], [10, 18], [10, 26]],
            ),
            (
                "r4qk1/1p3rbp/3p1np1/2pPp3/b1P5/P1NBBPP1/3Q1R1P/4R1K1 w - -",
                vec![[4, 1], [6, 14], [11, 2], [4, 12]],
            ),
            (
                "r4rk1/1p2pp1p/3p1np1/q2P1P2/2P3BQ/pP5R/P1R3PP/7K w - -",
                vec![[10, 12], [30, 12], [37, 46]],
            ),
            (
                "r4rk1/1p2ppbp/1q1p2p1/p1nP1P2/2P3B1/5R2/PPRB2PP/1Q5K w - -",
                vec![[1, 4], [11, 38], [21, 23]],
            ),
            (
                "r4rk1/5pb1/2p3pp/pp2n3/1n3B2/1PN1PP1P/1P2BPK1/2R3R1 w - -",
                vec![[6, 3]],
            ),
            (
                "r4rk1/6b1/2p1N1p1/8/p2p3R/8/R4P1P/5K2 b - -",
                vec![[61, 57], [61, 45], [42, 34]],
            ),
            (
                "r4rk1/ppp1qp1p/1n4p1/4P3/6Q1/1N6/PP3PPP/2R1R1K1 b - -",
                vec![[56, 59], [41, 35], [61, 59]],
            ),
            (
                "r5k1/p1q2pbp/Ppp1bnp1/4p1B1/Q1P1P3/2N4P/1P2BPP1/5RK1 w - -",
                vec![[5, 3], [38, 20], [5, 2], [9, 25]],
            ),
            (
                "r5k1/p4nbp/2qN2p1/2P2p2/3p1B2/6P1/P2Q1P1P/3R2K1 w - -",
                vec![[11, 12], [43, 53], [11, 19]],
            ),
            (
                "r7/3b1pk1/6p1/3Pp2p/2P4P/p4B2/5PP1/2R3K1 b - -",
                vec![[51, 37], [54, 45], [54, 61]],
            ),
            (
                "rbbrqnk1/pp3pp1/2p2n1p/5N2/3P1P2/1BN4P/PPQB2P1/R4RK1 w - -",
                vec![[0, 4], [5, 4]],
            ),
            (
                "1k2r2r/1bq2p2/pn4p1/3pP3/pbpN1P1p/4QN1B/1P4PP/2RR3K b - -",
                vec![[41, 51], [25, 34], [49, 42], [25, 52]],
            ),
            (
                "1q2bn2/6pk/2p1pr1p/2Q2p1P/1PP5/5N2/5PP1/4RBK1 w - -",
                vec![[21, 36], [21, 27], [4, 0], [25, 33]],
            ),
            (
                "1r1q1rk1/1b1n1p1p/p2b1np1/3pN3/3P1P2/P1N5/3BB1PP/1R1Q1RK1 b - -",
                vec![[45, 28], [43, 16], [51, 41]],
            ),
            (
                "1r1r1bk1/1bq2p1p/pn2p1p1/2p1P3/5P2/P1NBB3/1P3QPP/R2R2K1 b - -",
                vec![[41, 35], [49, 56], [62, 54], [40, 32]],
            ),
            (
                "1r1r2k1/5pp1/p2p4/1p2pnqp/1BP1Q3/PP1R2P1/5P1P/3R2K1 b - -",
                vec![[37, 27], [38, 45], [33, 26], [39, 31]],
            ),
            (
                "1r1r4/R3pk2/4n1p1/2p2p2/8/4B3/Pn2BPPP/5RK1 b - -",
                vec![[44, 27], [9, 19], [34, 26], [37, 29]],
            ),
            (
                "1r2k2r/pp2ppb1/2n2np1/7p/4P3/P3BB1P/1P1N1PP1/R2R2K1 b k -",
                vec![[45, 51], [54, 47], [48, 40]],
            ),
            (
                "1r2qrk1/3bn3/pp1p3p/n1p1p1p1/P1P5/B1PP1NPP/2Q2PB1/1R2R1K1 w - -",
                vec![[21, 11], [16, 2], [10, 12], [1, 9]],
            ),
            (
                "1r2r2k/2b2q1p/p4p2/3Pn2P/3N1N2/1P2R3/4Q3/1K1R4 w - -",
                vec![[29, 44], [1, 0], [27, 37], [12, 40], [20, 18], [39, 47]],
            ),
            (
                "1r3rk1/8/3p3p/p1qP2p1/R1b1P3/2Np1P2/1P1Q1RP1/6K1 w - -",
                vec![[18, 3], [18, 8]],
            ),
            (
                "1r6/1q2b1k1/pn1pb3/B1p1p1pp/2P1Pp2/NP3P1P/1R2Q1PN/6K1 b - -",
                vec![[41, 58], [44, 62], [54, 62], [41, 51]],
            ),
            (
                "1r6/2qnrpk1/2pp1np1/pp2P3/4P3/PBN2Q2/1PPR1PP1/3R2K1 b - -",
                vec![[51, 36], [45, 28], [52, 36]],
            ),
            (
                "1rr2qk1/3p1pp1/1pb2n1p/4p3/p1P1P2P/P1NQ1BP1/1P3PK1/2RR4 w - -",
                vec![[18, 33], [14, 15], [2, 10]],
            ),
            (
                "2b2rk1/1r1nbppp/4p3/1p2P3/p4P2/P1N1B3/BPP4P/R2R2K1 w - -",
                vec![[18, 28], [0, 2], [3, 11], [9, 25]],
            ),
            (
                "2b3n1/6kp/p1nB1pp1/8/1p2P1P1/4NP2/PP3K2/3B4 w - -",
                vec![[20, 35], [3, 24], [13, 22]],
            ),
            (
                "2b5/2p1r2k/1pP2q1p/p2Pp3/4R3/1PN1Q2P/P2KP3/8 w - -",
                vec![[18, 33], [11, 2], [28, 26], [23, 31]],
            ),
            (
                "2k3r1/1b2bp2/2p2n2/ppn1p1Bp/2p1P2P/P4B2/1P1RN1P1/4K2R b K -",
                vec![[34, 19], [58, 50], [34, 44]],
            ),
            (
                "2r2r1k/p3ppbp/qpnp2pn/5P2/2P1PP2/P1N1BB2/1PQ3RP/3R2K1 w - -",
                vec![[18, 33], [21, 12], [18, 35], [37, 46]],
            ),
            (
                "2r2rk1/4bpp1/p2pbn1p/Pp2p3/1Pq1P2N/2P4P/1BB2PP1/R2QR1K1 w - -",
                vec![[31, 37], [10, 1], [3, 12], [3, 21]],
            ),
            (
                "2r3k1/3q1pp1/ppr1p1np/4P3/P1nPQ3/5N1P/5PPK/RRB5 b - -",
                vec![[46, 52], [51, 50], [51, 59]],
            ),
            (
                "2r3k1/p1r1qpb1/1p2p1p1/nR2P3/P2B4/2P5/3NQPP1/R5K1 w - -",
                vec![[11, 28], [11, 17], [13, 29]],
            ),
            (
                "2rq1rk1/1p2b1p1/pn2p3/2p1Pn2/2pP3p/5N1P/PPQ2BP1/1BRR2K1 b - -",
                vec![[41, 35], [52, 38], [59, 50], [59, 60]],
            ),
            (
                "r7/p1r2nk1/1pNq1np1/1P1p1p2/P2Qp3/4P1P1/2R1P1BP/2R3K1 b - -",
                vec![[53, 38], [56, 58], [50, 49], [56, 60]],
            ),
            (
                "2rq2k1/3nbppp/pprp1nb1/4p3/P1P1P3/1PN1BN1P/2Q1BPP1/R2R2K1 w - -",
                vec![[21, 31], [0, 8], [0, 1], [14, 22]],
            ),
            (
                "2rqr1k1/1p2bppp/p2p1n2/3P1P2/2Pp4/1P1B4/P3Q1PP/R1B2RK1 b - -",
                vec![[45, 51], [59, 32], [55, 47]],
            ),
            (
                "2rr4/Bp3k1p/5pp1/8/2n3b1/P1N5/1PP2PPP/R1R3K1 w - -",
                vec![[18, 28], [18, 24], [13, 21], [15, 23]],
            ),
            (
                "3R4/5pk1/2p4r/1p2p1p1/p3P1P1/P1P2P2/1P2B1K1/n7 b - -",
                vec![[0, 17], [47, 45], [47, 46]],
            ),
            (
                "3q3r/2p2pk1/6p1/2p1p1Pn/1pBnP3/1P2BP1R/P5Q1/7K b - -",
                vec![[39, 29], [59, 51], [50, 42]],
            ),
            (
                "3r1bk1/1rq2p2/2npb1p1/p3p2p/2P1P2P/1PN3P1/2N1QPBK/R2R4 w - -",
                vec![[18, 33], [3, 1]],
            ),
            (
                "3r1rk1/1p4bp/2qPp1p1/p3n3/P2BN3/1PN4P/2PR2P1/4Q1K1 w - -",
                vec![[18, 33], [4, 20], [4, 31]],
            ),
            (
                "3r2k1/2q2ppp/p1p1bn2/1p2b1B1/4P3/1PN2B1P/P1Q2PP1/2R4K w - -",
                vec![[18, 35]],
            ),
            (
                "3r2k1/4qpn1/R2p3p/1Pp1p1p1/1rP1P1P1/6P1/3Q1P2/4RBK1 b - -",
                vec![[54, 44], [62, 61], [62, 55], [59, 51]],
            ),
            (
                "3r4/2p2pk1/2q1n1p1/2p1p1Pn/1pB1P3/1P2BP2/P6R/4Q1K1 b - -",
                vec![[44, 27], [54, 62], [39, 29]],
            ),
            (
                "3r4/bp1r2pk/p3npqp/P2Np3/1PR1P2B/5Q1P/2P3P1/5R1K b - -",
                vec![[44, 27], [48, 57], [55, 63]],
            ),
            (
                "3r4/r1pb3p/1p4kB/2p3P1/4pP2/1P2NnKP/PR3R2/8 b - -",
                vec![[21, 27], [51, 58], [51, 37], [34, 26]],
            ),
            (
                "3rb1k1/4qpbp/1p2p1p1/1P3n2/Q1P2p2/2N2B1P/6PK/1NR2R2 b - -",
                vec![[37, 20], [54, 27], [59, 57], [59, 58]],
            ),
            (
                "3rr1k1/1p3ppp/p1q2b2/P4P2/2P1p3/1P6/2N1Q1PP/4RR1K w - -",
                vec![[10, 25], [10, 20], [12, 39], [17, 25]],
            ),
            (
                "3rr1k1/pb1n1pp1/2q2b1p/2p5/2P1p2N/1P2B1P1/P1QR1PBP/3R2K1 b - -",
                vec![[51, 36], [45, 31], [51, 61], [54, 46]],
            ),
            (
                "3rr3/p3b1kp/2p2pp1/1q1np3/4Q1PB/1NP5/PP3P1P/R2R2K1 b - -",
                vec![[35, 29], [54, 53], [33, 40], [48, 32]],
            ),
            (
                "4b1nk/p1r1p1rp/Bpq5/n3Ppp1/8/5N1P/2P2BPQ/R3R1K1 w - -",
                vec![[21, 27], [40, 19], [0, 3], [36, 44]],
            ),
            (
                "4k2r/1r2np2/2q1p1p1/p2pP3/n1pP1PP1/1pP1NNK1/1P2Q3/R4R2 w k -",
                vec![[21, 38], [12, 11], [12, 14], [5, 13], [5, 7]],
            ),
            (
                "4n3/1p1b1pk1/2n5/rN4p1/1p1Np3/1B2P1P1/PP3PK1/2R5 b - -",
                vec![[42, 36]],
            ),
            (
                "4n3/1p1b1pk1/8/rNR1n1p1/1p1Np3/1B2P1P1/PP3PK1/8 b - -",
                vec![[36, 21], [51, 33], [53, 45]],
            ),
            (
                "4nk2/p4rr1/1pRp3b/1P1Pp2p/1P5P/2NBpP2/4R1P1/5K2 w - -",
                vec![[18, 28], [5, 6], [18, 3], [42, 58]],
            ),
            (
                "4r1k1/2pbqp1p/1r3p2/pP1p4/8/P1QBPN2/3P1PPP/4K2R w K -",
                vec![[21, 27], [18, 50], [16, 24]],
            ),
            (
                "4r1k1/3q1pp1/3p1n2/rp2nP1p/3B1R2/2N5/2PQ2PP/4R1K1 w - -",
                vec![[18, 28], [6, 7], [4, 1], [15, 23]],
            ),
            (
                "4r1k1/3r2p1/b1q2n1p/p7/Pp2P2Q/1N2BP2/1PP4P/2R1R2K w - -",
                vec![[17, 27], [17, 34], [17, 32]],
            ),
            (
                "4r3/1p3pk1/2p2n1p/2n1qP2/5bPQ/P3pB2/2R5/1R3N1K b - -",
                vec![[45, 28], [34, 19], [45, 55], [36, 27], [60, 59]],
            ),
            (
                "4rrk1/p4pp1/1b1p3p/2pP4/6q1/5N2/PP3PPP/2RQ1R1K w - -",
                vec![[21, 11], [7, 6], [3, 19], [15, 23]],
            ),
            (
                "5k2/6p1/Bnp1p2p/5p2/3P1n2/2q2P2/7P/5RQK b - -",
                vec![[41, 35], [61, 53], [61, 62], [54, 46]],
            ),
            (
                "5r1k/2p3q1/1p1npr2/pPn1N1pp/P1PN4/R4PPP/4Q1K1/3R4 w - -",
                vec![[27, 42], [14, 7], [3, 1], [3, 2]],
            ),
            (
                "5r2/2r2kp1/3n1p2/1p1Pp2p/p3P2P/PnP4R/1BB2KP1/4R3 b - -",
                vec![[43, 26], [53, 52], [61, 57]],
            ),
            (
                "5rk1/3nbp1p/2p1p3/2PpP1pN/3P2B1/q3PQ1P/6P1/5RK1 w - -",
                vec![[39, 45], [21, 13], [21, 22], [23, 31]],
            ),
            (
                "5rk1/6pp/pn1qp1r1/1p2R3/2pP1P2/P1P2Q1P/5P2/2B1R2K b - -",
                vec![[41, 35], [41, 51], [43, 51], [46, 45]],
            ),
            (
                "5rk1/pb1qbppp/1p6/n1rpP3/7P/2P2NP1/P3QPB1/R1BR2K1 w - -",
                vec![[21, 27], [2, 9], [2, 11]],
            ),
            (
                "6k1/3pbpp1/p3p2n/r3P2p/2rBNP2/2P3PP/P3R3/3R3K b - -",
                vec![[47, 37], [32, 35]],
            ),
            (
                "6k1/p2b3n/1p2pn1q/3r1p2/5P1B/P5NR/1Q2B1P1/4K3 b - -",
                vec![[45, 30], [47, 61]],
            ),
            (
                "7k/Rb4r1/3pPp1q/1pp2P2/3n2BP/4N1K1/1P3Q2/8 b - -",
                vec![[27, 42], [63, 62], [63, 55], [47, 20]],
            ),
            (
                "8/1b5p/1p1rrkp1/p2p1p2/P2P3P/1R1B1N2/nP3PPK/3R4 w - -",
                vec![[21, 36], [19, 33], [31, 39]],
            ),
            (
                "8/1p2kp2/p3pn2/4n1p1/1P2P3/P1r1NB1P/5PPK/R7 b - -",
                vec![[45, 60], [36, 21], [49, 33], [49, 41]],
            ),
            (
                "8/1q6/3pn1k1/2p1p1p1/2P1P1Pp/1rBP1p1P/2Q2P2/R5K1 b - -",
                vec![[44, 29], [46, 53], [46, 54]],
            ),
            (
                "8/2k2p2/pr1pbpn1/2p5/2P1P1P1/2P1KP1p/7P/R2N1B2 b - -",
                vec![[46, 36], [50, 51], [41, 17]],
            ),
            (
                "8/pB1b2k1/1p2pn2/5p2/5P1B/P7/3K1NPn/8 b - -",
                vec![[45, 60], [51, 33], [54, 46], [15, 5]],
            ),
            (
                "b1r2r1k/p2qp1bp/1p1pn1p1/8/1PP2P2/R1NBB3/P2Q2PP/5RK1 w - -",
                vec![[18, 33], [19, 12], [6, 7]],
            ),
            (
                "b4q2/1r5k/3p4/1p1Pn2B/1PpQP3/2N4P/6P1/B5K1 w - -",
                vec![[18, 3], [18, 12], [27, 20]],
            ),
            (
                "q3n3/2rp1ppk/2b4p/4p2P/p3P3/Pr2NBP1/1P1R1PK1/2R1Q3 w - -",
                vec![[20, 35], [14, 6], [4, 12]],
            ),
            (
                "r1b1k1nr/pp2p1bp/1q1p2p1/2pP4/2Pnp3/2NB1N1P/PP1B1PP1/R2QK2R w KQkq -",
                vec![[18, 28], [19, 28], [18, 24], [21, 27]],
            ),
            (
                "r1b2r1k/pp3pbp/3n4/4p3/2N4R/2N5/PP3PPP/R1B3K1 b - -",
                vec![[43, 37], [54, 45], [43, 26], [61, 59]],
            ),
            (
                "r1b2rk1/1pq1bppp/p2p4/P3p3/2N1n3/4B3/1PP1BPPP/R2QK2R w KQ -",
                vec![[26, 41], [4, 6], [13, 21]],
            ),
            (
                "r1b2rk1/p2q2b1/1nppp1pp/5pN1/P2P1B2/Q5P1/1P2PPBP/R1R3K1 b - -",
                vec![[41, 35], [58, 49], [47, 38]],
            ),
            (
                "r1b2rk1/pp3pp1/2np2qp/b1p1p3/2P1P3/2NPB1P1/PP3PBP/R2Q1RK1 w - -",
                vec![[18, 35], [3, 17], [3, 10], [13, 29]],
            ),
            (
                "r1bq1rk1/ppp3bp/n2p1pp1/3P4/2P1Pp2/2N2N1P/PP2BPP1/R2QR1K1 w - -",
                vec![[21, 27], [12, 5], [3, 11], [0, 2]],
            ),
            (
                "r1r3k1/pb3p1p/1pqBp1p1/4P3/3b4/2P2P2/PR1N2PP/2RQ3K w - -",
                vec![[11, 28]],
            ),
            (
                "r2b2k1/2pr4/1pn1b1qp/3Np1p1/p1P1p3/1P2B1P1/PQ2PP1P/R2R2NK b - -",
                vec![[42, 27], [44, 37], [24, 16], [47, 39]],
            ),
            (
                "r2br3/p2b1q1k/n2P2pp/1p1N1p2/4pP2/BP2Q1PN/P1R4P/3R2K1 w - -",
                vec![[35, 50], [23, 13], [20, 27], [20, 12]],
            ),
            (
                "r2q1bk1/1p1brpp1/p1np1n1p/4p3/PN2P3/1QPP1N1P/B2B1PP1/R3R1K1 w - -",
                vec![[25, 35], [11, 20], [17, 9]],
            ),
            (
                "r2q1k2/2p2pb1/p2n2rp/1p1RB1p1/8/2PQRN1P/P4PP1/6K1 w - -",
                vec![[21, 27], [36, 54], [20, 12], [23, 31]],
            ),
            (
                "r2q1rk1/3nbpp1/p2p3p/8/1p1BP3/3B2Q1/PPP4P/2KR3R b - -",
                vec![[51, 36], [52, 45], [52, 38], [54, 46]],
            ),
            (
                "r2q1rk1/pp1bp1bp/5np1/2pP1p2/8/2N2NP1/PP2PPBP/2RQ1RK1 w - -",
                vec![[21, 36], [21, 38], [3, 17], [12, 20]],
            ),
            (
                "r2q2kr/p3n3/1p1Bp1bp/3pP1pN/5PPn/3B3Q/2P2K2/1R5R w - -",
                vec![[39, 45], [19, 46], [1, 6]],
            ),
            (
                "r2qk2r/1bpnnpbp/p2pp1p1/4P3/Pp1P1P2/2NBBN2/1PP3PP/R2Q1RK1 w kq -",
                vec![[18, 28], [18, 8], [18, 12], [36, 43]],
            ),
            (
                "r2qr1k1/pp3ppp/2n2nb1/1P4B1/3p4/P2B1P2/2P1N1PP/R2Q1RK1 b - -",
                vec![[42, 36], [42, 32], [42, 52]],
            ),
            (
                "r2r2k1/1bqn1pp1/1p1p1b1p/1B2n3/2P1P3/P1N1B3/1P1NQ1PP/4RR1K w - -",
                vec![[18, 35], [20, 27], [11, 17]],
            ),
            (
                "r2r2k1/1p1n2q1/2ppbp2/6p1/2PBP2p/p1N2P2/Pb4PP/1R1RQBK1 b - -",
                vec![[51, 36], [54, 53], [54, 55], [31, 23]],
            ),
            (
                "r2rq3/pp1b3k/n2P1bpp/4pp2/8/BPN1Q1PN/P4P1P/2RR2K1 w - -",
                vec![[18, 35], [16, 9], [20, 19], [20, 21], [13, 21]],
            ),
            (
                "r3r1k1/1b1nq2p/p2pNppQ/1ppP3n/P3P3/7P/1PB2PP1/R3RNK1 b - -",
                vec![[51, 61], [51, 36], [60, 58]],
            ),
            (
                "r3r1k1/1p1q2bp/1n1p2p1/1PpPpp1n/p3P3/R1NQBN1P/1PP2PP1/4R1K1 w - -",
                vec![[21, 38], [19, 5], [4, 3], [28, 37]],
            ),
            (
                "r3r1k1/3nbppp/p1q5/1pp1P2b/5B2/1P2QN2/1P1N1PPP/3RR1K1 w - -",
                vec![[11, 28], [29, 22], [3, 2]],
            ),
            (
                "r3r1k1/4bppp/pnq5/1pp1P2b/4NB2/1P2QN2/1P3PPP/3RR1K1 w - -",
                vec![[28, 43], [29, 22], [20, 2]],
            ),
            (
                "r3r2k/pp3pp1/1np4p/3p2q1/1P1P2b1/P1NBP3/2Q2PPP/R4RK1 w - -",
                vec![[18, 12], [6, 7]],
            ),
            (
                "r3r3/2P4k/3Bbbqp/ppQ2pp1/4pPP1/1P6/P1R2N1P/3R2K1 w - -",
                vec![[13, 28], [34, 20], [10, 11], [10, 12]],
            ),
            (
                "r3rnk1/1bpq1pp1/p2p3p/1p1Pp1b1/P3P1P1/1BP1N1P1/1P3P2/R1BQR1K1 w - -",
                vec![[20, 37], [17, 10], [6, 14], [6, 14], [3, 12]],
            ),
            (
                "r4rk1/1b1q1ppp/pb1p1nn1/1pp1p3/1PP1P3/P2PNN1P/B2B1PP1/2RQR1K1 w - -",
                vec![[20, 37], [8, 1], [3, 17]],
            ),
            (
                "r4rk1/1pq1bppp/1n2p3/p1n1P3/2PR4/2N1BN2/P3QPPP/1R4K1 w - -",
                vec![[18, 33], [12, 11], [15, 31]],
            ),
            (
                "r4rk1/5p1p/p2qpnp1/1p2b3/3p4/3B1R2/PPQ3PP/R1BN3K b - -",
                vec![[45, 51], [45, 35], [45, 39], [56, 58], [61, 58]],
            ),
            (
                "r4rk1/ppp3b1/3p1q1p/3Ppn2/P1P3n1/2NQ1N2/1P1B1PP1/R3R1K1 w - -",
                vec![[18, 28], [18, 33], [21, 15], [0, 16]],
            ),
            (
                "r5r1/1pp2k1p/2bn4/2p3B1/p3pPP1/1P2N2P/P1P1R3/R5K1 b - -",
                vec![[43, 33]],
            ),
            (
                "r6k/pp3pp1/1n6/1pQp1q2/3PrN1p/P3P2P/5PP1/2R2RK1 w - -",
                vec![[29, 19], [29, 12], [34, 33], [2, 4]],
            ),
            (
                "r6r/1q1bbkp1/p1p1pn2/5p1p/N2Bp3/2Q3P1/PPP2PBP/R2R2K1 b - -",
                vec![[45, 35], [56, 59], [39, 31]],
            ),
            (
                "rqr3k1/1p2bppp/3pn3/p3p1Pn/P3P3/1PNBBP2/1P1Q3P/2KR3R b - -",
                vec![[44, 27], [44, 29], [39, 29]],
            ),
            (
                "6k1/p2pp2p/bp4n1/q1r4R/1RP1P3/2P2B2/P2Q2P1/4K3 w - -",
                vec![[39, 35], [39, 37], [14, 30]],
            ),
            (
                "r2r2k1/pp3ppp/2p1qn2/5N1b/1n2PP2/4Q2P/PPP3B1/R1B2RK1 w - -",
                vec![[20, 34], [20, 22]],
            ),
            (
                "3r4/p4pk1/P1pr3p/3nb3/1p6/5B1P/1P3PP1/R1BR2K1 w - -",
                vec![[0, 32], [6, 5], [14, 22]],
            ),
            (
                "1b1r3r/3pkpp1/3np1q1/2p5/2P1PPP1/5Q2/PP4B1/R1BR2K1 b - -",
                vec![[63, 31], [43, 26], [59, 58]],
            ),
            (
                "7k/1p6/1n1rrq1p/1R1p1p1p/3P1P2/QR2P2P/6PK/5B2 w - -",
                vec![[16, 48], [15, 6], [33, 25]],
            ),
            (
                "5r1k/5rp1/p1n1p1qp/2P1p3/P7/4QN1P/5PP1/2R1R2K w - -",
                vec![[2, 26], [20, 12], [2, 18]],
            ),
            (
                "6r1/1p2Q1pk/2p2p1p/3p1P1P/p1nP4/PqP1P3/1P2RN2/2K5 b - -",
                vec![[17, 8], [17, 33], [49, 41]],
            ),
            (
                "6r1/1p2Q1pk/2p2p1p/n2p1P1P/p2P4/P1P1P3/1P1KR3/q2N4 b - -",
                vec![[0, 1], [55, 63], [32, 17]],
            ),
            (
                "5r1k/1b3pp1/p3pb2/4N2q/3R1B2/4P3/1PB2PPP/1K4R1 b - -",
                vec![[39, 12], [45, 36], [63, 62]],
            ),
            (
                "8/1b2r2p/1p1r1kp1/p2p1p2/Pn1P3P/1R1B1N2/1P3PPK/2R5 w - -",
                vec![[19, 33], [19, 5], [21, 36]],
            ),
            (
                "1q1k2r1/p2bn3/1r3n2/2QPp1Rp/2P4P/2PB1P2/2K1N3/R7 w - -",
                vec![[34, 32], [34, 16], [38, 62]],
            ),
            (
                "8/p3q1kp/1p1p1rp1/3Bn3/P1PQ1p2/1P6/6PP/4R2K b - -",
                vec![[45, 37], [54, 47], [45, 61]],
            ),
            (
                "4r1rk/pp6/5q2/3pNp1P/2pPnQ2/2P1P2P/P6K/R5R1 w - -",
                vec![[6, 46], [0, 1], [0, 2]],
            ),
            (
                "5r2/p1qn3k/bp1p1pp1/3P4/P2BPP2/2Pp4/3N2P1/R2Q2K1 w - -",
                vec![[3, 30], [3, 4], [24, 32]],
            ),
            (
                "1r3q2/1n4pk/R4p1p/4p2P/2B1P1P1/2P1QPK1/8/8 w - -",
                vec![[26, 44], [26, 35], [40, 41]],
            ),
            (
                "4r1k1/1pb3p1/p1p3q1/3n4/1P1P4/P3pRP1/4Q1K1/2B2R2 b - -",
                vec![[46, 28], [50, 41], [50, 59]],
            ),
            (
                "3R4/pkn5/1p2qp2/1p5p/1P2P1pR/P5P1/1N3PP1/5K2 b - -",
                vec![[44, 17], [44, 8]],
            ),
            (
                "4r1k1/p5pp/4q3/P2R4/2p2P2/2N3P1/2nB1K1P/1R6 b - -",
                vec![[44, 23], [48, 40], [55, 47]],
            ),
            (
                "1Q6/1p2p2k/1r4pp/p1p2P2/q2nr1P1/7R/1P1R4/5NK1 w - -",
                vec![[57, 61], [6, 15], [11, 14]],
            ),
            (
                "4R3/p2q2k1/2p2r1p/6p1/8/P5P1/4QP1P/3rB1K1 b - -",
                vec![[51, 27], [54, 53], [54, 46]],
            ),
            (
                "3q1r1k/5ppp/1pR1pn2/p7/B1PP4/P3QP1P/5PK1/8 w - -",
                vec![[20, 36], [21, 29], [14, 5], [14, 15]],
            ),
            (
                "8/Qpnbk2q/4pp2/1P1p4/P1r5/2P3P1/1K2N3/1R3B2 b - -",
                vec![[55, 19], [55, 15], [44, 36]],
            ),
            (
                "2r2rk1/pb2qp1p/n3p1p1/2pp4/N1P1n3/1P2P1P1/PN3PBP/2R1QRK1 w - -",
                vec![[4, 32], [4, 12], [2, 3]],
            ),
            (
                "r5k1/1p5p/p1p1qrnP/2B1pb2/R1P5/2P2B2/P2Q2P1/5RK1 w - -",
                vec![[11, 38], [11, 9], [24, 25]],
            ),
            (
                "Q7/2pq3k/1rp2b1p/2R5/8/1P1NP3/P2K1Pr1/5R2 w - -",
                vec![[56, 61], [56, 24], [34, 10]],
            ),
            (
                "q5k1/p2p2bp/1p1p2r1/2p1np2/6p1/1PP2PP1/P2PQ1KP/4R1NR b - -",
                vec![[56, 35], [46, 44], [48, 32], [34, 26]],
            ),
            (
                "4r1k1/1p5p/1nq1p1p1/4B3/6R1/P6P/3Q1PPK/8 w - -",
                vec![[11, 47], [36, 0], [11, 19]],
            ),
            (
                "r5nk/p4p1r/qp1Rb2p/2p1Pp2/5P1Q/P4N2/2B3PP/5RK1 w - -",
                vec![[31, 39], [16, 24]],
            ),
            (
                "r2r2k1/1q2bpp1/4p2p/p6Q/N4P2/1P2R1P1/2P4P/2KR4 b - -",
                vec![[49, 25], [49, 14], [59, 58]],
            ),
            (
                "r4rk1/1b2bpp1/1P1p4/p1q1p2p/R3PPn1/3B3Q/2P1N1PP/1R1N3K b - -",
                vec![[34, 13], [43, 35]],
            ),
            (
                "1r5r/1p6/p2p2p1/P2P1pkq/1R1Qp3/2P1P1bP/1P2R3/5BK1 b - -",
                vec![[39, 21], [22, 36], [63, 47]],
            ),
            (
                "2r1r2k/q6p/6p1/3R1p2/1p1P1B2/2bQPBP1/5P1P/6K1 w - -",
                vec![[19, 33], [6, 14], [35, 33]],
            ),
            (
                "1r3q1k/6p1/r6p/3P3b/2PQ1R2/pp4P1/5P2/R1B3K1 b - -",
                vec![[61, 25], [61, 43], [61, 52]],
            ),
            (
                "5k2/pb3pp1/qp3n1p/2p5/2P2B2/8/P1PN1PPP/R3R1K1 b - -",
                vec![[40, 24], [49, 42]],
            ),
            (
                "2r1qr1k/1p4bp/p1n1bpp1/4p3/B3P3/4QN1P/PP1B1PP1/R1R3K1 w - -",
                vec![[20, 41], [2, 10], [2, 18], [8, 16]],
            ),
            (
                "6rk/5q1p/p2p4/P1nP1p2/2B2N1Q/1Pb1pPR1/7P/7K w - -",
                vec![[31, 47], [22, 38], [22, 62]],
            ),
            (
                "3r2k1/p4rp1/1pRp3q/1P1Ppb2/7P/Q4P2/P5BP/2R4K b - -",
                vec![[47, 11], [62, 55], [47, 29]],
            ),
            (
                "rq4k1/3bn1bp/3p2p1/3Pp1N1/4P1p1/2N1B1P1/1P2QP1P/4R1K1 b - -",
                vec![[57, 17], [62, 63], [55, 39]],
            ),
            (
                "3rk3/2p1r1p1/7q/p1p1P1R1/2B4P/4P3/PPQ5/6K1 w - -",
                vec![[10, 28], [6, 14], [9, 17]],
            ),
            (
                "2r2r1k/3q2np/p2P1pp1/1pP1p3/7N/5QP1/P4P1P/2R1R1K1 w - -",
                vec![[21, 35], [21, 3], [21, 19]],
            ),
            (
                "r2r2k1/5p1p/6nQ/ppq5/2p1P3/P4P2/1P4PP/R1N2R1K b - -",
                vec![[34, 27], [34, 43], [34, 13]],
            ),
            (
                "r3r1k1/ppq3p1/2p1n3/3pN1p1/Pb1P2P1/3QB2P/1P3P2/R1R3K1 w - -",
                vec![[19, 37], [6, 14], [36, 46], [19, 46], [2, 10]],
            ),
            (
                "2bq1rk1/1r3p2/3p2pp/p1p1p3/PnP1P3/2QPP1N1/2B3PP/1R3RK1 b - -",
                vec![[59, 38], [58, 51], [59, 60], [47, 39]],
            ),
            (
                "2q5/5kp1/pP2p1r1/3b1p1Q/3P1P2/6P1/5K2/4RB2 b - -",
                vec![[58, 18], [35, 49], [58, 10], [58, 42]],
            ),
            (
                "2q2r1k/7p/p2p1p2/1p1Np3/nP6/4Q2P/P4PP1/4R1K1 w - -",
                vec![[20, 47], [20, 11], [4, 2], [4, 12]],
            ),
            (
                "8/3k4/2p5/3r4/1PQBp1pq/4P3/P4PpP/6K1 w - -",
                vec![[26, 40], [6, 14], [8, 24], [25, 33]],
            ),
            (
                "8/7p/p2p1kqP/2r1b3/P1B1p1r1/1P6/4QP2/3R1K1R b - -",
                vec![[46, 37], [46, 38], [30, 29], [40, 32]],
            ),
            (
                "2b1rnk1/1p3pp1/r3p2p/1q2P3/p2PQ3/P1P2PB1/6PP/1BKR3R b - -",
                vec![[33, 17], [58, 51], [33, 32], [40, 42]],
            ),
            (
                "2r5/p4kp1/1pn2n2/3q1b2/3p4/P2P2Q1/2RB1PPP/4R1K1 b - -",
                vec![[35, 17], [45, 51], [45, 39], [35, 33]],
            ),
            (
                "r6k/1p2rR1P/2p3p1/4p1q1/8/pP6/P1P5/1K2Q2R w - -",
                vec![[4, 32], [53, 13], [53, 21], [53, 5]],
            ),
            (
                "r3r1k1/2q3p1/p2p1p2/3P2p1/n2B2P1/8/P1PQ3P/K2R1R2 b - -",
                vec![[50, 26], [50, 58], [56, 58], [60, 28]],
            ),
            (
                "3r2k1/2p2pp1/p6p/1p5q/1PbB4/P1Q3P1/5P1P/4R1K1 b - -",
                vec![[39, 30], [59, 43], [40, 32], [53, 45]],
            ),
            (
                "r5k1/1q4b1/6n1/3pp1Nn/1N3p2/3P2Pp/2QBP2P/1R4K1 w - -",
                vec![[10, 34], [38, 23]],
            ),
            (
                "6r1/4pp1k/3p3p/2qP1P2/r3P1PK/1R6/4Q3/1R6 b - -",
                vec![[34, 27], [24, 16], [62, 38], [62, 56], [62, 54]],
            ),
            (
                "8/p3b1k1/2p5/3qp2p/P2p3P/1P1Q1PP1/3NP1K1/8 w - -",
                vec![[19, 37], [11, 26], [19, 40], [19, 1], [19, 10]],
            ),
            (
                "1rb2rk1/2p3pp/p1p1p3/2N5/8/1PQ2PPq/P3P3/R2R2K1 w - -",
                vec![[18, 36], [6, 13], [34, 28]],
            ),
            (
                "2rqrbk1/1b1n1p2/p2p1npp/1p6/4P2B/1B3NNP/PP1Q1PP1/3RR1K1 w - -",
                vec![[11, 29], [4, 12], [8, 16], [8, 24]],
            ),
            (
                "r1k5/4qp2/P1p1b1p1/6Pp/n3P2P/6Q1/2N5/1K1R2R1 b - -",
                vec![[52, 34], [56, 40]],
            ),
            (
                "6k1/3qbp1p/6p1/3Pp1P1/3n3P/pP1Q4/P7/1K2B2B b - -",
                vec![[51, 30], [52, 43], [62, 61], [55, 47]],
            ),
            (
                "3n4/p2rk3/3pq1p1/pR3p1p/2PQ1P2/1PB4P/6PK/8 w - -",
                vec![[27, 63], [18, 9], [15, 7], [33, 57], [33, 35]],
            ),
            (
                "5b1k/r7/Pq5p/2p5/3p3N/R2Q2P1/5PK1/8 w - -",
                vec![[19, 37], [31, 46], [19, 21], [19, 46]],
            ),
        ];

        let start_time = std::time::Instant::now();

        for (i, (fen, moves)) in fens.iter().enumerate() {
            if i >= 50 {
                break;
            }
            println!("test {}/{}", i + 1, fens.len());

            println!("fen: {}", fen);
            println!("moves: {:?}\n", moves);
            let mut board = board::Board::new();
            board.load_from_fen(fen);

            let inner_start_time = std::time::Instant::now();

            let best_move = board.engine(
                4,
                1,
                false,
                false,
                false,
                false,
                std::time::Duration::from_secs(20),
            );
            dbg!((best_move.from(), best_move.to()));
            dbg!(inner_start_time.elapsed());

            if moves.contains(&[best_move.from() as u8, best_move.to() as u8]) {
                println!("✅ ok");
            } else {
                println!("❌ fail");
            }

            println!("\n\n\n");
        }

        println!("Time taken: {:?}", start_time.elapsed());
    } //
}
