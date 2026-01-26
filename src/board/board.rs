use super::constants::{RANK_1, RANK_2, RANK_7, RANK_8};
use super::zobrist::{Z_PIECE, Z_SIDE};
use super::{BitBoard, BitBoards, GameState, Turn};
use crate::board::Move;
use crate::board::PieceType;
use crate::board::bishop_magic::bishop_attacks;
use crate::board::constants::KNIGHTS_ATTACK_TABLE;
use crate::board::openings::{BookEntry, OPENING_BOOK};
use crate::board::rook_magic::rook_attacks;
use rand::Rng;
use rand::rand_core::le;
use shakmaty::Piece;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Board {
    pub bitboards: BitBoards,
    pub turn: Turn,
    pub piece_at: [Option<PieceType>; 64],
    pub occupied: BitBoard,
    pub hash: u64,
    pub en_passant: Option<u8>,
    pub castling: u8,
    pub history: Vec<u64>,
    pub last_irreversible_move: u64,
    pub mat_eval: i32,      // Always white favor
    pub mg_pst_eval: i32,   // Always white favor
    pub eg_pst_eval: i32,   // Always white favor
    pub mobility_eval: i32, // Always white favor
    pub number_of_pieces: u32,
    pub number_of_pawns: u32,
}

impl Board {
    pub fn new() -> Board {
        let mut board = Board {
            bitboards: BitBoards::default(),
            turn: Turn::WHITE,
            piece_at: [None; 64],
            hash: 0,
            occupied: BitBoard(RANK_1 | RANK_2 | RANK_7 | RANK_8),
            en_passant: None,
            castling: 15,
            mat_eval: 0,
            mg_pst_eval: 0,
            eg_pst_eval: 0,
            mobility_eval: 0,
            history: Vec::new(),
            last_irreversible_move: 0,
            number_of_pieces: 0,
            number_of_pawns: 0,
        };

        board.piece_at = board.generate_piece_at();
        board.hash = board.compute_hash();
        board.history = vec![board.hash];
        board.mat_eval = board.pieces_score();
        let (mg_score, eg_score) = board.generate_pst_score();
        board.mg_pst_eval = mg_score;
        board.eg_pst_eval = eg_score;

        board
    } //

    pub fn reset_to_default(&mut self) {
        self.bitboards = BitBoards::default();
        self.hash = self.compute_hash();
        self.piece_at = self.generate_piece_at();
        self.occupied = BitBoard(RANK_1 | RANK_2 | RANK_7 | RANK_8);
        self.turn = Turn::WHITE;
        self.en_passant = None;
        self.castling = 15;
        self.mat_eval = 0;
        self.mg_pst_eval = 0;
        self.eg_pst_eval = 0;
        self.mobility_eval = 0;
        self.number_of_pieces = 32;
        self.number_of_pawns = 16;
        self.history = vec![self.hash];
    } //
    pub fn reset_to_zero(&mut self) {
        self.bitboards = BitBoards::zero();
        self.occupied = BitBoard(0);
        self.piece_at = [None; 64];
        self.hash = self.compute_hash();
        self.turn = Turn::WHITE;
        self.en_passant = None;
        self.castling = 0;
        self.mat_eval = 0;
        self.mg_pst_eval = 0;
        self.eg_pst_eval = 0;
        self.mobility_eval = 0;
        self.number_of_pawns = 0;
        self.number_of_pieces = 0;
        self.history = vec![self.hash];
    } //
    pub fn get_all_white_bits(&self) -> BitBoard {
        return BitBoard(
            self.bitboards.0[PieceType::WhitePawn.piece_index()].0
                | self.bitboards.0[PieceType::WhiteKnight.piece_index()].0
                | self.bitboards.0[PieceType::WhiteBishop.piece_index()].0
                | self.bitboards.0[PieceType::WhiteRook.piece_index()].0
                | self.bitboards.0[PieceType::WhiteQueen.piece_index()].0
                | self.bitboards.0[PieceType::WhiteKing.piece_index()].0,
        );
    } //
    pub fn get_all_black_bits(&self) -> BitBoard {
        return BitBoard(
            self.bitboards.0[PieceType::BlackPawn.piece_index()].0
                | self.bitboards.0[PieceType::BlackKnight.piece_index()].0
                | self.bitboards.0[PieceType::BlackBishop.piece_index()].0
                | self.bitboards.0[PieceType::BlackRook.piece_index()].0
                | self.bitboards.0[PieceType::BlackQueen.piece_index()].0
                | self.bitboards.0[PieceType::BlackKing.piece_index()].0,
        );
    } //
    pub fn get_all_bits(&self) -> BitBoard {
        return BitBoard(self.get_all_white_bits().0 | self.get_all_black_bits().0);
    } //

    pub fn piece_at(&self, square: u64) -> Option<PieceType> {
        let bb = 1u64 << square;
        if self.bitboards.0[PieceType::BlackKing.piece_index()].0 & bb != 0 {
            return Some(PieceType::BlackKing);
        } else if self.bitboards.0[PieceType::WhitePawn.piece_index()].0 & bb != 0 {
            return Some(PieceType::WhitePawn);
        } else if self.bitboards.0[PieceType::WhiteKnight.piece_index()].0 & bb != 0 {
            return Some(PieceType::WhiteKnight);
        } else if self.bitboards.0[PieceType::WhiteBishop.piece_index()].0 & bb != 0 {
            return Some(PieceType::WhiteBishop);
        } else if self.bitboards.0[PieceType::WhiteRook.piece_index()].0 & bb != 0 {
            return Some(PieceType::WhiteRook);
        } else if self.bitboards.0[PieceType::WhiteQueen.piece_index()].0 & bb != 0 {
            return Some(PieceType::WhiteQueen);
        } else if self.bitboards.0[PieceType::WhiteKing.piece_index()].0 & bb != 0 {
            return Some(PieceType::WhiteKing);
        } else if self.bitboards.0[PieceType::BlackPawn.piece_index()].0 & bb != 0 {
            return Some(PieceType::BlackPawn);
        } else if self.bitboards.0[PieceType::BlackKnight.piece_index()].0 & bb != 0 {
            return Some(PieceType::BlackKnight);
        } else if self.bitboards.0[PieceType::BlackBishop.piece_index()].0 & bb != 0 {
            return Some(PieceType::BlackBishop);
        } else if self.bitboards.0[PieceType::BlackRook.piece_index()].0 & bb != 0 {
            return Some(PieceType::BlackRook);
        } else if self.bitboards.0[PieceType::BlackQueen.piece_index()].0 & bb != 0 {
            return Some(PieceType::BlackQueen);
        } else if self.bitboards.0[PieceType::BlackKing.piece_index()].0 & bb != 0 {
            return Some(PieceType::BlackKing);
        } else {
            return None;
        }
    } //

    pub fn generate_piece_at(&self) -> [Option<PieceType>; 64] {
        let mut piece_at = [None; 64];
        for square in 0..64 {
            piece_at[square] = self.piece_at(square as u64);
        }
        piece_at
    } //

    pub fn add_piece(&mut self, piece: PieceType, sq: u8) {
        let mask = 1u64 << sq;

        self.bitboards.0[piece.piece_index()].0 |= mask;
        self.occupied.0 |= mask;
        self.piece_at[sq as usize] = Some(piece);
        self.mat_eval += piece.value();
        self.number_of_pieces += 1;
        if piece == PieceType::WhitePawn || piece == PieceType::BlackPawn {
            self.number_of_pawns += 1;
        }
        // count mobility
        self.mobility_eval += piece.mobility_score(sq as usize, self.occupied.0);
        self.mg_pst_eval += piece.pst(sq, false);
        self.eg_pst_eval += piece.pst(sq, true);

        self.hash ^= Z_PIECE[piece.piece_index()][sq as usize];
    } //

    pub fn remove_piece(&mut self, piece: PieceType, sq: u8) {
        let mask = 1u64 << sq;

        self.bitboards.0[piece.piece_index()].0 &= !mask;
        self.occupied.0 &= !mask;
        self.piece_at[sq as usize] = None;
        self.mat_eval -= piece.value();
        self.number_of_pieces -= 1;
        if piece == PieceType::WhitePawn || piece == PieceType::BlackPawn {
            self.number_of_pawns -= 1;
        }
        // count mobility
        self.mobility_eval -= piece.mobility_score(sq as usize, self.occupied.0);
        self.mg_pst_eval -= piece.pst(sq, false);
        self.eg_pst_eval -= piece.pst(sq, true);

        self.hash ^= Z_PIECE[piece.piece_index()][sq as usize];
    } //

    pub fn load_from_fen(&mut self, fen: &str) {
        self.reset_to_zero();

        // let (position, turn) = fen.split_once(' ').unwrap();

        let splitted = fen.split_ascii_whitespace().collect::<Vec<_>>();

        if splitted.len() < 2 {
            panic!("Invalid FEN");
        }

        let position = splitted[0];
        let turn = splitted[1];

        if let Some(castling) = splitted.get(2) {
            if castling != &"-" {
                if castling.contains('K') {
                    self.castling |= 0b0001;
                }
                if castling.contains('Q') {
                    self.castling |= 0b0010;
                }
                if castling.contains('k') {
                    self.castling |= 0b0100;
                }
                if castling.contains('q') {
                    self.castling |= 0b1000;
                }
            }

            if let Some(en_passant) = splitted.get(3) {
                if en_passant != &"-" {
                    let bytes = en_passant.as_bytes();
                    let file = bytes[0] - b'a'; // 'a'..'h' -> 0..7
                    let rank = bytes[1] - b'1'; // '1'..'8' -> 0..7

                    if rank == 2 || rank == 5 {
                        self.en_passant = Some(rank * 8 + file);
                    } else {
                        self.en_passant = None; // invalid FEN safety
                    }
                }
            }
        }

        self.turn = match turn {
            "w" => Turn::WHITE,
            "b" => Turn::BLACK,
            _ => panic!("Invalid side to move"),
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
                        'P' => Some(&mut self.bitboards.0[PieceType::WhitePawn.piece_index()]),
                        'R' => Some(&mut self.bitboards.0[PieceType::WhiteRook.piece_index()]),
                        'Q' => Some(&mut self.bitboards.0[PieceType::WhiteQueen.piece_index()]),
                        'K' => Some(&mut self.bitboards.0[PieceType::WhiteKing.piece_index()]),
                        'N' => Some(&mut self.bitboards.0[PieceType::WhiteKnight.piece_index()]),
                        'B' => Some(&mut self.bitboards.0[PieceType::WhiteBishop.piece_index()]),
                        'p' => Some(&mut self.bitboards.0[PieceType::BlackPawn.piece_index()]),
                        'r' => Some(&mut self.bitboards.0[PieceType::BlackRook.piece_index()]),
                        'q' => Some(&mut self.bitboards.0[PieceType::BlackQueen.piece_index()]),
                        'k' => Some(&mut self.bitboards.0[PieceType::BlackKing.piece_index()]),
                        'n' => Some(&mut self.bitboards.0[PieceType::BlackKnight.piece_index()]),
                        'b' => Some(&mut self.bitboards.0[PieceType::BlackBishop.piece_index()]),
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
        self.piece_at = self.generate_piece_at();
        self.hash = self.compute_hash();

        self.mat_eval = self.pieces_score();
        self.mobility_eval = self.generate_mobility_eval();
        let (mg_score, eg_score) = self.generate_pst_score();
        self.mg_pst_eval = mg_score;
        self.eg_pst_eval = eg_score;

        self.history = vec![self.hash];
        self.number_of_pieces = self.generate_pieces_count();
        self.number_of_pawns = self.generate_pawns_count();
    } //

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();

        for rank in (0..8).rev() {
            let mut empty = 0;

            for file in 0..8 {
                let sq = rank * 8 + file;
                let mask = 1u64 << sq;

                let piece = if self.bitboards.0[PieceType::WhitePawn.piece_index()].0 & mask != 0 {
                    'P'
                } else if self.bitboards.0[PieceType::WhiteKnight.piece_index()].0 & mask != 0 {
                    'N'
                } else if self.bitboards.0[PieceType::WhiteBishop.piece_index()].0 & mask != 0 {
                    'B'
                } else if self.bitboards.0[PieceType::WhiteRook.piece_index()].0 & mask != 0 {
                    'R'
                } else if self.bitboards.0[PieceType::WhiteQueen.piece_index()].0 & mask != 0 {
                    'Q'
                } else if self.bitboards.0[PieceType::WhiteKing.piece_index()].0 & mask != 0 {
                    'K'
                } else if self.bitboards.0[PieceType::BlackPawn.piece_index()].0 & mask != 0 {
                    'p'
                } else if self.bitboards.0[PieceType::BlackKnight.piece_index()].0 & mask != 0 {
                    'n'
                } else if self.bitboards.0[PieceType::BlackBishop.piece_index()].0 & mask != 0 {
                    'b'
                } else if self.bitboards.0[PieceType::BlackRook.piece_index()].0 & mask != 0 {
                    'r'
                } else if self.bitboards.0[PieceType::BlackQueen.piece_index()].0 & mask != 0 {
                    'q'
                } else if self.bitboards.0[PieceType::BlackKing.piece_index()].0 & mask != 0 {
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

        fen.push(' ');
        if self.castling != 0 {
            if self.castling & 0b0001 != 0 {
                fen.push('K');
            }
            if self.castling & 0b0010 != 0 {
                fen.push('Q');
            }
            if self.castling & 0b0100 != 0 {
                fen.push('k');
            }
            if self.castling & 0b1000 != 0 {
                fen.push('q');
            }
        } else {
            fen.push('-');
        }

        fen.push(' ');
        if let Some(en_passant) = self.en_passant {
            let file = en_passant % 8;
            let rank = en_passant / 8;
            fen.push((b'a' + file as u8) as char);
            fen.push((b'1' + rank as u8) as char);
        } else {
            fen.push('-');
        }

        fen
    } //

    pub fn get_enemy_pieces(&self) -> BitBoard {
        return match self.turn {
            Turn::WHITE => self.get_all_black_bits(),
            Turn::BLACK => self.get_all_white_bits(),
        };
    } //

    pub fn get_allay_pieces(&self) -> BitBoard {
        return match self.turn {
            Turn::BLACK => self.get_all_black_bits(),
            Turn::WHITE => self.get_all_white_bits(),
        };
    } //

    pub fn switch_turn(&mut self) {
        self.turn = match self.turn {
            Turn::BLACK => Turn::WHITE,
            Turn::WHITE => Turn::BLACK,
        };

        self.hash ^= *Z_SIDE;
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
    } //

    pub fn opposite_turn(&self) -> Turn {
        return match self.turn {
            Turn::WHITE => Turn::BLACK,
            Turn::BLACK => Turn::WHITE,
        };
    } //

    pub fn is_3fold_repetition(&self) -> bool {
        let hash = self.hash;

        let mut count = 0;
        let mut len = self.history.len();

        if len < 0 {
            return false;
        }
        for i in (self.last_irreversible_move as usize..len).rev() {
            if self.history[i] == hash {
                count += 1;
                if count == 3 {
                    return true;
                }
            }
        }
        return false;
    } //

    pub fn print_board(&self) {
        let mut board_string = String::from("\n  a b c d e f g h\n");
        for rank in 0..8 {
            board_string.push_str(&format!("{} ", 8 - rank));
            for file in 0..8 {
                let square = rank * 8 + file;
                let piece = self.piece_at(square);
                let char = match piece {
                    Some(piece) => match piece {
                        PieceType::WhiteKing => 'K',
                        PieceType::WhiteQueen => 'Q',
                        PieceType::WhiteRook => 'R',
                        PieceType::WhiteBishop => 'B',
                        PieceType::WhiteKnight => 'N',
                        PieceType::WhitePawn => 'P',
                        PieceType::BlackKing => 'k',
                        PieceType::BlackQueen => 'q',
                        PieceType::BlackRook => 'r',
                        PieceType::BlackBishop => 'b',
                        PieceType::BlackKnight => 'n',
                        PieceType::BlackPawn => 'p',
                    },
                    None => '.',
                };
                board_string.push(char);
            }
            board_string.push_str("\n");
        }
        println!("{}", board_string);
    } //

    pub fn probe_opening(&mut self) -> Option<Move> {
        let mut lo = 0;
        let mut hi = OPENING_BOOK.len();

        let hash = self.hash;

        while lo < hi {
            let mid = (lo + hi) >> 1;
            let h = unsafe { OPENING_BOOK.get_unchecked(mid).hash };

            if h == hash {
                let moves = unsafe { OPENING_BOOK.get_unchecked(mid).moves };
                let valid_moves = self.generate_moves().iter().map(|mv| mv.to_uci()).collect::<Vec<String>>();
                let mut rng = rand::thread_rng();
                let index = rng.gen_range(0..moves.len());

                let mv = Move(moves[index]);

                if valid_moves.contains(&mv.to_uci()) {
                    return Some(mv);
                } else {
                    return None;
                }


            } else if h < hash {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        None
    } //

    pub fn generate_pst_score(&self) -> (i32, i32) {
        let mut eg_score = 0;
        let mut mg_score = 0;
        for (sq, piece) in self.piece_at.iter().enumerate() {
            if let Some(piece) = piece {
                eg_score += piece.pst(sq as u8, true);
                mg_score += piece.pst(sq as u8, false);
            }
        }
        return (mg_score, eg_score);
    } //

    pub fn generate_pieces_count(&self) -> u32 {
        let mut count = 0;
        for i in 0..64 {
            if self.piece_at(i) != None {
                count += 1;
            }
        }
        return count;
    } //

    pub fn generate_pawns_count(&self) -> u32 {
        let mut count = 0;
        for i in 0..64 {
            if self.piece_at(i) == Some(PieceType::WhitePawn)
                || self.piece_at(i) == Some(PieceType::BlackPawn)
            {
                count += 1;
            }
        }
        return count;
    } //

    pub fn generate_mobility_eval(&self) -> i32 {
        let mut eval = 0;
        for (sq, piece) in self.piece_at.iter().enumerate() {
            if let Some(piece) = piece {
                match piece {
                    PieceType::BlackBishop => {
                        eval += 2 * bishop_attacks(sq, self.occupied.0).count_ones() as i32;
                    }
                    PieceType::WhiteBishop => {
                        eval -= 2 * bishop_attacks(sq, self.occupied.0).count_ones() as i32;
                    }
                    PieceType::BlackRook => {
                        eval += 2 * rook_attacks(sq, self.occupied.0).count_ones() as i32;
                    }
                    PieceType::WhiteRook => {
                        eval -= 2 * rook_attacks(sq, self.occupied.0).count_ones() as i32;
                    }
                    PieceType::BlackKnight => {
                        eval -= 2 * KNIGHTS_ATTACK_TABLE[sq].count_ones() as i32;
                    }
                    PieceType::WhiteKnight => {
                        eval += 2 * KNIGHTS_ATTACK_TABLE[sq].count_ones() as i32;
                    }
                    PieceType::BlackQueen => {
                        eval -= 2 * bishop_attacks(sq, self.occupied.0).count_ones() as i32;
                        eval -= 2 * rook_attacks(sq, self.occupied.0).count_ones() as i32;
                    }
                    PieceType::WhiteQueen => {
                        eval += 2 * bishop_attacks(sq, self.occupied.0).count_ones() as i32;
                        eval += 2 * rook_attacks(sq, self.occupied.0).count_ones() as i32;
                    }
                    _ => (),
                }
            }
        }
        eval
    } //
} //
