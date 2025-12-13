#[allow(unused_variables)]
#[allow(dead_code)]

pub mod chess {

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
    pub const KING_ATTACK_TABLE: [u64; 64] = [
        0x0000000000000302,
        0x0000000000000705,
        0x0000000000000e0a,
        0x0000000000001c14,
        0x0000000000003828,
        0x0000000000007050,
        0x000000000000e0a0,
        0x000000000000c040,
        0x0000000000030203,
        0x0000000000070507,
        0x00000000000e0a0e,
        0x00000000001c141c,
        0x0000000000382838,
        0x0000000000705070,
        0x0000000000e0a0e0,
        0x0000000000c040c0,
        0x0000000003020300,
        0x0000000007050700,
        0x000000000e0a0e00,
        0x000000001c141c00,
        0x0000000038283800,
        0x0000000070507000,
        0x00000000e0a0e000,
        0x00000000c040c000,
        0x0000000302030000,
        0x0000000705070000,
        0x0000000e0a0e0000,
        0x0000001c141c0000,
        0x0000003828380000,
        0x0000007050700000,
        0x000000e0a0e00000,
        0x000000c040c00000,
        0x0000030203000000,
        0x0000070507000000,
        0x00000e0a0e000000,
        0x00001c141c000000,
        0x0000382838000000,
        0x0000705070000000,
        0x0000e0a0e0000000,
        0x0000c040c0000000,
        0x0003020300000000,
        0x0007050700000000,
        0x000e0a0e00000000,
        0x001c141c00000000,
        0x0038283800000000,
        0x0070507000000000,
        0x00e0a0e000000000,
        0x00c040c000000000,
        0x0302030000000000,
        0x0705070000000000,
        0x0e0a0e0000000000,
        0x1c141c0000000000,
        0x3828380000000000,
        0x7050700000000000,
        0xe0a0e00000000000,
        0xc040c00000000000,
        0x0203000000000000,
        0x0507000000000000,
        0x0a0e000000000000,
        0x141c000000000000,
        0x2838000000000000,
        0x5070000000000000,
        0xa0e0000000000000,
        0x40c0000000000000,
    ];

    const RANK_4: u64 = 0x00000000FF000000;
    const RANK_5: u64 = 0x000000FF00000000;
    const RANK_2: u64 = 0x000000000000FF00;
    const RANK_7: u64 = 0x00FF000000000000;
    const RANK_8: u64 = 0xFF00000000000000;
    const RANK_1: u64 = 0x00000000000000FF;

    const FILE_A: u64 = 0x0101010101010101;
    const FILE_H: u64 = 0x8080808080808080;

    #[derive(Debug, Clone)]
    pub enum PieceType {
        WhitePawn,
        WhiteKnight,
        WhiteBishop,
        WhiteRook,
        WhiteQueen,
        WhiteKing,
        BlackPawn,
        BlackKnight,
        BlackBishop,
        BlackRook,
        BlackQueen,
        BlackKing,
    }

    impl Copy for PieceType {}

    #[derive(Debug, Clone)]
    pub struct Move {
        from: u64,
        to: u64,
        piece_type: PieceType,
        captured_piece: Option<PieceType>,
        capture: bool,
    }

    impl Copy for Move {}

    impl Move {
        pub fn new(
            from: u64,
            to: u64,
            capture: bool,
            piece_type: PieceType,
            captured_piece: Option<PieceType>,
        ) -> Move {
            return Move {
                from,
                to,
                capture,
                piece_type,
                captured_piece,
            };
        }
    }

    #[derive(Copy, Clone, Debug)]
    struct BitBoard(u64);

    #[derive(Debug)]
    pub enum Turn {
        WHITE,
        BLACK,
    }

    #[derive(Debug, Copy, Clone)]
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
        undo: Option<Move>,
        turn: Turn,
        occupied: BitBoard,
    }

    impl Board {
        pub fn new() -> Board {
            return Board {
                bitboards: BitBoards::default(),
                undo: None,
                turn: Turn::WHITE,
                occupied: BitBoard(RANK_1 | RANK_2 | RANK_7 | RANK_8),
            };
        }
        fn reset_to_default(&mut self) {
            self.bitboards = BitBoards::default();
            self.occupied = BitBoard(RANK_1 | RANK_2 | RANK_7 | RANK_8);
            self.turn = Turn::WHITE;
        }
        fn reset_to_zero(&mut self) {
            self.bitboards = BitBoards::zero();
            self.occupied = BitBoard(0);
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

        pub fn switch_turn(&mut self) {
            self.turn = match self.turn {
                Turn::BLACK => Turn::WHITE,
                Turn::WHITE => Turn::BLACK,
            }
        } //

        pub fn generate_knight_moves(&self, moves: &mut Vec<Move>) {
            // let mut moves = Vec::new();
            let enemy_bits = self.get_enemy_pieces().0;
            let allay_bits = self.get_allay_pieces().0;

            let (mut knights, piece_type) = match self.turn {
                Turn::WHITE => (self.bitboards.white_knights.0, PieceType::WhiteKnight),
                Turn::BLACK => (self.bitboards.black_knights.0, PieceType::BlackKnight),
            };

            while knights != 0 {
                let from = knights.trailing_zeros() as u64;
                knights &= knights - 1;
                let mut attacks = KNIGHTS_ATTACK_TABLE.get(from as usize).unwrap() & !allay_bits;

                while attacks != 0 {
                    let to = attacks.trailing_zeros() as u64;
                    attacks &= attacks - 1;
                    let capture = (enemy_bits & to) != 0;
                    moves.push(Move::new(from.into(), to.into(), capture, piece_type, None));
                }
            }
        } //

        pub fn generate_king_moves(&self, moves: &mut Vec<Move>) {
            // let mut moves = Vec::new();
            let enemy_bits = self.get_enemy_pieces().0;
            let allay_bits = self.get_allay_pieces().0;

            let (king, piece_type) = match self.turn {
                Turn::WHITE => (self.bitboards.white_king.0, PieceType::WhiteKing),
                Turn::BLACK => (self.bitboards.black_king.0, PieceType::BlackKing),
            };

            let from = king.trailing_zeros() as u64;
            let mut attacks = KNIGHTS_ATTACK_TABLE.get(from as usize).unwrap() & !allay_bits;

            while attacks != 0 {
                let to = attacks.trailing_zeros() as u64;
                attacks &= attacks - 1;
                let capture = (enemy_bits & to) != 0;
                moves.push(Move::new(from.into(), to.into(), capture, piece_type, None));
            }
        } //

        pub fn generate_white_pawns_moves(&self, moves: &mut Vec<Move>) {
            let blockers = self.get_all_white_bits().0 | self.get_all_black_bits().0;
            // let pawn_squares = &self.bitboards.white_pawns;

            let enemy_pieces_bb = self.get_all_black_bits();

            let mut add = |from: u64, to: u64, capture: bool| {
                moves.push(Move::new(
                    from.into(),
                    to.into(),
                    capture,
                    PieceType::WhitePawn,
                    None,
                ));
            };

            let mut pawns = self.bitboards.white_pawns.0;

            while pawns != 0 {
                let from = pawns.trailing_zeros() as u64;
                pawns &= pawns - 1;

                let pawn_bb = 1u64 << from;

                // single and double jump
                if from < 55 && (blockers & 1u64 << from + 8) == 0 {
                    add(from.into(), (from + 8).into(), false);
                    if ((from as u64 & RANK_2) != 0) && (blockers & 1u64 << (from + 16)) == 0 {
                        add(from.into(), (from + 16).into(), false);
                    }
                }

                // attack left
                if from + 7 < 63
                    && ((enemy_pieces_bb.0 & 1u64 << (from + 7)) != 0)
                    && ((pawn_bb & FILE_A) == 0)
                {
                    add(from.into(), from + 7, true);
                }
                // attack right
                if from + 9 < 63
                    && (enemy_pieces_bb.0 & 1u64 << (from + 9)) != 0
                    && ((pawn_bb & FILE_H) == 0)
                {
                    add(from.into(), from + 9, true);
                }
            }
        } //

        pub fn generate_black_pawns_moves(&self, moves: &mut Vec<Move>) {
            let blockers = self.get_all_white_bits().0 | self.get_all_black_bits().0;
            let pawn_squares = &self.bitboards.black_pawns;

            let enemy_pieces_bb = self.get_all_white_bits();

            let mut add = |from: u64, to: u64, capture: bool| {
                moves.push(Move::new(
                    from.into(),
                    to.into(),
                    capture,
                    PieceType::BlackPawn,
                    None,
                ));
            };

            let mut pawns = self.bitboards.black_pawns.0;

            while pawns != 0 {
                let from = pawns.trailing_zeros() as u64;
                pawns &= pawns - 1;

                let pawn_bb = 1u64 << from;

                // single and double jump
                if from >= 8 && (blockers & 1u64 << (from - 8)) == 0 {
                    add(from.into(), (from - 8).into(), false);
                    if ((from as u64 & RANK_7) != 0) && (blockers & 1u64 << (from - 16)) == 0 {
                        add(from.into(), (from - 16).into(), false);
                    }
                }

                // attack left
                if from - 7 > 0
                    && ((enemy_pieces_bb.0 & 1u64 << (from - 7)) != 0)
                    && ((pawn_bb & FILE_H) == 0)
                {
                    add(from.into(), from - 7, true);
                }
                // attack right
                if from - 9 > 0
                    && (enemy_pieces_bb.0 & 1u64 << (from - 9)) != 0
                    && ((pawn_bb & FILE_A) == 0)
                {
                    add(from.into(), from - 9, true);
                }
            }
        } //

        pub fn generate_rook_moves(&self, moves: &mut Vec<Move>) {
            let allay_bits = &self.get_allay_pieces();
            let enemy_bits = &self.get_enemy_pieces();
            let all_bits = &self.occupied.0;

            let (mut rooks, piece_type) = match self.turn {
                Turn::WHITE => (self.bitboards.white_rooks.0, PieceType::WhiteRook),
                Turn::BLACK => (self.bitboards.black_rooks.0, PieceType::BlackRook),
            };
            let mut add = |from: u64, to: u64, capture: bool| {
                moves.push(Move::new(from.into(), to.into(), capture, piece_type, None));
            };

            while rooks != 0 {
                let from = rooks.trailing_zeros() as u64;
                rooks &= rooks - 1;

                // North
                if from < 56 {
                    for to in ((from + 8)..=63).step_by(8) {
                        let to_mask = 1u64 << (to);
                        add(from, to, (enemy_bits.0 & to_mask) != 0);
                        if all_bits & to_mask != 0 {
                            break;
                        };
                    }
                };
                // South
                if from > 7 {
                    for to in (0..=(from - 8)).rev().step_by(8) {
                        let to_mask = 1u64 << (to);
                        add(from, to, (enemy_bits.0 & to_mask) != 0);
                        if all_bits & to_mask != 0 {
                            break;
                        };
                    }
                };

                // East
                if from % 8 != 7 {
                    let mut to = from + 1;
                    while to % 8 != 0 {
                        let to_mask = 1u64 << (to);
                        add(from, to, (enemy_bits.0 & to_mask) != 0);
                        if all_bits & to_mask != 0 {
                            break;
                        };
                        to += 1;
                    }
                };
                // West
                if from % 8 != 0 {
                    let mut to = from - 1;
                    while to % 8 != 7 {
                        let to_mask = 1u64 << (to);
                        add(from, to, (enemy_bits.0 & to_mask) != 0);
                        if all_bits & to_mask != 0 {
                            break;
                        };
                        if to > 0 {
                            to -= 1;
                        }
                    }
                };
            }
        } //

        pub fn generate_rook_moves_by_square(&self, from: u64) -> u64 {
            let mut attacks = 0u64;
            let occupied = self.occupied.0;

            // north
            let mut sq = from + 8;
            while sq < 64 {
                let bb = 1u64 << sq;
                attacks |= bb;
                if occupied & bb != 0 {
                    break;
                }
                sq += 8;
            }

            // south
            let mut sq = from as i32 - 8;
            while sq >= 0 {
                let bb = 1u64 << sq;
                attacks |= bb;
                if occupied & bb != 0 {
                    break;
                }
                sq -= 8;
            }

            // east
            let mut sq = from + 1;
            while sq % 8 != 0 {
                let bb = 1u64 << sq;
                attacks |= bb;
                if occupied & bb != 0 {
                    break;
                }
                sq += 1;
            }

            // west
            let mut sq = from as i32 - 1;
            while sq >= 0 && sq % 8 != 7 {
                let bb = 1u64 << sq;
                attacks |= bb;
                if occupied & bb != 0 {
                    break;
                }
                sq -= 1;
            }

            attacks
        } //

        pub fn generate_bishop_moves(&self, moves: &mut Vec<Move>) {
            let allay_bits = &self.get_allay_pieces();
            let enemy_bits = &self.get_enemy_pieces();
            let all_bits = &self.occupied.0;

            let (mut bishops, piece_type) = match self.turn {
                Turn::WHITE => (self.bitboards.white_bishops.0, PieceType::WhiteBishop),
                Turn::BLACK => (self.bitboards.black_bishops.0, PieceType::BlackBishop),
            };

            let mut add = |from: u64, to: u64, capture: bool| {
                moves.push(Move::new(from.into(), to.into(), capture, piece_type, None));
            };

            while bishops != 0 {
                let from = bishops.trailing_zeros() as u64;
                bishops &= bishops - 1;

                // North East
                let mut to = from + 9;
                while to <= 63 && to % 8 != 0 {
                    let to_mask = 1u64 << (to);
                    if allay_bits.0 & to_mask != 0 {
                        break;
                    }
                    add(from, to, (enemy_bits.0 & to_mask) != 0);
                    if all_bits & to_mask != 0 {
                        break;
                    };
                    to += 9;
                }
                // North West
                let mut to = from + 7;
                while to <= 63 && to % 8 != 7 {
                    let to_mask = 1u64 << (to);
                    if allay_bits.0 & to_mask != 0 {
                        break;
                    }
                    add(from, to, (enemy_bits.0 & to_mask) != 0);
                    if all_bits & to_mask != 0 {
                        break;
                    };
                    to += 7;
                }
                // South East
                let mut to = from - 7;
                while to % 8 != 0 {
                    let to_mask = 1u64 << (to);
                    if allay_bits.0 & to_mask != 0 {
                        break;
                    }
                    add(from, to, (enemy_bits.0 & to_mask) != 0);
                    if all_bits & to_mask != 0 {
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
                    let to_mask = 1u64 << (to);
                    if allay_bits.0 & to_mask != 0 {
                        break;
                    }
                    add(from, to, (enemy_bits.0 & to_mask) != 0);
                    if all_bits & to_mask != 0 {
                        break;
                    };
                    if to > 9 {
                        to -= 9;
                    } else {
                        break;
                    }
                }
            }
        } //

        pub fn generate_bishop_moves_by_square(&self, from: u64) -> u64 {
            let mut attacks = 0u64;
            let occupied = self.occupied.0;
            // NE
            let mut sq = from + 9;
            while sq < 64 && sq % 8 != 0 {
                let bb = 1u64 << sq;
                attacks |= bb;
                if occupied & bb != 0 {
                    break;
                }
                sq += 9;
            }

            // NW
            let mut sq = from + 7;
            while sq < 64 && sq % 8 != 7 {
                let bb = 1u64 << sq;
                attacks |= bb;
                if occupied & bb != 0 {
                    break;
                }
                sq += 7;
            }

            // SE
            let mut sq = from as i32 - 7;
            while sq >= 0 && sq % 8 != 0 {
                let bb = 1u64 << sq;
                attacks |= bb;
                if occupied & bb != 0 {
                    break;
                }
                sq -= 7;
            }

            // SW
            let mut sq = from as i32 - 9;
            while sq >= 0 && sq % 8 != 7 {
                let bb = 1u64 << sq;
                attacks |= bb;
                if occupied & bb != 0 {
                    break;
                }
                sq -= 9;
            }

            attacks
        } //

        pub fn generate_queen_moves(&self, moves: &mut Vec<Move>) {
            let allay_bits = &self.get_allay_pieces();
            let enemy_bits = &self.get_enemy_pieces();
            let occupied = self.occupied.0;

            let (mut queen_bits, piece_type) = match self.turn {
                Turn::WHITE => (self.bitboards.white_queens.0, PieceType::WhiteQueen),
                Turn::BLACK => (self.bitboards.black_queens.0, PieceType::BlackQueen),
            };

            let mut add = |from: u64, to: u64, capture: bool| {
                moves.push(Move::new(from.into(), to.into(), capture, piece_type, None));
            };

            while queen_bits != 0 {
                let from = queen_bits.trailing_zeros() as u64;
                queen_bits &= queen_bits - 1;

                // BISHOPS
                // North East
                let mut to = from + 9;
                while to <= 63 && to % 8 != 0 {
                    let to_mask = 1u64 << (to);
                    if allay_bits.0 & to_mask != 0 {
                        break;
                    }
                    add(from, to, (enemy_bits.0 & to_mask) != 0);
                    if occupied & to_mask != 0 {
                        break;
                    };
                    to += 9;
                }
                // North West
                let mut to = from + 7;
                while to <= 63 && to % 8 != 7 {
                    let to_mask = 1u64 << (to);
                    if allay_bits.0 & to_mask != 0 {
                        break;
                    }
                    add(from, to, (enemy_bits.0 & to_mask) != 0);
                    if occupied & to_mask != 0 {
                        break;
                    };
                    to += 7;
                }
                // South East
                if from >= 7 {
                    let mut to = from - 7;
                    while to % 8 != 0 {
                        let to_mask = 1u64 << (to);
                        if allay_bits.0 & to_mask != 0 {
                            break;
                        }
                        add(from, to, (enemy_bits.0 & to_mask) != 0);
                        if occupied & to_mask != 0 {
                            break;
                        };
                        if to > 7 {
                            to -= 7;
                        } else {
                            break;
                        }
                    }
                };
                // South West
                if from >= 9 {
                    let mut to = from - 9;
                    while to > 0 && to % 8 != 7 {
                        let to_mask = 1u64 << (to);
                        if allay_bits.0 & to_mask != 0 {
                            break;
                        }
                        add(from, to, (enemy_bits.0 & to_mask) != 0);
                        if occupied & to_mask != 0 {
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
                        let to_mask = 1u64 << (to);
                        if allay_bits.0 & to_mask != 0 {
                            break;
                        }
                        add(from, to, (enemy_bits.0 & to_mask) != 0);
                        if occupied & to_mask != 0 {
                            break;
                        };
                    }
                };
                // South
                if from > 7 {
                    for to in (0..=(from - 8)).rev().step_by(8) {
                        let to_mask = 1u64 << (to);
                        if allay_bits.0 & to_mask != 0 {
                            break;
                        }
                        add(from, to, (enemy_bits.0 & to_mask) != 0);
                        if occupied & to_mask != 0 {
                            break;
                        };
                    }
                };

                // East
                if from % 8 != 7 {
                    let mut to = from + 1;
                    while to < 64 && to % 8 != 0 {
                        let to_mask = 1u64 << (to);
                        if allay_bits.0 & to_mask != 0 {
                            break;
                        }
                        add(from, to, (enemy_bits.0 & to_mask) != 0);
                        if occupied & to_mask != 0 {
                            break;
                        };
                        to += 1;
                    }
                };

                // West
                if from % 8 != 0 {
                    let mut to = from - 1;
                    loop {
                        let to_mask = 1u64 << (to);
                        if allay_bits.0 & to_mask != 0 {
                            break;
                        }
                        add(from, to, (enemy_bits.0 & to_mask) != 0);
                        if occupied & to_mask != 0 {
                            break;
                        }
                        if to % 8 == 0 {
                            break;
                        }
                        to -= 1;
                    }
                };
            }
        } //

        // pub fn generate_king_moves(&self, moves: &mut Vec<Move>) {
        //     let offsets: [i32; 8] = [8, -8, 1, -1, 9, 7, -7, -9];

        //     let allay_bits = &self.get_allay_pieces();
        //     let enemy_bits = &self.get_enemy_pieces();
        //     let occupied = self.occupied.0;

        //     let (king_bits, piece_type) = match self.turn {
        //         Turn::WHITE => (&self.bitboards.white_king, PieceType::WhiteKing),
        //         Turn::BLACK => (&self.bitboards.black_king, PieceType::BlackKing),
        //     };

        //     let mut add = |from: u64, to: u64, capture: bool| {
        //         moves.push(Move::new(from.into(), to.into(), capture, piece_type, None));
        //     };

        //     let from = king_bits.0.trailing_zeros() as u64;
        //     for offset in offsets {
        //         let to = (from as i32) + offset;
        //         if to < 0 || to > 63 {
        //             continue;
        //         };
        //         let from_file = from % 8;
        //         let to_file = to % 8;
        //         if (((from as i32) - to) as i64).abs() > 1 {
        //             continue;
        //         };

        //         let to_mask = 1u64 << (to as u64);

        //         if allay_bits.0 & to_mask != 0 {
        //             continue;
        //         }
        //         add(from, to as u64, (enemy_bits.0 & to_mask) != 0);
        //     }
        // } //

        pub fn generate_king_moves_by_square(&self, square: u64) -> u64 {
            let mut attacks = 0u64;
            let rank = square / 8;
            let file = square % 8;

            for (dr, df) in [
                (1, 0),
                (-1, 0),
                (0, 1),
                (0, -1),
                (1, 1),
                (1, -1),
                (-1, 1),
                (-1, -1),
            ] {
                let r = rank as i32 + dr;
                let f = file as i32 + df;
                if r >= 0 && r < 8 && f >= 0 && f < 8 {
                    attacks |= 1u64 << (r * 8 + f);
                }
            }

            attacks
        } //

        #[inline(always)]
        pub fn is_check_by_bishop(&self, king_bb: u64, sliders: u64) -> bool {
            let occ = self.occupied.0;
            let k = king_bb.trailing_zeros() as i32;

            const DIRS: [i32; 4] = [9, 7, -7, -9];

            for d in DIRS {
                let mut sq = k + d;
                while sq >= 0 && sq < 64 && ((sq ^ k) & 7).abs() <= 1 {
                    let bb = 1u64 << sq;
                    if occ & bb != 0 {
                        return (sliders & bb) != 0;
                    }
                    sq += d;
                }
            }
            false
        }

        #[inline(always)]
        pub fn is_check_by_rook(&self, king_bb: u64, sliders: u64) -> bool {
            let occ = self.occupied.0;
            let k = king_bb.trailing_zeros() as i32;
            let rank = k & 56;

            // North / South
            let mut sq = k + 8;
            while sq < 64 {
                let bb = 1u64 << sq;
                if occ & bb != 0 {
                    return sliders & bb != 0;
                }
                sq += 8;
            }

            let mut sq = k - 8;
            while sq >= 0 {
                let bb = 1u64 << sq;
                if occ & bb != 0 {
                    return sliders & bb != 0;
                }
                sq -= 8;
            }

            // East / West
            let mut sq = k + 1;
            while sq < 64 && (sq & 56) == rank {
                let bb = 1u64 << sq;
                if occ & bb != 0 {
                    return sliders & bb != 0;
                }
                sq += 1;
            }

            let mut sq = k - 1;
            while sq >= 0 && (sq & 56) == rank {
                let bb = 1u64 << sq;
                if occ & bb != 0 {
                    return sliders & bb != 0;
                }
                sq -= 1;
            }

            false
        }

        pub fn generate_pesudo_moves(&self, mut moves: &mut Vec<Move>) {
            self.generate_knight_moves(&mut moves);
            self.generate_bishop_moves(&mut moves);
            self.generate_rook_moves(&mut moves);
            self.generate_queen_moves(&mut moves);
            self.generate_king_moves(&mut moves);

            match self.turn {
                Turn::WHITE => self.generate_white_pawns_moves(&mut moves),
                Turn::BLACK => self.generate_black_pawns_moves(&mut moves),
            };
        } //

        pub fn generate_moves(&mut self) -> Vec<Move> {
            let mut pesudo_moves: Vec<Move> = Vec::new();
            let mut legal_moves: Vec<Move> = Vec::new();

            // println!("Started generating moves with FEN : {} " , self.to_fen());

            self.generate_pesudo_moves(&mut pesudo_moves);

            // println!("Passed pesudo generation");

            for mv in pesudo_moves {
                let old_bitboards = self.bitboards;
                self.make_move(mv);
                let is_illegal = self.is_king_in_check(match self.turn {
                    Turn::BLACK => Turn::WHITE,
                    Turn::WHITE => Turn::BLACK,
                });
                if !is_illegal {
                    legal_moves.push(mv);
                }
                self.bitboards = old_bitboards;
                self.switch_turn();
            }
            return legal_moves;
        }

        pub fn is_king_in_check(&self, turn: Turn) -> bool {
            let (king, enemy_king) = match turn {
                Turn::BLACK => (&self.bitboards.black_king, &self.bitboards.white_king),
                Turn::WHITE => (&self.bitboards.white_king, &self.bitboards.black_king),
            };

            let king_square = king.0.trailing_zeros() as u64;

            let enemy_rooks = match turn {
                Turn::BLACK => &self.bitboards.white_rooks.0,
                Turn::WHITE => &self.bitboards.black_rooks.0,
            };
            let enemy_queens = match turn {
                Turn::BLACK => &self.bitboards.black_queens.0,
                Turn::WHITE => &self.bitboards.black_queens.0,
            };
            let enemy_bishops = match turn {
                Turn::BLACK => &self.bitboards.white_bishops.0,
                Turn::WHITE => &self.bitboards.black_bishops.0,
            };
            let enemy_knights = match turn {
                Turn::BLACK => &self.bitboards.white_knights.0,
                Turn::WHITE => &self.bitboards.black_knights.0,
            };
            let enemy_pawns = match turn {
                Turn::BLACK => &self.bitboards.white_pawns.0,
                Turn::WHITE => &self.bitboards.black_pawns.0,
            };

            let is_attacked_by_knights =
                (KNIGHTS_ATTACK_TABLE.get(king_square as usize).unwrap() & enemy_knights) != 0;

            let is_attacked_by_king =
                (KING_ATTACK_TABLE.get(king_square as usize).unwrap() & enemy_knights) != 0;

            if is_attacked_by_knights {
                return true;
            }

            let is_attacked_by_bishops_or_queens =
                self.is_check_by_bishop(king.0, *enemy_bishops | *enemy_queens);

            if is_attacked_by_bishops_or_queens {
                return true;
            }

            let is_attacked_by_rooks_or_queens =
                self.is_check_by_rook(king.0, *enemy_rooks | *enemy_queens);

            if is_attacked_by_rooks_or_queens {
                return true;
            }

            let is_attacked_by_king =
                (self.generate_king_moves_by_square(king_square)) & enemy_king.0 != 0;

            if is_attacked_by_king {
                return true;
            }

            match turn {
                Turn::BLACK => {
                    if king.0 & FILE_A != 0 {
                        let attacking_pawns_mask = 1u64 << king_square - 7;
                        if enemy_pawns & attacking_pawns_mask != 0 {
                            return true;
                        }
                    } else if king.0 & FILE_H != 0 {
                        let attacking_pawns_mask = 1u64 << king_square - 9;
                        if enemy_pawns & attacking_pawns_mask != 0 {
                            return true;
                        }
                    } else {
                        let attacking_pawns_mask =
                            (1u64 << (king_square - 7)) | (1u64 << (king_square - 9));
                        if enemy_pawns & attacking_pawns_mask != 0 {
                            return true;
                        }
                    }
                }
                Turn::WHITE => {
                    if king.0 & FILE_A != 0 {
                        let attacking_pawns_mask = 1u64 << king_square + 9;
                        if enemy_pawns & attacking_pawns_mask != 0 {
                            return true;
                        }
                    } else if king.0 & FILE_H != 0 {
                        let attacking_pawns_mask = 1u64 << king_square + 7;
                        if enemy_pawns & attacking_pawns_mask != 0 {
                            return true;
                        }
                    } else {
                        let attacking_pawns_mask =
                            (1u64 << (king_square + 7)) | (1u64 << (king_square + 9));
                        if enemy_pawns & attacking_pawns_mask != 0 {
                            return true;
                        }
                    }
                }
            };

            return false;
        } //

        pub fn make_move(&mut self, mv: Move) {
            if let Some(piece_type) = &mv.captured_piece {
                match piece_type {
                    PieceType::WhitePawn => self.bitboards.white_pawns.0 &= !(1u64 << mv.to),
                    PieceType::WhiteKnight => self.bitboards.white_knights.0 &= !(1u64 << mv.to),
                    PieceType::WhiteBishop => self.bitboards.white_bishops.0 &= !(1u64 << mv.to),
                    PieceType::WhiteRook => self.bitboards.white_rooks.0 &= !(1u64 << mv.to),
                    PieceType::WhiteQueen => self.bitboards.white_queens.0 &= !(1u64 << mv.to),
                    PieceType::WhiteKing => self.bitboards.white_king.0 &= !(1u64 << mv.to),
                    PieceType::BlackPawn => self.bitboards.black_pawns.0 &= !(1u64 << mv.to),
                    PieceType::BlackKnight => self.bitboards.black_knights.0 &= !(1u64 << mv.to),
                    PieceType::BlackBishop => self.bitboards.black_bishops.0 &= !(1u64 << mv.to),
                    PieceType::BlackRook => self.bitboards.black_rooks.0 &= !(1u64 << mv.to),
                    PieceType::BlackQueen => self.bitboards.black_queens.0 &= !(1u64 << mv.to),
                    PieceType::BlackKing => self.bitboards.black_king.0 &= !(1u64 << mv.to),
                };
            };

            match mv.piece_type {
                PieceType::WhitePawn => {
                    self.bitboards.white_pawns.0 |= 1u64 << mv.to;
                    self.bitboards.white_pawns.0 &= !(1u64 << mv.from);
                }
                PieceType::WhiteKnight => {
                    self.bitboards.white_knights.0 |= 1u64 << mv.to;
                    self.bitboards.white_knights.0 &= !(1u64 << mv.from);
                }
                PieceType::WhiteBishop => {
                    self.bitboards.white_bishops.0 |= 1u64 << mv.to;
                    self.bitboards.white_bishops.0 &= !(1u64 << mv.from);
                }
                PieceType::WhiteRook => {
                    self.bitboards.white_rooks.0 |= 1u64 << mv.to;
                    self.bitboards.white_rooks.0 &= !(1u64 << mv.from);
                }
                PieceType::WhiteQueen => {
                    self.bitboards.white_queens.0 |= 1u64 << mv.to;
                    self.bitboards.white_queens.0 &= !(1u64 << mv.from);
                }
                PieceType::WhiteKing => {
                    self.bitboards.white_king.0 |= 1u64 << mv.to;
                    self.bitboards.white_king.0 &= !(1u64 << mv.from);
                }
                PieceType::BlackPawn => {
                    self.bitboards.black_pawns.0 |= 1u64 << mv.to;
                    self.bitboards.black_pawns.0 &= !(1u64 << mv.from);
                }
                PieceType::BlackKnight => {
                    self.bitboards.black_knights.0 |= 1u64 << mv.to;
                    self.bitboards.black_knights.0 &= !(1u64 << mv.from);
                }
                PieceType::BlackBishop => {
                    self.bitboards.black_bishops.0 |= 1u64 << mv.to;
                    self.bitboards.black_bishops.0 &= !(1u64 << mv.from);
                }
                PieceType::BlackRook => {
                    self.bitboards.black_rooks.0 |= 1u64 << mv.to;
                    self.bitboards.black_rooks.0 &= !(1u64 << mv.from);
                }
                PieceType::BlackQueen => {
                    self.bitboards.black_queens.0 |= 1u64 << mv.to;
                    self.bitboards.black_queens.0 &= !(1u64 << mv.from);
                }
                PieceType::BlackKing => {
                    self.bitboards.black_king.0 |= 1u64 << mv.to;
                    self.bitboards.black_king.0 &= !(1u64 << mv.from);
                }
            };
            self.turn = match self.turn {
                Turn::WHITE => Turn::BLACK,
                Turn::BLACK => Turn::WHITE,
            };
            self.occupied = self.get_all_bits();
            self.undo = Some(mv);
        } //

        pub fn undo_move(&mut self) {
            if let Some(mv) = self.undo {
                if let Some(piece_type) = mv.captured_piece {
                    match piece_type {
                        PieceType::WhitePawn => self.bitboards.white_pawns.0 |= 1u64 << mv.to,
                        PieceType::WhiteKnight => self.bitboards.white_knights.0 |= 1u64 << mv.to,
                        PieceType::WhiteBishop => self.bitboards.white_bishops.0 |= 1u64 << mv.to,
                        PieceType::WhiteRook => self.bitboards.white_rooks.0 |= 1u64 << mv.to,
                        PieceType::WhiteQueen => self.bitboards.white_queens.0 |= 1u64 << mv.to,
                        PieceType::WhiteKing => self.bitboards.white_king.0 |= 1u64 << mv.to,
                        PieceType::BlackPawn => self.bitboards.black_pawns.0 |= 1u64 << mv.to,
                        PieceType::BlackKnight => self.bitboards.black_knights.0 |= 1u64 << mv.to,
                        PieceType::BlackBishop => self.bitboards.black_bishops.0 |= 1u64 << mv.to,
                        PieceType::BlackRook => self.bitboards.black_rooks.0 |= 1u64 << mv.to,
                        PieceType::BlackQueen => self.bitboards.black_queens.0 |= 1u64 << mv.to,
                        PieceType::BlackKing => self.bitboards.black_king.0 |= 1u64 << mv.to,
                    };
                };

                self.turn = match self.turn {
                    Turn::WHITE => Turn::BLACK,
                    Turn::BLACK => Turn::WHITE,
                };

                match mv.piece_type {
                    PieceType::WhitePawn => {
                        self.bitboards.white_pawns.0 &= !(1u64 << mv.to);
                        self.bitboards.white_pawns.0 |= 1u64 << mv.from;
                    }
                    PieceType::WhiteKnight => {
                        self.bitboards.white_knights.0 &= !(1u64 << mv.to);
                        self.bitboards.white_knights.0 |= 1u64 << mv.from;
                    }
                    PieceType::WhiteBishop => {
                        self.bitboards.white_bishops.0 &= !(1u64 << mv.to);
                        self.bitboards.white_bishops.0 |= 1u64 << mv.from;
                    }
                    PieceType::WhiteRook => {
                        self.bitboards.white_rooks.0 &= !(1u64 << mv.to);
                        self.bitboards.white_rooks.0 |= 1u64 << mv.from;
                    }
                    PieceType::WhiteQueen => {
                        self.bitboards.white_queens.0 &= !(1u64 << mv.to);
                        self.bitboards.white_queens.0 |= 1u64 << mv.from;
                    }
                    PieceType::WhiteKing => {
                        self.bitboards.white_king.0 &= !(1u64 << mv.to);
                        self.bitboards.white_king.0 |= 1u64 << mv.from;
                    }
                    PieceType::BlackPawn => {
                        self.bitboards.black_pawns.0 &= !(1u64 << mv.to);
                        self.bitboards.black_pawns.0 |= 1u64 << mv.from;
                    }
                    PieceType::BlackKnight => {
                        self.bitboards.black_knights.0 &= !(1u64 << mv.to);
                        self.bitboards.black_knights.0 |= 1u64 << mv.from;
                    }
                    PieceType::BlackBishop => {
                        self.bitboards.black_bishops.0 &= !(1u64 << mv.to);
                        self.bitboards.black_bishops.0 |= 1u64 << mv.from;
                    }
                    PieceType::BlackRook => {
                        self.bitboards.black_rooks.0 &= !(1u64 << mv.to);
                        self.bitboards.black_rooks.0 |= 1u64 << mv.from;
                    }
                    PieceType::BlackQueen => {
                        self.bitboards.black_queens.0 &= !(1u64 << mv.to);
                        self.bitboards.black_queens.0 |= 1u64 << mv.from;
                    }
                    PieceType::BlackKing => {
                        self.bitboards.black_king.0 &= !(1u64 << mv.to);
                        self.bitboards.black_king.0 |= 1u64 << mv.from;
                    }
                };

                self.undo = None;
            }
        } //
    }

    #[inline]
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
        for _ in 0..1_000_000 {
            // let mut moves: Vec<_> = Vec::new();
            let _ = board.generate_moves();
            count += 1;
        }
        let end = std::time::Instant::now();
        let duration = end.duration_since(start);
        println!(
            "Time took: {:?} seconds for {} moves",
            duration.as_secs_f64(),
            count
        );

        // println!("white queen Moves {:#?}", board.generate_moves());

        // println!("{:?}", board);
    }

    #[test]
    fn check_king_check() {
        let mut board = Board::new();
        board.load_from_fen("rnbqkbnr/ppppp1pp/5p2/7Q/4P3/8/PPPP1PPP/RNB1KBNR w");

        assert!(board.is_king_in_check(Turn::BLACK));
        assert!(!board.is_king_in_check(Turn::WHITE));

        board.load_from_fen("r2q1rk1/pp1b1ppp/2np1n2/2p1p3/2P1P3/2NP1N2/PP1B1PPP/R2Q1RK1 w");

        assert!(!board.is_king_in_check(Turn::BLACK));
        assert!(!board.is_king_in_check(Turn::WHITE));
    } //

    #[test]
    fn speed_test() {
        let mut board = Board::new();
        board.load_from_fen("rnbqkbnr/ppppp1pp/5p2/7Q/4P3/8/PPPP1PPP/RNB1KBNR w");

        let start = std::time::Instant::now();
        let mut count = 0;
        let mv = Move::new(8, 16, false, PieceType::WhitePawn, None);
        for _ in 0..1_000_000 {
            board.generate_moves();
            count += 1;
        }
        let end = std::time::Instant::now();
        let duration = end.duration_since(start);
        println!(
            "Time took: {:?} seconds for {} moves",
            duration.as_secs_f64(),
            count
        );
    }

    #[test]
    fn moves_making() {
        let mut board = Board::new();
        board.load_from_fen("rnbqkbnr/ppppp1pp/5p2/8/4P3/8/PPPP1PPP/RNBQKBNR w");
        let mv = Move::new(8, 16, false, PieceType::WhitePawn, None);
        board.make_move(mv);
        // println!("{}" , board.to_fen());
        assert!(
            board.to_fen() == String::from("rnbqkbnr/ppppp1pp/5p2/8/4P3/P7/1PPP1PPP/RNBQKBNR b")
        );
        let mv = Move::new(60, 53, false, PieceType::BlackKing, None);
        board.make_move(mv);
        assert!(
            board.to_fen() == String::from("rnbq1bnr/pppppkpp/5p2/8/4P3/P7/1PPP1PPP/RNBQKBNR w")
        );

        board.undo_move();
        println!("{}", board.to_fen());
        assert!(
            board.to_fen() == String::from("rnbqkbnr/ppppp1pp/5p2/8/4P3/P7/1PPP1PPP/RNBQKBNR b")
        );
    }
}
