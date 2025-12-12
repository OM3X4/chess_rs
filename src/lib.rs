mod chess {
    use std::f32::RADIX;

    const RANK_4: u64 = 0x00000000FF000000;
    const RANK_5: u64 = 0x000000FF00000000;
    const RANK_2: u64 = 0x000000000000FF00;
    const RANK_7: u64 = 0x00FF000000000000;
    const RANK_8: u64 = 0xFF00000000000000;
    const RANK_1: u64 = 0x00000000000000FF;

    const FILE_A: u64 = 0x0101010101010101;
    const FILE_H: u64 = 0x8080808080808080;

    #[derive(Debug)]
    struct BitBoard(u64);

    #[derive(Debug)]
    enum Turn {
        WHITE,
        BLACK,
    }

    #[derive(Debug)]
    struct BitBoards {
        // white
        white_pawns: BitBoard,
        white_knights: BitBoard,
        white_bishops: BitBoard,
        white_rooks: BitBoard,
        white_queens: BitBoard,
        white_king: BitBoard,
        //black
        black_pawns: BitBoard,
        black_knights: BitBoard,
        black_bishops: BitBoard,
        black_rooks: BitBoard,
        black_queens: BitBoard,
        black_king: BitBoard,
    }

    impl BitBoards {
        pub fn default() -> BitBoards {
            BitBoards {
                // white
                white_pawns: BitBoard(0x000000000000FF00),
                white_knights: BitBoard(0x0000000000000042),
                white_bishops: BitBoard(0x0000000000000024),
                white_rooks: BitBoard(0x0000000000000081),
                white_queens: BitBoard(0x0000000000000008),
                white_king: BitBoard(0x0000000000000010),

                //black
                black_pawns: BitBoard(0x00FF000000000000),
                black_knights: BitBoard(0x4200000000000000),
                black_bishops: BitBoard(0x2400000000000000),
                black_rooks: BitBoard(0x8100000000000000),
                black_queens: BitBoard(0x0800000000000000),
                black_king: BitBoard(0x1000000000000000),
            }
        }

        pub fn zero() -> BitBoards {
            BitBoards {
                // white
                white_pawns: BitBoard(0),
                white_knights: BitBoard(0),
                white_bishops: BitBoard(0),
                white_rooks: BitBoard(0),
                white_queens: BitBoard(0),
                white_king: BitBoard(0),

                //black
                black_pawns: BitBoard(0),
                black_knights: BitBoard(0),
                black_bishops: BitBoard(0),
                black_rooks: BitBoard(0),
                black_queens: BitBoard(0),
                black_king: BitBoard(0),
            }
        }
    }

    #[derive(Debug)]
    pub struct Board {
        bitboards: BitBoards,
        prev_bitboards: Option<BitBoards>,
        turn: Turn,
    }

    impl Board {
        pub fn new() -> Board {
            return Board {
                bitboards: BitBoards::default(),
                prev_bitboards: None,
                turn: Turn::WHITE,
            };
        }
        fn reset_to_default(&mut self) {
            self.bitboards = BitBoards::default();
            self.turn = Turn::WHITE;
        }
        fn reset_to_zero(&mut self) {
            self.bitboards = BitBoards::zero();
            self.turn = Turn::WHITE;
        }
        fn get_all_white_bits(&self) -> BitBoard {
            return BitBoard(
                self.bitboards.white_pawns.0
                    | self.bitboards.white_knights.0
                    | self.bitboards.white_bishops.0
                    | self.bitboards.white_rooks.0
                    | self.bitboards.white_queens.0
                    | self.bitboards.white_king.0,
            );
        }
        fn get_all_black_bits(&self) -> BitBoard {
            return BitBoard(
                self.bitboards.white_pawns.0
                    | self.bitboards.black_knights.0
                    | self.bitboards.black_bishops.0
                    | self.bitboards.black_rooks.0
                    | self.bitboards.black_queens.0
                    | self.bitboards.black_king.0,
            );
        }
        fn get_all_bits(&self) -> BitBoard {
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
        }

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
                let mut file = 0;
                for char in rows[rank].chars() {
                    if let Some(number) = char.to_digit(10) {
                        file += number;
                    } else {
                        let square_index = (7 - rank as u32) * 8 + file;
                        let bit = generate_bitboard_with_one_piece(square_index as i32);
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
                            bitboard.0 = bit.0 | bitboard.0;
                            file += 1;
                        }
                    }
                }
            }
        }
    }

    fn generate_bitboard_with_one_piece(index: i32) -> BitBoard {
        return BitBoard(1u64 << index);
    }

    fn extract_bits(bitboard: &BitBoard) -> Vec<u32> {
        let mut res = Vec::new();
        let mut bb = bitboard.0;
        while bb != 0 {
            let lsb = bb.trailing_zeros();
            res.push(lsb);
            bb &= bb - 1;
        }
        res
    }
}

#[cfg(test)]
mod test {

    use crate::chess::*;

    #[test]
    fn hi() {
        let mut board = Board::new();

        board.load_from_fen("r2q1rk1/pp1b1ppp/2np1n2/2p1p3/2P1P3/2NP1N2/PP1B1PPP/R2Q1RK1 w");

        println!("{:?}", board);
    }
}
