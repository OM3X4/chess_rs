#[allow(unused_variables)]
#[allow(dead_code)]

mod chess {

    const KNIGHTS_ATTACK_TABLE: [u64; 64] = [
        0x0000000000020400,
        0x0000000000050800,
        0x00000000000a1100,
        0x0000000000142200,
        0x0000000000284400,
        0x0000000000508800,
        0x0000000000a01000,
        0x0000000000402000,
        0x0000000002040004,
        0x0000000005080008,
        0x000000000a110011,
        0x0000000014220022,
        0x0000000028440044,
        0x0000000050880088,
        0x00000000a0100010,
        0x0000000040200020,
        0x0000000204000402,
        0x0000000508000805,
        0x0000000a1100110a,
        0x0000001422002214,
        0x0000002844004428,
        0x0000005088008850,
        0x000000a0100010a0,
        0x0000004020002040,
        0x0000020400040200,
        0x0000050800080500,
        0x00000a1100110a00,
        0x0000142200221400,
        0x0000284400442800,
        0x0000508800885000,
        0x0000a0100010a000,
        0x0000402000204000,
        0x0002040004020000,
        0x0005080008050000,
        0x000a1100110a0000,
        0x0014220022140000,
        0x0028440044280000,
        0x0050880088500000,
        0x00a0100010a00000,
        0x0040200020400000,
        0x0204000402000000,
        0x0508000805000000,
        0x0a1100110a000000,
        0x1422002214000000,
        0x2844004428000000,
        0x5088008850000000,
        0xa0100010a0000000,
        0x4020002040000000,
        0x0400040200000000,
        0x0800080500000000,
        0x1100110a00000000,
        0x2200221400000000,
        0x4400442800000000,
        0x8800885000000000,
        0x100010a000000000,
        0x2000204000000000,
        0x0004020000000000,
        0x0008050000000000,
        0x00110a0000000000,
        0x0022140000000000,
        0x0044280000000000,
        0x0088500000000000,
        0x0010a00000000000,
        0x0020400000000000,
    ];

    const RANK_4: u64 = 0x00000000FF000000;
    const RANK_5: u64 = 0x000000FF00000000;
    const RANK_2: u64 = 0x000000000000FF00;
    const RANK_7: u64 = 0x00FF000000000000;
    const RANK_8: u64 = 0xFF00000000000000;
    const RANK_1: u64 = 0x00000000000000FF;

    const FILE_A: u64 = 0x0101010101010101;
    const FILE_H: u64 = 0x8080808080808080;

    #[derive(Debug)]
    pub struct Move {
        from: u64,
        to: u64,
        capture: bool,
    }

    impl Move {
        pub fn new(from: u64, to: u64, capture: bool) -> Move {
            return Move { from, to, capture };
        }
    }

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
                self.bitboards.black_pawns.0
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
                let mut file: u64 = 0;
                for char in rows[rank].chars() {
                    if let Some(number) = char.to_digit(10) {
                        file += number as u64;
                    } else {
                        let square_index = (7 - rank as u64) * 8 + file;
                        let bit = generate_bitboard_with_one_piece(square_index);
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

        fn get_enemy_pieces(&self) -> BitBoard {
            return match self.turn {
                Turn::WHITE => self.get_all_black_bits(),
                Turn::BLACK => self.get_all_white_bits(),
            };
        }
        fn get_allay_pieces(&self) -> BitBoard {
            return match self.turn {
                Turn::BLACK => self.get_all_black_bits(),
                Turn::WHITE => self.get_all_white_bits(),
            };
        }

        pub fn generate_knight_moves(&self) -> Vec<Move> {
            let mut moves = Vec::new();
            let knight_squares = match self.turn {
                Turn::WHITE => extract_bits(&self.bitboards.white_knights),
                Turn::BLACK => extract_bits(&self.bitboards.black_knights),
            };

            // let (enemy_pieces_bb, allay_pieces_bb) =
            //     (self.get_enemy_pieces(), self.get_allay_pieces());

            let enemy_bits = self.get_enemy_pieces().0;
            let allay_bits = self.get_allay_pieces().0;

            for from in knight_squares {
                let attacks = KNIGHTS_ATTACK_TABLE.get(from as usize).unwrap() & !allay_bits;
                for to in extract_bits(&BitBoard(attacks)) {
                    let capture = (enemy_bits & to) != 0;
                    moves.push(Move::new(from.into(), to.into(), capture));
                }
            }
            return moves;
        }

        pub fn generate_white_pawns_moves(&self) -> Vec<Move> {
            let mut moves = Vec::new();
            let blockers = self.get_all_white_bits().0 | self.get_all_black_bits().0;
            let pawn_squares = &self.bitboards.white_pawns;

            let enemy_pieces_bb = self.get_all_black_bits();

            let mut add = |from: u64, to: u64, capture: bool| {
                moves.push(Move::new(from.into(), to.into(), capture));
            };

            for from in extract_bits(&pawn_squares) {
                let pawn_bb = generate_bitboard_with_one_piece(from);

                // single and double jump
                if from < 55 && (blockers & generate_bitboard_with_one_piece(from + 8).0) == 0 {
                    add(from.into(), (from + 8).into(), false);
                    if ((from as u64 & RANK_2) != 0)
                        && (blockers & generate_bitboard_with_one_piece(from + 16).0) == 0
                    {
                        add(from.into(), (from + 16).into(), false);
                    }
                }

                // attack left
                if from + 7 < 63
                    && ((enemy_pieces_bb.0 & generate_bitboard_with_one_piece(from + 7).0) != 0)
                    && ((pawn_bb.0 & FILE_A) == 0)
                {
                    add(from.into(), from + 7, true);
                }
                // attack right
                if from + 9 < 63
                    && (enemy_pieces_bb.0 & generate_bitboard_with_one_piece(from + 9).0) != 0
                    && ((pawn_bb.0 & FILE_H) == 0)
                {
                    add(from.into(), from + 9, true);
                }
            }
            return moves;
        }

        pub fn generate_black_pawns_moves(&self) -> Vec<Move> {
            let mut moves = Vec::new();
            let blockers = self.get_all_white_bits().0 | self.get_all_black_bits().0;
            let pawn_squares = &self.bitboards.black_pawns;

            let enemy_pieces_bb = self.get_all_white_bits();

            let mut add = |from: u64, to: u64, capture: bool| {
                moves.push(Move::new(from.into(), to.into(), capture));
            };

            for from in extract_bits(&pawn_squares) {
                let pawn_bb = generate_bitboard_with_one_piece(from);

                // single and double jump
                if from >= 8 && (blockers & generate_bitboard_with_one_piece(from - 8).0) == 0 {
                    add(from.into(), (from - 8).into(), false);
                    if ((from as u64 & RANK_7) != 0)
                        && (blockers & generate_bitboard_with_one_piece(from - 16).0) == 0
                    {
                        add(from.into(), (from - 16).into(), false);
                    }
                }

                // attack left
                if from - 7 > 0
                    && ((enemy_pieces_bb.0 & generate_bitboard_with_one_piece(from - 7).0) != 0)
                    && ((pawn_bb.0 & FILE_H) == 0)
                {
                    add(from.into(), from - 7, true);
                }
                // attack right
                if from - 9 > 0
                    && (enemy_pieces_bb.0 & generate_bitboard_with_one_piece(from - 9).0) != 0
                    && ((pawn_bb.0 & FILE_A) == 0)
                {
                    add(from.into(), from - 9, true);
                }
            }
            return moves;
        }

        pub fn generate_rook_moves(&self) -> Vec<Move> {
            let mut moves = Vec::new();
            let allay_bits = &self.get_allay_pieces();
            let enemy_bits = &self.get_enemy_pieces();

            let mut add = |from: u64, to: u64, capture: bool| {
                moves.push(Move::new(from.into(), to.into(), capture));
            };

            let rooks = match self.turn {
                Turn::WHITE => &self.bitboards.white_rooks,
                Turn::BLACK => &self.bitboards.black_rooks,
            };

            for from in extract_bits(rooks) {
                // North
                if from < 56 {
                    for to in ((from + 8)..=63).step_by(8) {
                        let to_mask = generate_bitboard_with_one_piece(to);
                        if allay_bits.0 & to_mask.0 != 0 {
                            break;
                        }
                        add(from, to, (enemy_bits.0 & to_mask.0) != 0);
                        if self.get_all_bits().0 & to_mask.0 != 0 {
                            break;
                        };
                    }
                }
                // South
                if from > 7 {
                    for to in (0..=(from - 8)).rev().step_by(8) {
                        let to_mask = generate_bitboard_with_one_piece(to);
                        if allay_bits.0 & to_mask.0 != 0 {
                            break;
                        }
                        add(from, to, (enemy_bits.0 & to_mask.0) != 0);
                        if self.get_all_bits().0 & to_mask.0 != 0 {
                            break;
                        };
                    }
                }

                // East
                if from % 8 != 7 {
                    let mut to = from + 1;
                    while to % 8 != 0 {
                        let to_mask = generate_bitboard_with_one_piece(to);
                        if allay_bits.0 & to_mask.0 != 0 {
                            break;
                        }
                        add(from, to, (enemy_bits.0 & to_mask.0) != 0);
                        if self.get_all_bits().0 & to_mask.0 != 0 {
                            break;
                        };
                        to += 1;
                    }
                }

                // West
                if from % 8 != 0 {
                    let mut to = from - 1;
                    while to % 8 != 7 {
                        let to_mask = generate_bitboard_with_one_piece(to);
                        if allay_bits.0 & to_mask.0 != 0 {
                            break;
                        }
                        add(from, to, (enemy_bits.0 & to_mask.0) != 0);
                        if self.get_all_bits().0 & to_mask.0 != 0 {
                            break;
                        };
                        if to > 0 {
                            to -= 1;
                        }
                    }
                }
            }
            return moves;
        }

        pub fn generate_bishop_moves(&self) -> Vec<Move> {
            let mut moves = Vec::new();
            let allay_bits = &self.get_allay_pieces();
            let enemy_bits = &self.get_enemy_pieces();

            let mut add = |from: u64, to: u64, capture: bool| {
                moves.push(Move::new(from.into(), to.into(), capture));
            };

            let bishops = match self.turn {
                Turn::WHITE => &self.bitboards.white_bishops,
                Turn::BLACK => &self.bitboards.black_bishops,
            };
            for from in extract_bits(bishops) {
                // North East
                let mut to = from + 9;
                while to <= 63 && to % 8 != 0 {
                    let to_mask = generate_bitboard_with_one_piece(to);
                    if allay_bits.0 & to_mask.0 != 0 {
                        break;
                    }
                    add(from, to, (enemy_bits.0 & to_mask.0) != 0);
                    if self.get_all_bits().0 & to_mask.0 != 0 {
                        break;
                    };
                    to += 9;
                }
                // North West
                let mut to = from + 7;
                while to <= 63 && to % 8 != 7 {
                    let to_mask = generate_bitboard_with_one_piece(to);
                    if allay_bits.0 & to_mask.0 != 0 {
                        break;
                    }
                    add(from, to, (enemy_bits.0 & to_mask.0) != 0);
                    if self.get_all_bits().0 & to_mask.0 != 0 {
                        break;
                    };
                    to += 7;
                }
                // South East
                let mut to = from - 7;
                while to % 8 != 0 {
                    let to_mask = generate_bitboard_with_one_piece(to);
                    if allay_bits.0 & to_mask.0 != 0 {
                        break;
                    }
                    add(from, to, (enemy_bits.0 & to_mask.0) != 0);
                    if self.get_all_bits().0 & to_mask.0 != 0 {
                        break;
                    };
                    if to > 7 {
                        to -= 7;
                    } else {
                        break;
                    }
                }
                // South West
                let mut to = from - 9;
                while to > 0 && to % 8 != 7 {
                    let to_mask = generate_bitboard_with_one_piece(to);
                    if allay_bits.0 & to_mask.0 != 0 {
                        break;
                    }
                    add(from, to, (enemy_bits.0 & to_mask.0) != 0);
                    if self.get_all_bits().0 & to_mask.0 != 0 {
                        break;
                    };
                    if to > 9 {
                        to -= 9;
                    } else {
                        break;
                    }
                }
            }

            return moves;
        }

        pub fn generate_queen_moves(&self) -> Vec<Move> {
            let mut moves = Vec::new();
            let allay_bits = &self.get_allay_pieces();
            let enemy_bits = &self.get_enemy_pieces();
            let occupied = self.get_all_bits().0;

            let queen_bits = match self.turn {
                Turn::WHITE => &self.bitboards.white_queens,
                Turn::BLACK => &self.bitboards.black_queens,
            };

            let mut add = |from: u64, to: u64, capture: bool| {
                moves.push(Move::new(from.into(), to.into(), capture));
            };

            for from in extract_bits(queen_bits) {
                // BISHOPS
                // North East
                let mut to = from + 9;
                while to <= 63 && to % 8 != 0 {
                    let to_mask = generate_bitboard_with_one_piece(to);
                    if allay_bits.0 & to_mask.0 != 0 {
                        break;
                    }
                    add(from, to, (enemy_bits.0 & to_mask.0) != 0);
                    if self.get_all_bits().0 & to_mask.0 != 0 {
                        break;
                    };
                    to += 9;
                }
                // North West
                let mut to = from + 7;
                while to <= 63 && to % 8 != 7 {
                    let to_mask = generate_bitboard_with_one_piece(to);
                    if allay_bits.0 & to_mask.0 != 0 {
                        break;
                    }
                    add(from, to, (enemy_bits.0 & to_mask.0) != 0);
                    if self.get_all_bits().0 & to_mask.0 != 0 {
                        break;
                    };
                    to += 7;
                }
                // South East
                if from >= 7 {
                    let mut to = from - 7;
                    while to % 8 != 0 {
                        let to_mask = generate_bitboard_with_one_piece(to);
                        if allay_bits.0 & to_mask.0 != 0 {
                            break;
                        }
                        add(from, to, (enemy_bits.0 & to_mask.0) != 0);
                        if self.get_all_bits().0 & to_mask.0 != 0 {
                            break;
                        };
                        if to > 7 {
                            to -= 7;
                        } else {
                            break;
                        }
                    }
                }
                // South West
                if from >= 9 {
                    let mut to = from - 9;
                    while to > 0 && to % 8 != 7 {
                        let to_mask = generate_bitboard_with_one_piece(to);
                        if allay_bits.0 & to_mask.0 != 0 {
                            break;
                        }
                        add(from, to, (enemy_bits.0 & to_mask.0) != 0);
                        if self.get_all_bits().0 & to_mask.0 != 0 {
                            break;
                        };
                        if to > 9 {
                            to -= 9;
                        } else {
                            break;
                        }
                    }
                }

                // ROOKS
                // North
                if from < 56 {
                    for to in ((from + 8)..=63).step_by(8) {
                        let to_mask = generate_bitboard_with_one_piece(to);
                        if allay_bits.0 & to_mask.0 != 0 {
                            break;
                        }
                        add(from, to, (enemy_bits.0 & to_mask.0) != 0);
                        if self.get_all_bits().0 & to_mask.0 != 0 {
                            break;
                        };
                    }
                }
                // South
                if from > 7 {
                    for to in (0..=(from - 8)).rev().step_by(8) {
                        let to_mask = generate_bitboard_with_one_piece(to);
                        if allay_bits.0 & to_mask.0 != 0 {
                            break;
                        }
                        add(from, to, (enemy_bits.0 & to_mask.0) != 0);
                        if self.get_all_bits().0 & to_mask.0 != 0 {
                            break;
                        };
                    }
                }

                // East
                if from % 8 != 7 {
                    let mut to = from + 1;
                    while to < 64 && to % 8 != 0 {
                        let to_mask = generate_bitboard_with_one_piece(to);
                        if allay_bits.0 & to_mask.0 != 0 {
                            break;
                        }
                        add(from, to, (enemy_bits.0 & to_mask.0) != 0);
                        if self.get_all_bits().0 & to_mask.0 != 0 {
                            break;
                        };
                        to += 1;
                    }
                }

                // West
                if from % 8 != 0 {
                    let mut to = from - 1;
                    loop {
                        let to_mask = generate_bitboard_with_one_piece(to);
                        if allay_bits.0 & to_mask.0 != 0 {
                            break;
                        }
                        add(from, to, (enemy_bits.0 & to_mask.0) != 0);
                        if self.get_all_bits().0 & to_mask.0 != 0 {
                            break;
                        }
                        if to % 8 == 0 {
                            break;
                        }
                        to -= 1;
                    }
                }
            }

            return moves;
        }

        pub fn generate_king_moves(&self) -> Vec<Move> {
            let mut moves: Vec<Move> = Vec::new();

            let offsets: [i32; 8] = [8, -8, 1, -1, 9, 7, -7, -9];

            let allay_bits = &self.get_allay_pieces();
            let enemy_bits = &self.get_enemy_pieces();
            let occupied = self.get_all_bits().0;

            let king_bits = match self.turn {
                Turn::WHITE => &self.bitboards.white_king,
                Turn::BLACK => &self.bitboards.black_king,
            };

            let mut add = |from: u64, to: u64, capture: bool| {
                moves.push(Move::new(from.into(), to.into(), capture));
            };

            let from = extract_bits(&king_bits)[0];
            for offset in offsets {
                let to = (from as i32) + offset;
                if to < 0 || to > 63 { continue; };
                let from_file = from % 8;
                let to_file = to / 8;
                if (((from as i32) - to) as i64).abs() > 1 { continue; };

                let to_mask = generate_bitboard_with_one_piece(to as u64);

                if allay_bits.0 & to_mask.0 != 0 {
                    continue;
                }
                add(from , to as u64 , (enemy_bits.0 & to_mask.0) != 0);
            }

            return moves;
        }

        pub fn generate_moves(&self) -> Vec<Move> {
            let mut moves: Vec<Move> = Vec::new();
            moves.extend(self.generate_knight_moves());
            moves.extend(self.generate_bishop_moves());
            moves.extend(self.generate_rook_moves());
            moves.extend(self.generate_queen_moves());
            moves.extend(self.generate_king_moves());
            moves.extend(match self.turn {
                Turn::WHITE => self.generate_white_pawns_moves(),
                Turn::BLACK => self.generate_black_pawns_moves(),
            });
            moves
        }

    }

    fn generate_bitboard_with_one_piece(index: u64) -> BitBoard {
        return BitBoard(1u64 << index);
    }

    fn extract_bits(bitboard: &BitBoard) -> Vec<u64> {
        let mut res: Vec<u64> = Vec::new();
        let mut bb = bitboard.0;
        while bb != 0 {
            let lsb = bb.trailing_zeros();
            res.push(lsb as u64);
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

        let start = std::time::Instant::now();
        board.load_from_fen("r2q1rk1/pp1b1ppp/2np1n2/2p1p3/2P1P3/2NP1N2/PP1B1PPP/R2Q1RK1 w");
        let mut count = 0;
        for _ in 0..9_000_000 {
            let _ = board.generate_moves();
            count += 1;
        }
        let end = std::time::Instant::now();
        let duration = end.duration_since(start);
        println!("Time took: {:?} seconds for {} moves", duration.as_secs_f64(), count);

        // println!("white queen Moves {:#?}", board.generate_moves());

        // println!("{:?}", board);
    }
}
