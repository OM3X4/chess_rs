use rand::prelude::IndexedRandom;
use smallvec::{SmallVec, smallvec};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, current}; // Import the trait for random selection
use std::time::Duration;

use crate::board::constants::{RANK_1, RANK_8};

use super::constants::{MVV_LVA, get_book_moves};
use super::zobrist::{Z_CASTLING, Z_PIECE, Z_SIDE};
use super::{Board, Bound, Move, PieceType, TTEntry, TranspositionTable, Turn};

static NODE_COUNT: AtomicU64 = AtomicU64::new(0);
static SKIP_COUNT: AtomicU64 = AtomicU64::new(0);

static MAXIMUM_NODE_COUNT: AtomicU64 = AtomicU64::new(0);

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
    pub fn probe(&self, key: u64) -> Option<TTEntry> {
        let entry = self.table[self.index(key)]?;

        if entry.key != key {
            return None;
        }

        return Some(entry);
    } //

    #[inline(always)]
    pub fn store(
        &mut self,
        key: u64,
        depth: i8,
        score: i32,
        alpha: i32,
        beta: i32,
        best_move: Move,
    ) {
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
                    best_move,
                });
            }
            Some(old) if depth >= old.depth => {
                self.table[idx] = Some(TTEntry {
                    key,
                    depth,
                    score,
                    bound,
                    best_move,
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

        if self.castling & 0b0001 != 0 {
            h ^= Z_CASTLING[0];
        }
        if self.castling & 0b0010 != 0 {
            h ^= Z_CASTLING[1];
        }
        if self.castling & 0b0100 != 0 {
            h ^= Z_CASTLING[2];
        }
        if self.castling & 0b1000 != 0 {
            h ^= Z_CASTLING[3];
        }

        h
    } //

    /// Returns a random move from the opening book for a given FEN string.
    /// Returns None if the position is not in the book.
    pub fn get_random_opening_move(&self) -> Option<&'static str> {
        // We fetch the slice of available moves
        let fen = self.to_fen();
        let mut parts = fen.split_whitespace().collect::<Vec<_>>();
        parts.truncate(parts.len() - 2);

        let moves = get_book_moves(&parts.join(" "))?;

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
        NODE_COUNT.fetch_add(1, Ordering::Relaxed);
        let stand_pat = match self.turn {
            Turn::WHITE => self.eval,
            Turn::BLACK => -self.eval,
        };

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
        remaining_depth: i32,
        mut alpha: i32,
        mut beta: i32,
        tt: &mut TranspositionTable,
        is_alpha_beta: bool,
        is_tt: bool,
        is_null_move_pruning: bool,
        is_lmr: bool,
        is_quiesense: bool,
    ) -> i32 {
        NODE_COUNT.fetch_add(1, Ordering::Relaxed);

        if self.is_3fold_repetition() {
            return 0;
        }

        let orig_alpha = alpha;
        let orig_beta = beta;
        let mut best_move_from_tt: Option<Move> = None;

        // 1. TT LOOKUP
        // We will insert the move to the moves array and skip the same move if we found it in the main loop
        if is_tt {
            if let Some(entry) = tt.probe(self.hash) {
                best_move_from_tt = Some(entry.best_move);

                if entry.depth >= remaining_depth as i8 {
                    match entry.bound {
                        Bound::Exact => {
                            return entry.score;
                        }
                        Bound::Lower => {
                            alpha = alpha.max(entry.score);
                        }
                        Bound::Upper => {
                            beta = beta.min(entry.score);
                        }
                    }
                    if alpha >= beta {
                        return alpha;
                    }
                }
            }
        };

        // 2. BASE CASE (Optimized)
        if remaining_depth == 0 {
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
        if remaining_depth >= 3 && !self.is_king_in_check(self.turn) && is_null_move_pruning {
            let r = 2;
            self.switch_turn();
            let score = -self.alpha_beta(
                remaining_depth - 2 - 1,
                -beta,
                -(beta - 1),
                tt,
                is_alpha_beta,
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

        // Inserting the TT best move , didn't delete the original move (the same move) yet
        if let Some(tt_mv) = best_move_from_tt {
            moves.retain(|m| *m != tt_mv);
            moves.insert(0, tt_mv);
        }

        let iter = moves.iter();

        let mut found_legal = false;

        let mut best_score = -30_000;
        let mut best_move = moves[0];

        let opposite_turn = self.opposite_turn();

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
                    remaining_depth - 1,
                    -beta,
                    -alpha,
                    tt,
                    is_alpha_beta,
                    is_tt,
                    is_null_move_pruning,
                    is_lmr,
                    is_quiesense,
                );
            } else {
                // Reduction

                let reduction = 1;
                let reduced_remaining = remaining_depth_next - reduction;

                let reduced_score = -self.alpha_beta(
                    reduced_remaining,
                    -beta,
                    -alpha,
                    tt,
                    is_alpha_beta,
                    is_tt,
                    is_null_move_pruning,
                    is_lmr,
                    false,
                );

                if reduced_score >= alpha {
                    score = -self.alpha_beta(
                        remaining_depth - 1,
                        -beta,
                        -alpha,
                        tt,
                        is_alpha_beta,
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

            if score > best_score {
                best_score = score;
                best_move = *mv;
            }
            alpha = alpha.max(best_score);

            if alpha >= beta && is_alpha_beta {
                break; // Alpha Cutoff
            }
        } //

        if !found_legal {
            if self.is_king_in_check(self.turn) {
                let mate = 30_000;
                return -mate - remaining_depth;
            } else {
                return 0; // Stalemate
            }
        };

        if is_tt && best_score.abs() < 29_000 { // The Move isn't Mate
            tt.store(
                self.hash,
                remaining_depth as i8,
                best_score,
                orig_alpha,
                orig_beta,
                best_move
            );
        };

        return best_score;
    } //

    pub fn engine_singlethread(
        &mut self,
        max_depth: i32,
        is_alpha_beta: bool,
        is_tt: bool,
        is_null_move_pruning: bool,
        is_lmr: bool,
        is_quiesense: bool,
        is_move_ordering: bool,
        maximum_time: Duration,
    ) -> Move {
        let mut moves = self.generate_moves();
        partition_by_bool(&mut moves, |mv| mv.is_capture());

        let start_time = std::time::Instant::now();

        let mut searched_depth = 0;
        let mut best_stable_move = moves[0];
        let mut best_move = moves[0];
        let mut tt = TranspositionTable::new(20);

        let mut root_moves = vec![];

        moves.iter().for_each(|mv| root_moves.push((*mv, 0)));

        // let mut pv = Vec::new();

        for current_depth in 1..=max_depth {
            // dbg!(current_depth);
            let mut alpha = -30_000;
            let beta = 30_000;
            let mut best_score = -30_000;

            for (mv, prev_score) in &mut root_moves {
                if start_time.elapsed() > maximum_time {
                    dbg!(searched_depth);
                    return best_stable_move;
                }

                let unmake_move = self.make_move(*mv);

                let score = -self.alpha_beta(
                    current_depth - 1,
                    -beta,
                    -alpha,
                    &mut tt,
                    is_alpha_beta,
                    is_tt,
                    is_null_move_pruning,
                    is_lmr,
                    is_quiesense,
                );

                *prev_score = score;

                if score > best_score {
                    best_score = score;
                    best_move = *mv;
                }

                alpha = alpha.max(score);

                self.unmake_move(unmake_move);
            } //

            // if let Some(idx) = moves.iter().position(|m| *m == best_move) {
            //     moves.swap(0, idx);
            // };

            if is_move_ordering {
                root_moves.sort_by_key(|(_, score)| -*score);
            }

            // uci info print
            println!(
                "info depth {current_depth} score cp {best_score} nodes {} pv {}",
                NODE_COUNT.load(Ordering::Relaxed),
                best_move.to_uci()
            );

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
        is_alpha_beta: bool,
        is_tt: bool,
        is_null_move_pruning: bool,
        is_lmr: bool,
        is_quiesense: bool,
        is_move_ordering: bool,
        maximum_time: Duration,
    ) -> Move {
        // if let Some(uci) = self.get_random_opening_move() {
        //     let bytes = uci.as_bytes();

        //     // Decode squares
        //     let file_from = bytes[0] - b'a';
        //     let rank_from = bytes[1] - b'1';
        //     let from = (rank_from << 3) | file_from;

        //     let file_to = bytes[2] - b'a';
        //     let rank_to = bytes[3] - b'1';
        //     let to = (rank_to << 3) | file_to;

        //     // Get moving piece from board
        //     let piece = self.piece_at[from as usize]
        //         .expect("Opening book move refers to empty from-square");

        //     // Detect capture
        //     let capture = self.piece_at[to as usize].is_some();

        //     dbg!("Opening book move: {}", uci);

        //     return Move::new(from, to, piece, capture, false, false, false);
        // };

        self.engine_singlethread(
            max_depth,
            is_alpha_beta,
            is_tt,
            is_null_move_pruning,
            is_lmr,
            is_quiesense,
            is_move_ordering,
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

    use crate::board::{Move, board};

    #[test]
    fn test() {
        use crate::board::bishop_magic::init_bishop_magics;
        use crate::board::rook_magic::init_rook_magics;

        init_rook_magics();
        init_bishop_magics();

        let mut board = board::Board::new();
        dbg!(&board.history);
        // board.load_from_fen("r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 4 ");
        board.make_move(Move::from_uci("g1f3", &board));
        board.make_move(Move::from_uci("g8f6", &board));
        board.make_move(Move::from_uci("f3g1", &board));
        board.make_move(Move::from_uci("f6g8", &board));
        board.make_move(Move::from_uci("g1f3", &board));
        board.make_move(Move::from_uci("g8f6", &board));
        board.make_move(Move::from_uci("f3g1", &board));
        board.make_move(Move::from_uci("f6g8", &board));
        dbg!(&board.history);
        // board.make_move(Move::from_uci("f6g8", &board));


        let start = std::time::Instant::now();
        // dbg!(
        //     board
        //         .generate_moves()
        //         .iter()
        //         .map(|mv| mv.to_uci())
        //         .collect::<Vec<String>>()
        // );
        // let moves = board
        //     .generate_moves()
        //     .iter()
        //     .map(|mv| mv.to_uci())
        //     .collect::<Vec<String>>();
        // dbg!(moves);
        // dbg!(
        //     board
        //         .engine(64, true, false, true, true, false, std::time::Duration::from_millis(5000))
        //         .to_uci()
        // );
        dbg!(start.elapsed());
    } //
}
