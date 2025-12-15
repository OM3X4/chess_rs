use super::{BitBoard, BitBoards, Move, Turn , GameState};
use super::constants::{RANK_1 , RANK_2, RANK_7, RANK_8};

pub struct Board {
    pub bitboards: BitBoards,
    pub undo: Option<Move>,
    pub turn: Turn,
    pub occupied: BitBoard,
}



impl Board {
    pub fn new() -> Board {
        return Board {
            bitboards: BitBoards::default(),
            undo: None,
            turn: Turn::WHITE,
            occupied: BitBoard(RANK_1 | RANK_2 | RANK_7 | RANK_8),
        };
    }//
    pub fn reset_to_default(&mut self) {
        self.bitboards = BitBoards::default();
        self.occupied = BitBoard(RANK_1 | RANK_2 | RANK_7 | RANK_8);
        self.turn = Turn::WHITE;
    }//
    pub fn reset_to_zero(&mut self) {
        self.bitboards = BitBoards::zero();
        self.occupied = BitBoard(0);
        self.turn = Turn::WHITE;
    }//
    pub fn get_all_white_bits(&self) -> BitBoard {
        return BitBoard(
            self.bitboards.white_pawns.0
                | self.bitboards.white_knights.0
                | self.bitboards.white_bishops.0
                | self.bitboards.white_rooks.0
                | self.bitboards.white_queens.0
                | self.bitboards.white_king.0,
        );
    }//
    pub fn get_all_black_bits(&self) -> BitBoard {
        return BitBoard(
            self.bitboards.black_pawns.0
                | self.bitboards.black_knights.0
                | self.bitboards.black_bishops.0
                | self.bitboards.black_rooks.0
                | self.bitboards.black_queens.0
                | self.bitboards.black_king.0,
        );
    }//
    pub fn get_all_bits(&self) -> BitBoard {
        return BitBoard(
            self.bitboards.white_pawns.0
                | self.bitboards.white_knights.0
                | self.bitboards.white_bishops.0
                | self.bitboards.white_rooks.0
                | self.bitboards.white_queens.0
                | self.bitboards.white_king.0
                | self.bitboards.black_pawns.0
                | self.bitboards.black_knights.0
                | self.bitboards.black_bishops.0
                | self.bitboards.black_rooks.0
                | self.bitboards.black_queens.0
                | self.bitboards.black_king.0,
        );
    } //

    pub fn load_from_fen(&mut self, fen: &str) {
        self.reset_to_zero();

        let (position, turn) = fen.split_once(' ').unwrap();
        self.turn = if turn == "w" {
            Turn::WHITE
        } else {
            Turn::BLACK
        };

        let rows: Vec<&str> = position.split('/').collect();

        for rank in 0..8 {
            let mut file: u64 = 0;
            for char in rows[rank].chars() {
                if let Some(number) = char.to_digit(10) {
                    file += number as u64;
                } else {
                    let square_index = (7 - rank as u64) * 8 + file;
                    let bit = 1u64 << square_index;
                    let target_board = match char {
                        'P' => Some(&mut self.bitboards.white_pawns),
                        'R' => Some(&mut self.bitboards.white_rooks),
                        'Q' => Some(&mut self.bitboards.white_queens),
                        'K' => Some(&mut self.bitboards.white_king),
                        'N' => Some(&mut self.bitboards.white_knights),
                        'B' => Some(&mut self.bitboards.white_bishops),
                        'p' => Some(&mut self.bitboards.black_pawns),
                        'r' => Some(&mut self.bitboards.black_rooks),
                        'q' => Some(&mut self.bitboards.black_queens),
                        'k' => Some(&mut self.bitboards.black_king),
                        'n' => Some(&mut self.bitboards.black_knights),
                        'b' => Some(&mut self.bitboards.black_bishops),
                        _ => None,
                    };

                    if let Some(bitboard) = target_board {
                        bitboard.0 = bit | bitboard.0;
                        file += 1;
                    }
                }
            }
        }

        self.occupied = self.get_all_bits();
    } //

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();

        for rank in (0..8).rev() {
            let mut empty = 0;

            for file in 0..8 {
                let sq = rank * 8 + file;
                let mask = 1u64 << sq;

                let piece = if self.bitboards.white_pawns.0 & mask != 0 {
                    'P'
                } else if self.bitboards.white_knights.0 & mask != 0 {
                    'N'
                } else if self.bitboards.white_bishops.0 & mask != 0 {
                    'B'
                } else if self.bitboards.white_rooks.0 & mask != 0 {
                    'R'
                } else if self.bitboards.white_queens.0 & mask != 0 {
                    'Q'
                } else if self.bitboards.white_king.0 & mask != 0 {
                    'K'
                } else if self.bitboards.black_pawns.0 & mask != 0 {
                    'p'
                } else if self.bitboards.black_knights.0 & mask != 0 {
                    'n'
                } else if self.bitboards.black_bishops.0 & mask != 0 {
                    'b'
                } else if self.bitboards.black_rooks.0 & mask != 0 {
                    'r'
                } else if self.bitboards.black_queens.0 & mask != 0 {
                    'q'
                } else if self.bitboards.black_king.0 & mask != 0 {
                    'k'
                } else {
                    empty += 1;
                    continue;
                };

                if empty > 0 {
                    fen.push(char::from_digit(empty, 10).unwrap());
                    empty = 0;
                }

                fen.push(piece);
            }

            if empty > 0 {
                fen.push(char::from_digit(empty, 10).unwrap());
            }

            if rank != 0 {
                fen.push('/');
            }
        }

        fen.push(' ');
        fen.push(match self.turn {
            Turn::WHITE => 'w',
            Turn::BLACK => 'b',
        });

        fen
    } //

    pub fn get_enemy_pieces(&self) -> BitBoard {
        return match self.turn {
            Turn::WHITE => self.get_all_black_bits(),
            Turn::BLACK => self.get_all_white_bits(),
        };
    }//

    pub fn get_allay_pieces(&self) -> BitBoard {
        return match self.turn {
            Turn::BLACK => self.get_all_black_bits(),
            Turn::WHITE => self.get_all_white_bits(),
        };
    }//

    pub fn switch_turn(&mut self) {
        self.turn = match self.turn {
            Turn::BLACK => Turn::WHITE,
            Turn::WHITE => Turn::BLACK,
        }
    } //

    pub fn get_game_state(&mut self) -> GameState {
        let moves = self.generate_moves();
        let is_king_in_check = self.is_king_in_check(self.turn);
        if moves.len() == 0 && is_king_in_check {
            return GameState::CheckMate;
        } else if moves.len() == 0 && !is_king_in_check {
            return GameState::StaleMate;
        } else {
            return GameState::InProgress;
        }
    }//
}//

mod test {

    use super::*;

    #[test]
    fn check_mate() {
        let mut board = Board::new();
        board.load_from_fen("rnbqkbnr/pppppQpp/8/8/2B5/8/PPPPPPPP/RNB1K1NR b");

        assert_eq!(board.get_game_state(), GameState::CheckMate);
        // assert_eq!(board.is_check_mate(), true);
    }
}