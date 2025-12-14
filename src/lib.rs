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
    ]; //
    const KING_ATTACK_TABLE: [u64; 64] = [
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
    const BLACK_PAWN_ATTACKS: [u64; 64] = [
        0x0000000000000200,
        0x0000000000000500,
        0x0000000000000a00,
        0x0000000000001400,
        0x0000000000002800,
        0x0000000000005000,
        0x000000000000a000,
        0x0000000000004000,
        0x0000000000020000,
        0x0000000000050000,
        0x00000000000a0000,
        0x0000000000140000,
        0x0000000000280000,
        0x0000000000500000,
        0x0000000000a00000,
        0x0000000000400000,
        0x0000000002000000,
        0x0000000005000000,
        0x000000000a000000,
        0x0000000014000000,
        0x0000000028000000,
        0x0000000050000000,
        0x00000000a0000000,
        0x0000000040000000,
        0x0000000200000000,
        0x0000000500000000,
        0x0000000a00000000,
        0x0000001400000000,
        0x0000002800000000,
        0x0000005000000000,
        0x000000a000000000,
        0x0000004000000000,
        0x0000020000000000,
        0x0000050000000000,
        0x00000a0000000000,
        0x0000140000000000,
        0x0000280000000000,
        0x0000500000000000,
        0x0000a00000000000,
        0x0000400000000000,
        0x0002000000000000,
        0x0005000000000000,
        0x000a000000000000,
        0x0014000000000000,
        0x0028000000000000,
        0x0050000000000000,
        0x00a0000000000000,
        0x0040000000000000,
        0x0200000000000000,
        0x0500000000000000,
        0x0a00000000000000,
        0x1400000000000000,
        0x2800000000000000,
        0x5000000000000000,
        0xa000000000000000,
        0x4000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
    ];
    const WHITE_PAWN_ATTACKS: [u64; 64] = [
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000200,
        0x0000000000000500,
        0x0000000000000a00,
        0x0000000000001400,
        0x0000000000002800,
        0x0000000000005000,
        0x000000000000a000,
        0x0000000000004000,
        0x0000000000020000,
        0x0000000000050000,
        0x00000000000a0000,
        0x0000000000140000,
        0x0000000000280000,
        0x0000000000500000,
        0x0000000000a00000,
        0x0000000000400000,
        0x0000000002000000,
        0x0000000005000000,
        0x000000000a000000,
        0x0000000014000000,
        0x0000000028000000,
        0x0000000050000000,
        0x00000000a0000000,
        0x0000000040000000,
        0x0000000200000000,
        0x0000000500000000,
        0x0000000a00000000,
        0x0000001400000000,
        0x0000002800000000,
        0x0000005000000000,
        0x000000a000000000,
        0x0000004000000000,
        0x0000020000000000,
        0x0000050000000000,
        0x00000a0000000000,
        0x0000140000000000,
        0x0000280000000000,
        0x0000500000000000,
        0x0000a00000000000,
        0x0000400000000000,
        0x0002000000000000,
        0x0005000000000000,
        0x000a000000000000,
        0x0014000000000000,
        0x0028000000000000,
        0x0050000000000000,
        0x00a0000000000000,
        0x0040000000000000,
        0x0200000000000000,
        0x0500000000000000,
        0x0a00000000000000,
        0x1400000000000000,
        0x2800000000000000,
        0x5000000000000000,
        0xa000000000000000,
        0x4000000000000000,
    ];

    /// For each square:
    /// rank ∪ file ∪ diagonals (excluding the square itself)
    pub const SQUARE_RAYS: [u64; 64] = [
        /* A1 */ 0x81412111090503FE,
        /* B1 */ 0x02824222120A07FD,
        /* C1 */ 0x0404844424150EFB,
        /* D1 */ 0x08080888492A1CF7,
        /* E1 */ 0x10101011925438EF,
        /* F1 */ 0x2020212224A870DF,
        /* G1 */ 0x404142444850E0BF,
        /* H1 */ 0x8182848890A0C07F,
        /* A2 */ 0x412111090503FE01,
        /* B2 */ 0x824222120A07FD02,
        /* C2 */ 0x04844424150EFB04,
        /* D2 */ 0x080888492A1CF708,
        /* E2 */ 0x101011925438EF10,
        /* F2 */ 0x20212224A870DF20,
        /* G2 */ 0x4142444850E0BF40,
        /* H2 */ 0x82848890A0C07F80,
        /* A3 */ 0x2111090503FE0101,
        /* B3 */ 0x4222120A07FD0202,
        /* C3 */ 0x844424150EFB0404,
        /* D3 */ 0x0888492A1CF70808,
        /* E3 */ 0x1011925438EF1010,
        /* F3 */ 0x212224A870DF2020,
        /* G3 */ 0x42444850E0BF4040,
        /* H3 */ 0x848890A0C07F8080,
        /* A4 */ 0x11090503FE010101,
        /* B4 */ 0x22120A07FD020202,
        /* C4 */ 0x4424150EFB040404,
        /* D4 */ 0x88492A1CF7080808,
        /* E4 */ 0x11925438EF101010,
        /* F4 */ 0x2224A870DF202020,
        /* G4 */ 0x444850E0BF404040,
        /* H4 */ 0x8890A0C07F808080,
        /* A5 */ 0x090503FE01010101,
        /* B5 */ 0x120A07FD02020202,
        /* C5 */ 0x24150EFB04040404,
        /* D5 */ 0x492A1CF708080808,
        /* E5 */ 0x925438EF10101010,
        /* F5 */ 0x24A870DF20202020,
        /* G5 */ 0x4850E0BF40404040,
        /* H5 */ 0x90A0C07F80808080,
        /* A6 */ 0x0503FE0101010101,
        /* B6 */ 0x0A07FD0202020202,
        /* C6 */ 0x150EFB0404040404,
        /* D6 */ 0x2A1CF70808080808,
        /* E6 */ 0x5438EF1010101010,
        /* F6 */ 0xA870DF2020202020,
        /* G6 */ 0x50E0BF4040404040,
        /* H6 */ 0xA0C07F8080808080,
        /* A7 */ 0x03FE010101010101,
        /* B7 */ 0x07FD020202020202,
        /* C7 */ 0x0EFB040404040404,
        /* D7 */ 0x1CF7080808080808,
        /* E7 */ 0x38EF101010101010,
        /* F7 */ 0x70DF202020202020,
        /* G7 */ 0xE0BF404040404040,
        /* H7 */ 0xC07F808080808080,
        /* A8 */ 0xFE01010101010101,
        /* B8 */ 0xFD02020202020202,
        /* C8 */ 0xFB04040404040404,
        /* D8 */ 0xF708080808080808,
        /* E8 */ 0xEF10101010101010,
        /* F8 */ 0xDF20202020202020,
        /* G8 */ 0xBF40404040404040,
        /* H8 */ 0x7F80808080808080,
    ];
    pub const SQUARE_RAYS_WITH_SELF: [u64; 64] = {
        let mut arr = [0u64; 64];
        let mut i = 0;
        while i < 64 {
            arr[i] = SQUARE_RAYS[i] | (1u64 << i);
            i += 1;
        }
        arr
    };

    const RANK_4: u64 = 0x00000000FF000000;
    const RANK_5: u64 = 0x000000FF00000000;
    const RANK_2: u64 = 0x000000000000FF00;
    const RANK_7: u64 = 0x00FF000000000000;
    const RANK_8: u64 = 0xFF00000000000000;
    const RANK_1: u64 = 0x00000000000000FF;

    const FILE_A: u64 = 0x0101010101010101;
    const FILE_H: u64 = 0x8080808080808080;

    #[derive(Debug, Clone, PartialEq, Eq)]
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
        pub from: u64,
        pub to: u64,
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

    #[derive(Debug, Copy, Clone)]
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
            let mut attacks = KING_ATTACK_TABLE.get(from as usize).unwrap() & !allay_bits;

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
                    if (((1u64 << from) & RANK_2) != 0) && (blockers & 1u64 << (from + 16)) == 0 {
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
                    if (((1u64 << from) & RANK_7) != 0) && (blockers & (1u64 << (from - 16))) == 0 {
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
            let allay = self.get_allay_pieces().0;
            let enemy = self.get_enemy_pieces().0;

            let (mut rooks, piece_type) = match self.turn {
                Turn::WHITE => (self.bitboards.white_rooks.0, PieceType::WhiteRook),
                Turn::BLACK => (self.bitboards.black_rooks.0, PieceType::BlackRook),
            };

            let mut add = |from: u64, to: u64, capture: bool| {
                moves.push(Move::new(from, to, capture, piece_type, None));
            };

            while rooks != 0 {
                let from = rooks.trailing_zeros() as u64;
                rooks &= rooks - 1;

                /* ================= NORTH ================= */
                let mut to = from + 8;
                while to < 64 {
                    let bb = 1u64 << to;
                    if allay & bb != 0 {
                        break;
                    }
                    if enemy & bb != 0 {
                        add(from, to, true);
                        break;
                    }
                    add(from, to, false);
                    to += 8;
                }

                /* ================= SOUTH ================= */
                if from >= 8 {
                    let mut to = from - 8;
                    loop {
                        let bb = 1u64 << to;
                        if allay & bb != 0 {
                            break;
                        }
                        if enemy & bb != 0 {
                            add(from, to, true);
                            break;
                        }
                        add(from, to, false);
                        if to < 8 {
                            break;
                        }
                        to -= 8;
                    }
                }

                /* ================= EAST ================= */
                let mut to = from + 1;
                while to < 64 && to % 8 != 0 {
                    let bb = 1u64 << to;
                    if allay & bb != 0 {
                        break;
                    }
                    if enemy & bb != 0 {
                        add(from, to, true);
                        break;
                    }
                    add(from, to, false);
                    to += 1;
                }

                /* ================= WEST ================= */
                if from % 8 != 0 {
                    let mut to = from - 1;
                    loop {
                        let bb = 1u64 << to;
                        if allay & bb != 0 {
                            break;
                        }
                        if enemy & bb != 0 {
                            add(from, to, true);
                            break;
                        }
                        add(from, to, false);
                        if to % 8 == 0 {
                            break;
                        }
                        to -= 1;
                    }
                }
            }
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
                while to <= 63 && ((1u64 << to) & FILE_A) == 0 {
                    let to_mask = 1u64 << (to);
                    if all_bits & to_mask != 0 {
                        if allay_bits.0 & to_mask != 0 {
                            break;
                        }
                        add(from, to, true);
                        break;
                    } else {
                        add(from, to, false);
                    }
                    to += 9;
                }

                // North West
                let mut to = from + 7;
                while to <= 63 && ((1u64 << to) & FILE_H) == 0 {
                    let to_mask = 1u64 << (to);
                    if all_bits & to_mask != 0 {
                        if allay_bits.0 & to_mask != 0 {
                            break;
                        }
                        add(from, to, true);
                        break;
                    } else {
                        add(from, to, false);
                    }
                    to += 7;
                }

                // South East
                if from >= 7 {
                    let mut to = from - 7;
                    while ((1u64 << to) & FILE_A) == 0 {
                        let to_mask = 1u64 << (to);
                        if all_bits & to_mask != 0 {
                            if allay_bits.0 & to_mask != 0 {
                                break;
                            }
                            add(from, to, true);
                            break;
                        } else {
                            add(from, to, false);
                        }
                        if to >= 7 {
                            to -= 7;
                        } else {
                            break;
                        }
                    }
                }

                // South West
                if from >= 9 {
                    let mut to = from - 9;
                    while ((1u64 << to) & FILE_H) == 0 {
                        let to_mask = 1u64 << (to);
                        if all_bits & to_mask != 0 {
                            if allay_bits.0 & to_mask != 0 {
                                break;
                            }
                            add(from, to, true);
                            break;
                        } else {
                            add(from, to, false);
                        }
                        if to >= 9 {
                            to -= 9;
                        } else {
                            break;
                        }
                    }
                }
            }
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

        #[inline]
        pub fn is_check_by_bishop(&self, king_bb: u64, sliders: u64) -> bool {
            let occ = self.occupied.0;
            let k = king_bb.trailing_zeros() as i32;

            let mut all_locations = 0u64;

            // NE
            let mut sq = k + 9;
            while sq < 64 && ((1u64 << sq) & FILE_A) == 0 {
                let bb = 1u64 << sq;
                all_locations |= bb;
                if occ & bb != 0 {
                    if sliders & bb != 0 {
                        return true;
                    }
                    break;
                }
                sq += 9;
            }

            // NW
            let mut sq = k + 7;
            while sq < 64 && ((1u64 << sq) & FILE_H) == 0 {
                let bb = 1u64 << sq;
                all_locations |= bb;
                if occ & bb != 0 {
                    if sliders & bb != 0 {
                        return true;
                    }
                    break;
                }
                sq += 7;
            }

            // SE
            let mut sq = k - 7;
            while sq >= 0 && ((1u64 << sq) & FILE_A) == 0 {
                let bb = 1u64 << sq;
                all_locations |= bb;
                if occ & bb != 0 {
                    if sliders & bb != 0 {
                        return true;
                    }
                    break;
                }
                sq -= 7;
            }

            // SW
            let mut sq = k - 9;
            while sq >= 0 && ((1u64 << sq) & FILE_H) == 0 {
                let bb = 1u64 << sq;
                all_locations |= bb;
                if occ & bb != 0 {
                    if sliders & bb != 0 {
                        return true;
                    }
                    break;
                }
                sq -= 9;
            }

            // all_locations & sliders != 0
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
                    if sliders & bb != 0 {
                        return true;
                    }
                    break;
                }
                sq += 8;
            }

            let mut sq = k - 8;
            while sq >= 0 {
                let bb = 1u64 << sq;
                if occ & bb != 0 {
                    if sliders & bb != 0 {
                        return true;
                    }
                    break;
                }
                sq -= 8;
            }

            // East / West
            let mut sq = k + 1;
            while sq < 64 && (sq & 56) == rank {
                let bb = 1u64 << sq;
                if occ & bb != 0 {
                    if sliders & bb != 0 {
                        return true;
                    }
                    break;
                }
                sq += 1;
            }

            let mut sq = k - 1;
            while sq >= 0 && (sq & 56) == rank {
                let bb = 1u64 << sq;
                if occ & bb != 0 {
                    if sliders & bb != 0 {
                        return true;
                    }
                    break;
                }
                sq -= 1;
            }

            false
        } //

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

            self.generate_pesudo_moves(&mut pesudo_moves);

            let king_bb = match self.turn {
                Turn::WHITE => self.bitboards.white_king.0,
                Turn::BLACK => self.bitboards.black_king.0,
            };

            let king_type = match self.turn {
                Turn::WHITE => PieceType::WhiteKing,
                Turn::BLACK => PieceType::BlackKing,
            };

            let is_king_in_check_now = self.is_king_in_check(self.turn);

            for mv in pesudo_moves {
                // if !is_king_in_check_now {
                //     if (1u64 << mv.from) & SQUARE_RAYS[king_bb.trailing_zeros() as usize] == 0
                //         && mv.piece_type != king_type
                //     {
                //         legal_moves.push(mv);
                //         continue;
                //     }
                // } else {
                //     if (1u64 << mv.to) & SQUARE_RAYS[king_bb.trailing_zeros() as usize] == 0
                //         && mv.piece_type != king_type
                //     {
                //         continue;
                //     }
                // }

                let old_bitboards = self.bitboards;
                self.make_move(mv);
                self.switch_turn();
                let is_illegal = self.is_king_in_check(self.turn);
                if !is_illegal {
                    legal_moves.push(mv);
                }
                self.bitboards = old_bitboards;
            }
            return legal_moves;
        } //

        pub fn is_king_in_check(&self, turn: Turn) -> bool {
            let (king, enemy_king) = match turn {
                Turn::BLACK => (&self.bitboards.black_king.0, &self.bitboards.white_king.0),
                Turn::WHITE => (&self.bitboards.white_king.0, &self.bitboards.black_king.0),
            };

            let king_square = king.trailing_zeros() as u64;

            let enemy_rooks = match turn {
                Turn::BLACK => &self.bitboards.white_rooks.0,
                Turn::WHITE => &self.bitboards.black_rooks.0,
            };
            let enemy_queens = match turn {
                Turn::BLACK => &self.bitboards.white_queens.0,
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

            if is_attacked_by_knights {
                return true;
            }

            let is_attacked_by_king =
                (KING_ATTACK_TABLE.get(king_square as usize).unwrap() & enemy_king) != 0;

            if is_attacked_by_king {
                return true;
            }

            let is_attacked_by_bishops_or_queens =
                self.is_check_by_bishop(*king, *enemy_bishops | *enemy_queens);

            if is_attacked_by_bishops_or_queens {
                return true;
            }

            let is_attacked_by_rooks_or_queens =
                self.is_check_by_rook(*king, *enemy_rooks | *enemy_queens);

            if is_attacked_by_rooks_or_queens {
                return true;
            }

            let file = king_square % 8;

            match turn {
                Turn::BLACK => {
                    // White pawns attack DOWN (-7, -9)
                    if king_square >= 7 && file != 0 {
                        if enemy_pawns & (1u64 << (king_square - 7)) != 0 {
                            return true;
                        }
                    }
                    if king_square >= 9 && file != 7 {
                        if enemy_pawns & (1u64 << (king_square - 9)) != 0 {
                            return true;
                        }
                    }
                }
                Turn::WHITE => {
                    // Black pawns attack UP (+7, +9)
                    if king_square <= 56 && file != 7 {
                        if enemy_pawns & (1u64 << (king_square + 7)) != 0 {
                            return true;
                        }
                    }
                    if king_square <= 54 && file != 0 {
                        if enemy_pawns & (1u64 << (king_square + 9)) != 0 {
                            return true;
                        }
                    }
                }
            }

            return false;
        } //

        pub fn make_move(&mut self, mv: Move) {

            let mut captured_piece_type = None;
            if self.bitboards.white_pawns.0 & (1u64 << mv.to) != 0 {
                captured_piece_type = Some(PieceType::WhitePawn);
            } else if self.bitboards.white_knights.0 & (1u64 << mv.to) != 0 {
                captured_piece_type = Some(PieceType::WhiteKnight);
            } else if self.bitboards.white_bishops.0 & (1u64 << mv.to) != 0 {
                captured_piece_type = Some(PieceType::WhiteBishop);
            } else if self.bitboards.white_rooks.0 & (1u64 << mv.to) != 0 {
                captured_piece_type = Some(PieceType::WhiteRook);
            } else if self.bitboards.white_queens.0 & (1u64 << mv.to) != 0 {
                captured_piece_type = Some(PieceType::WhiteQueen);
            } else if self.bitboards.white_king.0 & (1u64 << mv.to) != 0 {
                captured_piece_type = Some(PieceType::WhiteKing);
            } else if self.bitboards.black_pawns.0 & (1u64 << mv.to) != 0 {
                captured_piece_type = Some(PieceType::BlackPawn);
            } else if self.bitboards.black_knights.0 & (1u64 << mv.to) != 0 {
                captured_piece_type = Some(PieceType::BlackKnight);
            } else if self.bitboards.black_bishops.0 & (1u64 << mv.to) != 0 {
                captured_piece_type = Some(PieceType::BlackBishop);
            } else if self.bitboards.black_rooks.0 & (1u64 << mv.to) != 0 {
                captured_piece_type = Some(PieceType::BlackRook);
            } else if self.bitboards.black_queens.0 & (1u64 << mv.to) != 0 {
                captured_piece_type = Some(PieceType::BlackQueen);
            } else if self.bitboards.black_king.0 & (1u64 << mv.to) != 0 {
                captured_piece_type = Some(PieceType::BlackKing);
            }

            if let Some(piece_type) = captured_piece_type {
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
        let _mv = Move::new(8, 16, false, PieceType::WhitePawn, None);
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
    fn move_generation() {
        let mut board = Board::new();
        // board.load_from_fen("rnb2b1r/pp2kp2/6p1/2p1p1Pp/2P1n3/2QPB2B/qP2KP1P/RN4NR b");
        // board.load_from_fen("1rb3kr/p2p4/np6/2p1qppp/5PnP/PPPp4/1B2P1KR/RN3BN1 w");
        // board.load_from_fen("1nk2bnr/4p3/r2qNp2/p6p/pP1pP1pP/3R1N2/1BPP1PP1/3QK1R1 w");
        // board.load_from_fen("2kr1bnB/pppN4/6p1/1N3p2/1np2P1r/P3Q3/4P1PP/1bK2BR1 w");
        board.load_from_fen("r4bnr/p2k4/Bpnp1p2/2p1p1pp/4PqP1/3PBP1P/PPP5/RNK2QNR w");
        // println!(
        //     "is king in check: {:#?}",
        //     board.is_king_in_check(Turn::WHITE)
        // )
        let moves = board.generate_moves();
        println!("{:#?}", moves);
    }

    #[test]
    fn is_king_in_check() {
        let mut board = Board::new();

        board.load_from_fen("r4bnr/p2k4/Bpnp1p2/2p1p1pp/4PBP1/3P1P1P/PPP5/RNK2QNR w");

        board.load_from_fen("r4bnr/p2k4/Bpnp1p2/2p1p1pp/4PqP1/3PBP1P/PPP5/RNK2QNR w");
        board.make_move(Move::new(20 , 29 , true , PieceType::WhiteBishop , None));

        println!("{:#?}", board.to_fen());

        println!(
            "is king in check: {:#?}",
            board.is_king_in_check(Turn::WHITE)
        )
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
