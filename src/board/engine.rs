use crate::board;

use super::{Board, GameState, Turn};

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
    pub fn minimax(&mut self, depth: i32) -> i32 {
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

        let best_score = 0;

        let moves = self.generate_moves();
        match self.turn {
            Turn::WHITE => {
                let mut best_score = i32::MIN;
                for mv in moves {
                    let old_bitmaps = self.bitboards;
                    self.make_move(mv);
                    let score = self.minimax(depth + 1);
                    self.bitboards = old_bitmaps;
                    if score > best_score {
                        best_score = score;
                    }
                }
                return best_score;
            }//
            Turn::BLACK => {
                let mut best_score = i32::MAX;
                for mv in moves {
                    let old_bitmaps = self.bitboards;
                    self.make_move(mv);
                    let score = self.minimax(depth + 1);
                    self.bitboards = old_bitmaps;
                    if score < best_score {
                        best_score = score;
                    }
                }
                return best_score;
            }//
        }//
    }
}

mod test {
    use super::*;

    #[test]
    fn is_position_equal() {
        let mut board = Board::new();
        assert_eq!(board.evaluate(), 0);
    }

    #[test]
    fn minimax() {
        let mut board = Board::new();
        println!("{}", board.minimax(0));
    }

}
