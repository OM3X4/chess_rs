use std::collections::HashMap;

use super::{Board, GameState, Move, Turn};

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
        moves_map: &mut HashMap<u64, (i32, i32)>,
    ) -> i32 {
        let game_state = self.get_game_state();
        if game_state == GameState::CheckMate {
            match self.turn {
                Turn::WHITE => return i32::MIN + depth,
                Turn::BLACK => return i32::MAX - depth,
            }
        } else if game_state == GameState::StaleMate {
            return 0;
        }

        if depth >= 5 {
            return self.evaluate();
        }

        let moves: Vec<Move>;
        if let Some((score, stored_depth)) = moves_map.get(&self.hash) {
            if *stored_depth > depth {
                return *score;
            }
        }
        let moves = self.generate_moves();
        let hashmap: HashMap<Move, Vec<String>> = HashMap::new();
        match self.turn {
            Turn::WHITE => {
                let mut best_score = i32::MIN;
                for mv in moves {
                    let unmake_move = self.make_move(mv);

                    let score = self.alpha_beta(depth + 1, alpha, beta, moves_map);

                    self.unmake_move(unmake_move);

                    best_score = best_score.max(score);
                    alpha = alpha.max(best_score);

                    if alpha >= beta {
                        break;
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

                    best_score = best_score.min(score);
                    beta = beta.min(best_score);

                    if alpha >= beta {
                        break;
                    }
                }
                moves_map.insert(self.hash, (best_score, depth));
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

    pub fn compute_hash(&mut self) -> u64 {
        let mut h = 0u64;
        let z = &self.zobrist;

        for sq in 0..64 {
            if let Some(piece) = self.piece_at(sq) {
                let piece_index = piece.piece_index();
                h ^= z.piece_square[piece_index][sq as usize];
            }
        }

        if self.turn == Turn::BLACK {
            h ^= z.side_to_move;
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
        use std::collections::HashMap;

        let mut board = Board::new();
        let mut moves_map: HashMap<u64, (i32, i32)> = HashMap::new();
        let score_minimax = board.minimax(0, &mut moves_map);
        let mut moves_map2: HashMap<u64, (i32, i32)> = HashMap::new();
        let score_alpha_beta = board.alpha_beta(0, i32::MIN, i32::MAX, &mut moves_map2);
        println!("minimax: {} , alphabeta: {}", score_minimax, score_alpha_beta);
    }
}
