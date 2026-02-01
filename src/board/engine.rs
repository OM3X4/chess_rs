use crate::board::constants::IS_STOP;
use smallvec::SmallVec;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use super::constants::MVV_LVA;
use crate::board::{Board, Move, Turn};
use crate::board::tt::{TranspositionTable, Bound};

static NODE_COUNT: AtomicU64 = AtomicU64::new(0);

impl Board {

    #[inline(always)]
    pub fn pieces_score(&self) -> i32 {
        let bbs = &self.bitboards;

        let white = bbs[0].0.count_ones() * 100   // pawn
        + bbs[1].0.count_ones() * 300   // knight
        + bbs[2].0.count_ones() * 300   // bishop
        + bbs[3].0.count_ones() * 500   // rook
        + bbs[4].0.count_ones() * 900; // queen

        let black = bbs[6].0.count_ones() * 100
            + bbs[7].0.count_ones() * 300
            + bbs[8].0.count_ones() * 300
            + bbs[9].0.count_ones() * 500
            + bbs[10].0.count_ones() * 900;

        (white - black) as i32
    } //

    pub fn evaluate(&mut self) -> i32 {
        const MAX_PHASE: i32 = 16;

        let mut phase = (self.number_of_pieces - self.number_of_pawns) as i32 * MAX_PHASE / 14;

        phase = phase.clamp(0, MAX_PHASE);

        let pst_score =
            (self.mg_pst_eval * phase + self.eg_pst_eval * (MAX_PHASE - phase)) / MAX_PHASE;

        let score = self.mat_eval + pst_score + self.mobility_eval;

        score
    } //

    #[inline(always)]
    pub fn mvv_lva(&self, mv: Move) -> i32 {
        if !mv.is_capture() || mv.is_en_passant() {
            return 0;
        }

        let victim = self.piece_at[mv.to()].unwrap_or_else(|| {
            println!("{}", mv.to());
            println!("{}", mv.from());
            println!("{}", self.to_fen());
            panic!()
        });
        let attacker = mv.piece();

        MVV_LVA[victim.piece_index() % 6][attacker.piece_index() % 6]
    } //

    pub fn score_move(
        &self,
        mv: Move,
        ply: usize,
        killer: &[[Option<Move>; 2]],
        tt_move: Option<Move>,
    ) -> i32 {
        if Some(mv) == tt_move {
            10_000
        } else if Some(mv) == killer[ply][0] {
            9_000
        } else if Some(mv) == killer[ply][1] {
            8_000
        } else if mv.is_capture() {
            10_000 + self.mvv_lva(mv)
        } else {
            0
        }
    } //

    pub fn sort_moves_by_score(
        &mut self,
        moves: &mut SmallVec<[Move; 256]>,
        ply: usize,
        killer: &[[Option<Move>; 2]],
        tt_move: Option<Move>,
    ) {
        moves.sort_unstable_by(|a, b| {
            let va = self.score_move(*a, ply, killer, tt_move);
            let vb = self.score_move(*b, ply, killer, tt_move);
            vb.cmp(&va)
        });
    } //

    pub fn quiescence(&mut self, alpha: i32, beta: i32) -> i32 {
        NODE_COUNT.fetch_add(1, Ordering::Relaxed);
        let stand_pat = match self.turn {
            Turn::WHITE => self.evaluate(),
            Turn::BLACK => -self.evaluate(),
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

        let iter = moves.iter().filter(|mv| mv.is_capture());

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
        ply: usize,
        remaining_depth: i8,
        mut alpha: i32,
        beta: i32,
        tt: &mut TranspositionTable,
        is_alpha_beta: bool,
        is_tt: bool,
        is_null_move_pruning: bool,
        is_lmr: bool,
        is_quiesense: bool,
        killer_moves: &mut [[Option<Move>; 2]; 128],
    ) -> i32 {
        NODE_COUNT.fetch_add(1, Ordering::Relaxed);
        if IS_STOP.load(Ordering::Relaxed) {
            return alpha;
        }

        if self.is_3fold_repetition() {
            return 0;
        }

        let orig_alpha = alpha;
        let orig_beta = beta;
        let mut best_move_from_tt: Option<Move> = None;

        // 1. TT LOOKUP
        if is_tt {
            if let Some(entry) = tt.probe(self.hash) {
                best_move_from_tt = Some(entry.best_move);

                if entry.depth >= remaining_depth {
                    match entry.bound {
                        Bound::Exact => {
                            let score = if entry.score.abs() > 29_000 {
                                entry.score - (ply as i32)
                            } else {
                                entry.score
                            };
                            return score;
                        }
                        _ => (),
                    }
                }
            }
        };

        // 2. BASE CASE (Optimized)
        if remaining_depth == 0 {
            if is_quiesense {
                return self.quiescence(alpha, beta);
            }
            match self.turn {
                Turn::BLACK => return -self.evaluate(),
                Turn::WHITE => return self.evaluate(),
            }
        };

        // 3. NULL MOVE PRUNING
        if remaining_depth >= 3 && !self.is_king_in_check(self.turn) && is_null_move_pruning {
            let r = 2;
            self.switch_turn();
            let score = -self.alpha_beta(
                ply + 1,
                remaining_depth - r - 1,
                -beta,
                -(beta - 1),
                tt,
                is_alpha_beta,
                false,
                false,
                false,
                false,
                killer_moves,
            );
            self.switch_turn();
            if score >= beta {
                return beta;
            }
        };

        // 4. MOVE GENERATION (Only for internal nodes)
        let mut moves = SmallVec::new();
        self.generate_pesudo_moves(&mut moves);

        self.sort_moves_by_score(&mut moves, ply, killer_moves, best_move_from_tt);

        let iter = moves.iter();

        let mut found_legal = false;
        let mut all_searched = true;

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

            let score: i32;

            let can_lmr = !mv.is_capture() && index >= 4 && remaining_depth_next >= 3 && is_lmr;

            if !can_lmr {
                score = -self.alpha_beta(
                    ply + 1,
                    remaining_depth - 1,
                    -beta,
                    -alpha,
                    tt,
                    is_alpha_beta,
                    is_tt,
                    is_null_move_pruning,
                    is_lmr,
                    is_quiesense,
                    killer_moves,
                );
            } else {
                // Reduction
                all_searched = false;

                let reduction = 1;
                // let reduction = if remaining_depth >= 6 { 2 } else { 1 };
                let reduced_remaining = remaining_depth_next - reduction;

                let reduced_score = -self.alpha_beta(
                    ply + 1,
                    reduced_remaining,
                    -beta,
                    -alpha,
                    tt,
                    is_alpha_beta,
                    is_tt,
                    is_null_move_pruning,
                    is_lmr,
                    false,
                    killer_moves,
                );

                if reduced_score >= alpha {
                    score = -self.alpha_beta(
                        ply + 1,
                        remaining_depth - 1,
                        -beta,
                        -alpha,
                        tt,
                        is_alpha_beta,
                        is_tt,
                        is_null_move_pruning,
                        is_lmr,
                        is_quiesense,
                        killer_moves,
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
                if !mv.is_capture() {
                    if let Some(killer_move_1) = killer_moves[ply][0] {
                        if *mv != killer_move_1 {
                            killer_moves[ply][1] = Some(killer_move_1);
                            killer_moves[ply][0] = Some(*mv);
                        }
                    } else {
                        killer_moves[ply][0] = Some(*mv);
                    }
                }
                all_searched = false;
                break; // Alpha Cutoff
            }
        } //

        if !found_legal {
            if self.is_king_in_check(self.turn) {
                let mate: i32 = 30_000;
                best_score = -mate - (remaining_depth as i32);
            } else {
                best_score = 0; // Stalemate
            }
        };

        let tt_score = if best_score.abs() > 29_000 {
            best_score + (ply as i32)
        } else {
            best_score
        };

        if is_tt {
            // The Move isn't Mate
            tt.store(
                self.hash,
                remaining_depth as i8,
                tt_score,
                orig_alpha,
                orig_beta,
                best_move,
                all_searched,
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
        tt_global: Option<&mut TranspositionTable>,
    ) -> Move {
        let moves = self.generate_moves();
        let start_time = std::time::Instant::now();

        let mut searched_depth = 0;
        let mut best_stable_move = moves[0];
        let mut best_move = moves[0];

        let mut tt = match tt_global {
            Some(tt) => tt,
            None => &mut TranspositionTable::new(20),
        };

        // let mut tt = TranspositionTable::new(20);
        let mut killer_moves: [[Option<Move>; 2]; 128] = [[None; 2]; 128];

        let mut root_moves = vec![];

        moves.iter().for_each(|mv| root_moves.push((*mv, 0)));

        for current_depth in 1..=max_depth {
            let mut alpha = -30_000;
            let beta = 30_000;
            let mut best_score = -30_000;

            for (mv, prev_score) in &mut root_moves {
                let unmake_move = self.make_move(*mv);

                let score = -self.alpha_beta(
                    1,
                    (current_depth - 1) as i8,
                    -beta,
                    -alpha,
                    &mut tt,
                    is_alpha_beta,
                    is_tt,
                    is_null_move_pruning,
                    is_lmr,
                    is_quiesense,
                    &mut killer_moves,
                );

                *prev_score = score;

                if score > best_score {
                    best_score = score;
                    best_move = *mv;
                }

                alpha = alpha.max(score);

                self.unmake_move(unmake_move);
            } //

            if is_move_ordering {
                root_moves.sort_by_key(|(_, score)| -*score);
            }

            if IS_STOP.load(Ordering::Relaxed) || start_time.elapsed() > maximum_time {
                return best_stable_move;
            }

            // uci info print
            println!(
                "info depth {current_depth} score cp {best_score} nodes {} time {} pv {}",
                NODE_COUNT.load(Ordering::Relaxed),
                start_time.elapsed().as_millis(),
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
        tt: Option<&mut TranspositionTable>,
    ) -> Move {
        if let Some(opening) = self.probe_opening() {
            return opening;
        }

        self.engine_singlethread(
            max_depth,
            is_alpha_beta,
            is_tt,
            is_null_move_pruning,
            is_lmr,
            is_quiesense,
            is_move_ordering,
            maximum_time,
            tt,
        )
    } //

    pub fn perft(&mut self, depth: i32, max_depth: i32) -> i64 {
        if depth == max_depth {
            return 1;
        }
        let mut moves = SmallVec::new();
        self.generate_pesudo_moves(&mut moves);

        let mut nodes = 0;

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
