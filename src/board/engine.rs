use std::collections::HashMap;

use super::zobrist::{Z_PIECE, Z_SIDE};
use super::{Board, GameState, Move, PieceType, TTEntry, TranspositionTable, Turn};

impl TranspositionTable {
    pub fn new(size_pow2: usize) -> Self {
        let size = 1usize << size_pow2;
        Self {
            table: vec![None; size],
            mask: size - 1,
        }
    }

    #[inline(always)]
    fn index(&self, key: u64) -> usize {
        (key as usize) & self.mask
    }

    #[inline(always)]
    pub fn get(&self, key: u64, depth: i8) -> Option<i32> {
        let entry = self.table[self.index(key)]?;
        if entry.key == key && entry.depth >= depth {
            Some(entry.score)
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn put(&mut self, key: u64, depth: i8, score: i32) {
        let idx = self.index(key);
        self.table[idx] = Some(TTEntry { key, depth, score });
    }
}

impl Board {
    /// The score is turn agnostic , it always returns the score of the white player
    pub fn pieces_score(&self) -> i32 {
        let mut score: i32 = 0;
        let num_of_knights = (self.bitboards.white_knights.0).count_ones();
        let num_of_pawns = self.bitboards.white_pawns.0.count_ones();
        let num_of_bishops = self.bitboards.white_bishops.0.count_ones();
        let num_of_rooks = self.bitboards.white_rooks.0.count_ones();
        let num_of_queens = self.bitboards.white_queens.0.count_ones();

        let num_of_enemy_knights = self.bitboards.black_knights.0.count_ones();
        let num_of_enemy_pawns = self.bitboards.black_pawns.0.count_ones();
        let num_of_enemy_bishops = self.bitboards.black_bishops.0.count_ones();
        let num_of_enemy_rooks = self.bitboards.black_rooks.0.count_ones();
        let num_of_enemy_queens = self.bitboards.black_queens.0.count_ones();

        score += (num_of_knights * 3) as i32;
        score += (num_of_pawns * 1) as i32;
        score += (num_of_bishops * 3) as i32;
        score += (num_of_rooks * 5) as i32;
        score += (num_of_queens * 9) as i32;

        score -= (num_of_enemy_knights * 3) as i32;
        score -= (num_of_enemy_pawns * 1) as i32;
        score -= (num_of_enemy_bishops * 3) as i32;
        score -= (num_of_enemy_rooks * 5) as i32;
        score -= (num_of_enemy_queens * 9) as i32;

        score
    } //

    pub fn evaluate(&mut self) -> i32 {
        let game_state = self.get_game_state();
        match game_state {
            GameState::CheckMate => match self.turn {
                Turn::WHITE => return i32::MIN,
                Turn::BLACK => return i32::MAX,
            },
            GameState::StaleMate => return 0,
            _ => (),
        }

        let score = self.pieces_score();

        return score;
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
        mut alpha: i32,
        mut beta: i32,
        tt: &mut TranspositionTable,
    ) -> i32 {
        const MAX_DEPTH: i32 = 8;
        let remaining_depth = (MAX_DEPTH - depth) as i8;

        if let Some(score) = tt.get(self.hash, (MAX_DEPTH - depth) as i8) {
            // dbg!("skipped evaluation");
            return score;
        }
        let game_state = self.get_game_state();
        if game_state == GameState::CheckMate {
            match self.turn {
                Turn::WHITE => return i32::MIN + depth,
                Turn::BLACK => return i32::MAX - depth,
            }
        } else if game_state == GameState::StaleMate {
            return 0;
        }

        if depth >= MAX_DEPTH {
            return self.evaluate();
        }
        let mut moves = Vec::new();
        self.generate_pesudo_moves(&mut moves);

        // let mut moves = self.generate_moves();

        let iter = moves.iter().filter(|m| m.capture)
                                                .chain(moves.iter().filter(|m| !m.capture));


        match self.turn {
            Turn::WHITE => {
                let mut best_score = i32::MIN;
                for mv in iter {
                    let unmake_move = self.make_move(*mv);

                    let is_illegal_move = self.is_king_in_check(self.opposite_turn());

                    if is_illegal_move {
                        self.unmake_move(unmake_move);
                        continue;
                    }

                    let score = self.alpha_beta(depth + 1, alpha, beta, tt);

                    self.unmake_move(unmake_move);

                    best_score = best_score.max(score);
                    alpha = alpha.max(best_score);

                    if alpha >= beta {
                        break;
                    }
                }
                tt.put(self.hash, remaining_depth, best_score);
                return best_score;
            } //
            Turn::BLACK => {
                let mut best_score = i32::MAX;
                for mv in iter {
                    let unmake_move = self.make_move(*mv);

                    let is_illegal_move = self.is_king_in_check(self.opposite_turn());

                    if is_illegal_move {
                        self.unmake_move(unmake_move);
                        continue;
                    }

                    let score = self.alpha_beta(depth + 1, alpha, beta, tt);

                    self.unmake_move(unmake_move);

                    best_score = best_score.min(score);
                    beta = beta.min(best_score);

                    if alpha >= beta {
                        break;
                    }
                }
                tt.put(self.hash, remaining_depth, best_score);
                return best_score;
            } //
        } //
    } //

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
            let mut bb = self.bitboards.get(piece);
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
}

mod test {

    #[test]
    fn is_position_equal() {
        use super::Board;
        let mut board = Board::new();
        assert_eq!(board.evaluate(), 0);
    }

    #[test]
    fn minimax() {
        use super::Board;
        use super::TranspositionTable;
        use std::collections::HashMap;

        let mut board = Board::new();
        board.load_from_fen("1rbk1bnr/pp3ppp/1Pp1p3/3p1P2/5N1q/2NQ2P1/1PP1P2P/R1B1KB1R w ");
        let mut moves_map: HashMap<u64, (i32, i32)> = HashMap::new();
        let score_minimax = board.minimax(0, &mut moves_map);
        // let mut moves_map2: HashMap<u64, (i32, i32)> = HashMap::new();
        let mut tt = TranspositionTable::new(20);
        let score_alpha_beta = board.alpha_beta(0, i32::MIN, i32::MAX, &mut tt);
        println!(
            "minimax: {} , alphabeta: {}",
            score_minimax, score_alpha_beta
        );
    }
}
