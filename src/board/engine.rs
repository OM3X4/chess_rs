use super::{Board, Turn , GameState};

impl Board {
    /// The score is turn agnostic , it always returns the score of the white player
    pub fn pieces_score(&self) -> i32 {
        let mut score = 0;
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

        score += num_of_knights * 3;
        score += num_of_pawns * 1;
        score += num_of_bishops * 3;
        score += num_of_rooks * 5;
        score += num_of_queens * 9;

        score -= num_of_enemy_knights * 3;
        score -= num_of_enemy_pawns * 1;
        score -= num_of_enemy_bishops * 3;
        score -= num_of_enemy_rooks * 5;
        score -= num_of_enemy_queens * 9;

        score as i32
    } //

    pub fn evaluate(&mut self) -> i32 {
        let game_state = self.get_game_state();
        match game_state {
            GameState::CheckMate => {
                match self.turn {
                    Turn::WHITE => return i32::MIN,
                    Turn::BLACK => return i32::MAX,
                }
            },
            GameState::StaleMate => return 0,
            _ => (),
        }

        let score = self.pieces_score();

        return score;
    } //
}

mod test {
    use super::*;

    #[test]
    fn is_position_equal() {
        let mut board = Board::new();
        assert_eq!(board.evaluate(), 0);
    }
}
