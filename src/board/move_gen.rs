use crate::board::UnMakeMove;
use crate::board::bishop_magic::bishop_attacks;
use crate::board::rook_magic::rook_attacks;
use smallvec::SmallVec;

use super::constants::{
    BLACK_PAWN_ATTACKS, KING_ATTACK_TABLE, KNIGHTS_ATTACK_TABLE, WHITE_PAWN_ATTACKS,
};
use super::constants::{RANK_2, RANK_7};
use super::{Board, Move, PieceType, Turn};

pub fn extract_bits(bitboard: u64) -> Vec<u64> {
    let mut res: Vec<u64> = Vec::new();
    let mut bb = bitboard;
    while bb != 0 {
        let lsb = bb.trailing_zeros();
        res.push(lsb as u64);
        bb &= bb - 1;
    }
    res
}

impl Board {
    #[inline(always)]
    pub fn generate_knight_moves(&self, moves: &mut SmallVec<[Move; 256]>) {
        // let mut moves = Vec::new();
        let enemy_bits = self.get_enemy_pieces().0;
        let allay_bits = self.get_allay_pieces().0;

        let (mut knights, piece_type) = match self.turn {
            Turn::WHITE => (
                self.bitboards.0[PieceType::WhiteKnight.piece_index()].0,
                PieceType::WhiteKnight,
            ),
            Turn::BLACK => (
                self.bitboards.0[PieceType::BlackKnight.piece_index()].0,
                PieceType::BlackKnight,
            ),
        };

        while knights != 0 {
            let from = knights.trailing_zeros() as u64;
            knights &= knights - 1;
            let mut attacks = KNIGHTS_ATTACK_TABLE[from as usize] & !allay_bits;

            while attacks != 0 {
                let to = attacks.trailing_zeros() as u64;
                attacks &= attacks - 1;
                let capture = (enemy_bits & (1u64 << to)) != 0;
                moves.push(Move::new(
                    from as u8, to as u8, piece_type, capture, false, false, false,
                ));
            }
        }
    } //

    #[inline(always)]
    pub fn generate_king_moves(&self, moves: &mut SmallVec<[Move; 256]>) {
        // let mut moves = Vec::new();
        let enemy_bits = self.get_enemy_pieces().0;
        let allay_bits = self.get_allay_pieces().0;
        let occupied = self.occupied.0;

        let (king, piece_type) = match self.turn {
            Turn::WHITE => (
                self.bitboards.0[PieceType::WhiteKing.piece_index()].0,
                PieceType::WhiteKing,
            ),
            Turn::BLACK => (
                self.bitboards.0[PieceType::BlackKing.piece_index()].0,
                PieceType::BlackKing,
            ),
        };

        match self.turn {
            Turn::WHITE => {
                if (self.castling & 0b0001 != 0) && ((((1u64 << 5) | (1u64 << 6)) & occupied) == 0)
                {
                    moves.push(Move::new(4, 6, piece_type, false, true, false, false));
                }
                if self.castling & 0b0010 != 0
                    && ((((1u64 << 1) | (1u64 << 2) | (1u64 << 3)) & occupied) == 0)
                {
                    moves.push(Move::new(4, 2, piece_type, false, true, false, false));
                }
            }
            Turn::BLACK => {
                if (self.castling & 0b0100 != 0)
                    && ((((1u64 << 61) | (1u64 << 62)) & occupied) == 0)
                {
                    moves.push(Move::new(60, 62, piece_type, false, true, false, false));
                }
                if self.castling & 0b1000 != 0
                    && ((((1u64 << 57) | (1u64 << 58) | (1u64 << 59)) & occupied) == 0)
                {
                    moves.push(Move::new(60, 58, piece_type, false, true, false, false));
                }
            }
        }

        let from = king.trailing_zeros() as u64;
        if from > 63 {
            return;
        }

        let mut attacks = (KING_ATTACK_TABLE[from as usize]) & !allay_bits;

        while attacks != 0 {
            let to = attacks.trailing_zeros() as u64;
            attacks &= attacks - 1;
            let capture = (enemy_bits & (1u64 << to)) != 0;
            moves.push(Move::new(
                from as u8, to as u8, piece_type, capture, false, false, false,
            ));
        }
    } //

    #[inline(always)]
    pub fn generate_white_pawns_moves(&self, moves: &mut SmallVec<[Move; 256]>) {
        let blockers = self.occupied.0;
        // let pawn_squares = &self.bitboards.0[PieceType::WhitePawn.piece_index()];

        let enemy_pieces_bb = self.get_all_black_bits();

        let mut pawns = self.bitboards.0[PieceType::WhitePawn.piece_index()].0;

        while pawns != 0 {
            let from = pawns.trailing_zeros() as u64;
            pawns &= pawns - 1;

            let pawn_bb = 1u64 << from;

            // single and double jump
            if (blockers & 1u64 << from + 8) == 0 {
                moves.push(Move::new(
                    from as u8,
                    (from + 8) as u8,
                    PieceType::WhitePawn,
                    false,
                    false,
                    (pawn_bb & RANK_7) != 0,
                    false,
                ));
                if (((1u64 << from) & RANK_2) != 0) && (blockers & 1u64 << (from + 16)) == 0 {
                    moves.push(Move::new(
                        from as u8,
                        (from + 16) as u8,
                        PieceType::WhitePawn,
                        false,
                        false,
                        false,
                        false,
                    ));
                }
            }

            // attacks
            let attacks_bb = WHITE_PAWN_ATTACKS[from as usize];
            let mut attacks = attacks_bb & enemy_pieces_bb.0;

            if let Some(en_passant_square) = self.en_passant {
                if attacks_bb & (1u64 << en_passant_square) != 0 {
                    moves.push(Move::new(
                        from as u8,
                        en_passant_square as u8,
                        PieceType::WhitePawn,
                        true,
                        false,
                        false,
                        true,
                    ));
                }
            }

            while attacks != 0 {
                let to = attacks.trailing_zeros() as u64;
                attacks &= attacks - 1;
                moves.push(Move::new(
                    from as u8,
                    to as u8,
                    PieceType::WhitePawn,
                    true,
                    false,
                    to > 55,
                    false,
                ));
            }
        }
    } //

    #[inline(always)]
    pub fn generate_black_pawns_moves(&self, moves: &mut SmallVec<[Move; 256]>) {
        let blockers = self.occupied.0;
        let enemy_pieces_bb = self.get_all_white_bits();

        let mut pawns = self.bitboards.0[PieceType::BlackPawn.piece_index()].0;

        while pawns != 0 {
            let from = pawns.trailing_zeros() as u64;
            pawns &= pawns - 1;

            let pawn_bb = 1u64 << from;

            // single and double jump
            if (blockers & 1u64 << (from - 8)) == 0 {
                moves.push(Move::new(
                    from as u8,
                    (from - 8) as u8,
                    PieceType::BlackPawn,
                    false,
                    false,
                    (pawn_bb & RANK_2) != 0,
                    false,
                ));
                if (((1u64 << from) & RANK_7) != 0) && (blockers & (1u64 << (from - 16))) == 0 {
                    moves.push(Move::new(
                        from as u8,
                        (from - 16) as u8,
                        PieceType::BlackPawn,
                        false,
                        false,
                        false,
                        false,
                    ));
                }
            }
            // attacks
            let attacks_bb = BLACK_PAWN_ATTACKS[from as usize];
            let mut attacks = attacks_bb & enemy_pieces_bb.0;

            if let Some(en_passant_square) = self.en_passant {
                if attacks_bb & (1u64 << en_passant_square) != 0 {
                    moves.push(Move::new(
                        from as u8,
                        en_passant_square as u8,
                        PieceType::BlackPawn,
                        true,
                        false,
                        false,
                        true,
                    ));
                }
            }

            while attacks != 0 {
                let to = attacks.trailing_zeros() as u64;
                attacks &= attacks - 1;
                moves.push(Move::new(
                    from as u8,
                    to as u8,
                    PieceType::BlackPawn,
                    true,
                    false,
                    to < 8,
                    false,
                ));
            }
        }
    } //

    #[inline(always)]
    pub fn generate_rook_moves_magics(&self, moves: &mut SmallVec<[Move; 256]>) {
        let allay = self.get_allay_pieces().0;
        let enemy = self.get_enemy_pieces().0;
        let occupied = self.occupied.0;

        let (mut rooks, piece_type) = match self.turn {
            Turn::WHITE => (
                self.bitboards.0[PieceType::WhiteRook.piece_index()].0,
                PieceType::WhiteRook,
            ),
            Turn::BLACK => (
                self.bitboards.0[PieceType::BlackRook.piece_index()].0,
                PieceType::BlackRook,
            ),
        };

        while rooks != 0 {
            let from = rooks.trailing_zeros() as u64;
            rooks &= rooks - 1;

            let attacks_bb = rook_attacks(from as usize, occupied);
            let mut attacks = attacks_bb & !allay;

            while attacks != 0 {
                let to = attacks.trailing_zeros() as u64;
                attacks &= attacks - 1;
                let capture = (enemy & (1u64 << to)) != 0;
                moves.push(Move::new(
                    from as u8, to as u8, piece_type, capture, false, false, false,
                ));
            }
        }

        let (mut queens, piece_type) = match self.turn {
            Turn::WHITE => (
                self.bitboards.0[PieceType::WhiteQueen.piece_index()].0,
                PieceType::WhiteQueen,
            ),
            Turn::BLACK => (
                self.bitboards.0[PieceType::BlackQueen.piece_index()].0,
                PieceType::BlackQueen,
            ),
        };

        while queens != 0 {
            let from = queens.trailing_zeros() as u64;
            queens &= queens - 1;

            let attacks_bb = rook_attacks(from as usize, occupied);
            let mut attacks = attacks_bb & !allay;

            while attacks != 0 {
                let to = attacks.trailing_zeros() as u64;
                attacks &= attacks - 1;
                let capture = (enemy & (1u64 << to)) != 0;
                moves.push(Move::new(
                    from as u8, to as u8, piece_type, capture, false, false, false,
                ));
            }
        }
    } //

    #[inline(always)]
    pub fn generate_bishop_moves_magics(&self, moves: &mut SmallVec<[Move; 256]>) {
        let allay_bits = self.get_allay_pieces();
        let enemy_bits = self.get_enemy_pieces();
        let all_bits = self.occupied.0;

        let (mut bishops, piece_type) = match self.turn {
            Turn::WHITE => (
                self.bitboards.0[PieceType::WhiteBishop.piece_index()].0,
                PieceType::WhiteBishop,
            ),
            Turn::BLACK => (
                self.bitboards.0[PieceType::BlackBishop.piece_index()].0,
                PieceType::BlackBishop,
            ),
        };

        while bishops != 0 {
            let from = bishops.trailing_zeros() as u64;
            bishops &= bishops - 1;

            let attacks_bb = bishop_attacks(from as usize, all_bits);
            let mut attacks = attacks_bb & !allay_bits.0;

            while attacks != 0 {
                let to = attacks.trailing_zeros() as u64;
                attacks &= attacks - 1;
                let capture = (enemy_bits.0 & (1u64 << to)) != 0;
                moves.push(Move::new(
                    from as u8, to as u8, piece_type, capture, false, false, false,
                ));
            }
        }

        let (mut queens, piece_type) = match self.turn {
            Turn::WHITE => (
                self.bitboards.0[PieceType::WhiteQueen.piece_index()].0,
                PieceType::WhiteQueen,
            ),
            Turn::BLACK => (
                self.bitboards.0[PieceType::BlackQueen.piece_index()].0,
                PieceType::BlackQueen,
            ),
        };

        while queens != 0 {
            let from = queens.trailing_zeros() as u64;
            queens &= queens - 1;

            let attacks_bb = bishop_attacks(from as usize, all_bits);
            let mut attacks = attacks_bb & !allay_bits.0;

            while attacks != 0 {
                let to = attacks.trailing_zeros() as u64;
                attacks &= attacks - 1;
                let capture = (enemy_bits.0 & (1u64 << to)) != 0;
                moves.push(Move::new(
                    from as u8, to as u8, piece_type, capture, false, false, false,
                ));
            }
        }
    } //

    #[inline(always)]
    pub fn is_check_by_bishop(&self, king_bb: u64, sliders: u64) -> bool {
        let occupied = self.occupied.0;

        let from = king_bb.trailing_zeros() as u64;

        let attacks_bb = bishop_attacks(from as usize, occupied);
        let attacks = attacks_bb & sliders;

        return attacks != 0;
    } //

    #[inline(always)]
    pub fn is_check_by_rook(&self, king_bb: u64, sliders: u64) -> bool {
        let occupied = self.occupied.0;

        let from = king_bb.trailing_zeros() as u64;

        let attacks_bb = rook_attacks(from as usize, occupied);
        let attacks = attacks_bb & sliders;

        return attacks != 0;
    } //

    #[inline(always)]
    pub fn generate_pesudo_moves(&self, mut moves: &mut SmallVec<[Move; 256]>) {
        self.generate_knight_moves(&mut moves);
        self.generate_bishop_moves_magics(&mut moves);
        self.generate_rook_moves_magics(&mut moves);
        self.generate_king_moves(&mut moves);
        // self.generate_queen_moves(&mut moves);

        match self.turn {
            Turn::WHITE => self.generate_white_pawns_moves(&mut moves),
            Turn::BLACK => self.generate_black_pawns_moves(&mut moves),
        };
    } //

    pub fn generate_moves(&mut self) -> SmallVec<[Move; 256]> {
        let mut pesudo_moves: SmallVec<[Move; 256]> = SmallVec::new();
        let mut legal_moves: SmallVec<[Move; 256]> = SmallVec::new();

        self.generate_pesudo_moves(&mut pesudo_moves);

        let king_bb = match self.turn {
            Turn::WHITE => self.bitboards.0[PieceType::WhiteKing.piece_index()].0,
            Turn::BLACK => self.bitboards.0[PieceType::BlackKing.piece_index()].0,
        };

        let king_type = match self.turn {
            Turn::WHITE => PieceType::WhiteKing,
            Turn::BLACK => PieceType::BlackKing,
        };

        let is_king_in_check_now = self.is_king_in_check(self.turn);

        let king_square = king_bb.trailing_zeros() as usize;

        if king_square > 63 {
            return legal_moves;
        }

        for mv in pesudo_moves {
            // if !is_king_in_check_now {
            //     if ((1u64 << mv.from()) & SQUARE_RAYS[king_square]) == 0 && mv.piece() != king_type
            //     {
            //         legal_moves.push(mv);
            //         continue;
            //     }
            // } else {
            //     if (1u64 << mv.to())
            //         & (SQUARE_RAYS[king_square] | KNIGHTS_ATTACK_TABLE[king_square])
            //         == 0
            //         && mv.piece() != king_type
            //     {
            //         continue;
            //     }
            // }

            if mv.is_castling() {
                match mv.to() {
                    6 => {
                        if self.is_square_attacked(6, self.opposite_turn()) {
                            continue;
                        } else if self.is_square_attacked(5, self.opposite_turn()) {
                            continue;
                        } else if self.is_square_attacked(4, self.opposite_turn()) {
                            continue;
                        }
                    }
                    2 => {
                        if self.is_square_attacked(2, self.opposite_turn()) {
                            continue;
                        } else if self.is_square_attacked(3, self.opposite_turn()) {
                            continue;
                        } else if self.is_square_attacked(4, self.opposite_turn()) {
                            continue;
                        }
                    }
                    58 => {
                        if self.is_square_attacked(58, self.opposite_turn()) {
                            continue;
                        } else if self.is_square_attacked(59, self.opposite_turn()) {
                            continue;
                        } else if self.is_square_attacked(60, self.opposite_turn()) {
                            continue;
                        }
                    }
                    62 => {
                        if self.is_square_attacked(62, self.opposite_turn()) {
                            continue;
                        } else if self.is_square_attacked(61, self.opposite_turn()) {
                            continue;
                        } else if self.is_square_attacked(60, self.opposite_turn()) {
                            continue;
                        }
                    }
                    _ => (),
                }
            };

            let unmake_move = self.make_move(mv);

            let is_illegal = self.is_king_in_check(self.opposite_turn());

            if !is_illegal {
                legal_moves.push(mv);
            }

            self.unmake_move(unmake_move);
        }
        return legal_moves;
    } //

    pub fn is_king_in_check(&self, turn: Turn) -> bool {
        let (king, enemy_king) = match turn {
            Turn::BLACK => (
                &self.bitboards.0[PieceType::BlackKing.piece_index()].0,
                &self.bitboards.0[PieceType::WhiteKing.piece_index()].0,
            ),
            Turn::WHITE => (
                &self.bitboards.0[PieceType::WhiteKing.piece_index()].0,
                &self.bitboards.0[PieceType::BlackKing.piece_index()].0,
            ),
        };

        if king.trailing_zeros() > 63 {
            return false;
        }

        let king_square = king.trailing_zeros() as u64;

        let enemy_rooks = match turn {
            Turn::BLACK => &self.bitboards.0[PieceType::WhiteRook.piece_index()].0,
            Turn::WHITE => &self.bitboards.0[PieceType::BlackRook.piece_index()].0,
        };
        let enemy_queens = match turn {
            Turn::BLACK => &self.bitboards.0[PieceType::WhiteQueen.piece_index()].0,
            Turn::WHITE => &self.bitboards.0[PieceType::BlackQueen.piece_index()].0,
        };
        let enemy_bishops = match turn {
            Turn::BLACK => &self.bitboards.0[PieceType::WhiteBishop.piece_index()].0,
            Turn::WHITE => &self.bitboards.0[PieceType::BlackBishop.piece_index()].0,
        };
        let enemy_knights = match turn {
            Turn::BLACK => &self.bitboards.0[PieceType::WhiteKnight.piece_index()].0,
            Turn::WHITE => &self.bitboards.0[PieceType::BlackKnight.piece_index()].0,
        };
        let enemy_pawns = match turn {
            Turn::BLACK => &self.bitboards.0[PieceType::WhitePawn.piece_index()].0,
            Turn::WHITE => &self.bitboards.0[PieceType::BlackPawn.piece_index()].0,
        };
        let is_attacked_by_knights =
            (KNIGHTS_ATTACK_TABLE[king_square as usize] & enemy_knights) != 0;

        if is_attacked_by_knights {
            return true;
        }

        let is_attacked_by_king = (KING_ATTACK_TABLE[king_square as usize] & enemy_king) != 0;

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

        match turn {
            Turn::BLACK => {
                // Use BLACK_PAWN_ATTACKS to look "down" (South) towards where
                // White pawns would be attacking from.
                let mask = BLACK_PAWN_ATTACKS[king_square as usize];

                if enemy_pawns & mask != 0 {
                    return true;
                }
            }
            Turn::WHITE => {
                // Use WHITE_PAWN_ATTACKS to look "up" (North) towards where
                // Black pawns would be attacking from.
                let mask = WHITE_PAWN_ATTACKS[king_square as usize];
                if enemy_pawns & mask != 0 {
                    return true;
                }
            }
        }

        return false;
    } //

    pub fn is_square_attacked(&self, square: u8, turn: Turn) -> bool {
        let enemy_king = match turn {
            Turn::BLACK => &self.bitboards.0[PieceType::WhiteKing.piece_index()].0,
            Turn::WHITE => &self.bitboards.0[PieceType::BlackKing.piece_index()].0,
        };

        let square_bb = 1u64 << square as u64;

        let enemy_rooks = match turn {
            Turn::BLACK => &self.bitboards.0[PieceType::WhiteRook.piece_index()].0,
            Turn::WHITE => &self.bitboards.0[PieceType::BlackRook.piece_index()].0,
        };
        let enemy_queens = match turn {
            Turn::BLACK => &self.bitboards.0[PieceType::WhiteQueen.piece_index()].0,
            Turn::WHITE => &self.bitboards.0[PieceType::BlackQueen.piece_index()].0,
        };
        let enemy_bishops = match turn {
            Turn::BLACK => &self.bitboards.0[PieceType::WhiteBishop.piece_index()].0,
            Turn::WHITE => &self.bitboards.0[PieceType::BlackBishop.piece_index()].0,
        };
        let enemy_knights = match turn {
            Turn::BLACK => &self.bitboards.0[PieceType::WhiteKnight.piece_index()].0,
            Turn::WHITE => &self.bitboards.0[PieceType::BlackKnight.piece_index()].0,
        };
        let enemy_pawns = match turn {
            Turn::BLACK => &self.bitboards.0[PieceType::WhitePawn.piece_index()].0,
            Turn::WHITE => &self.bitboards.0[PieceType::BlackPawn.piece_index()].0,
        };

        let is_attacked_by_knights = (KNIGHTS_ATTACK_TABLE[square as usize] & enemy_knights) != 0;

        if is_attacked_by_knights {
            return true;
        }

        let is_attacked_by_king = (KING_ATTACK_TABLE[square as usize] & enemy_king) != 0;

        if is_attacked_by_king {
            return true;
        }

        let is_attacked_by_bishops_or_queens =
            self.is_check_by_bishop(square_bb, *enemy_bishops | *enemy_queens);

        if is_attacked_by_bishops_or_queens {
            return true;
        }

        let is_attacked_by_rooks_or_queens =
            self.is_check_by_rook(square_bb, *enemy_rooks | *enemy_queens);

        if is_attacked_by_rooks_or_queens {
            return true;
        }

        match turn {
            Turn::BLACK => {
                // Use BLACK_PAWN_ATTACKS to look "down" (South) towards where
                // White pawns would be attacking from.
                let mask = BLACK_PAWN_ATTACKS[square as usize];

                if enemy_pawns & mask != 0 {
                    return true;
                }
            }
            Turn::WHITE => {
                // Use WHITE_PAWN_ATTACKS to look "up" (North) towards where
                // Black pawns would be attacking from.
                let mask = WHITE_PAWN_ATTACKS[square as usize];
                if enemy_pawns & mask != 0 {
                    return true;
                }
            }
        }

        return false;
    } //

    pub fn make_move(&mut self, mv: Move) -> UnMakeMove {
        let from = mv.from() as u8;
        let to = mv.to() as u8;
        let piece = mv.piece();

        let captured = self.piece_at[to as usize];

        let undo = UnMakeMove {
            from,
            to,
            piece,
            captured,
            hash: self.hash,
            is_en_passant: mv.is_en_passant(),
            occupied: self.occupied,
            is_castling: mv.is_castling(),
            castling: self.castling,
            en_passant: self.en_passant,
        };

        /* -----------------------------
            Update castling rights
        ----------------------------- */
        match piece {
            PieceType::WhiteKing => self.castling &= !0b0011,
            PieceType::BlackKing => self.castling &= !0b1100,
            PieceType::WhiteRook => {
                if from == 0 {
                    self.castling &= !0b0010;
                }
                if from == 7 {
                    self.castling &= !0b0001;
                }
            }
            PieceType::BlackRook => {
                if from == 56 {
                    self.castling &= !0b1000;
                }
                if from == 63 {
                    self.castling &= !0b0100;
                }
            }
            _ => {}
        };

        if let Some(cap) = captured {
            match cap {
                PieceType::WhiteRook => {
                    if to == 0 {
                        self.castling &= !0b0010;
                    }
                    if to == 7 {
                        self.castling &= !0b0001;
                    }
                }
                PieceType::BlackRook => {
                    if to == 56 {
                        self.castling &= !0b1000;
                    }
                    if to == 63 {
                        self.castling &= !0b0100;
                    }
                }
                _ => {}
            }
        }

        /* -----------------------------
            Clear en-passant by default
        ----------------------------- */
        self.en_passant = None;

        /* -----------------------------
            Castling move
        ----------------------------- */
        if mv.is_castling() {
            match (self.turn, to) {
                (Turn::WHITE, 6) => {
                    // e1g1
                    self.remove_piece(PieceType::WhiteKing, 4);
                    self.remove_piece(PieceType::WhiteRook, 7);
                    self.add_piece(PieceType::WhiteKing, 6);
                    self.add_piece(PieceType::WhiteRook, 5);
                }
                (Turn::WHITE, 2) => {
                    // e1c1
                    self.remove_piece(PieceType::WhiteKing, 4);
                    self.remove_piece(PieceType::WhiteRook, 0);
                    self.add_piece(PieceType::WhiteKing, 2);
                    self.add_piece(PieceType::WhiteRook, 3);
                }
                (Turn::BLACK, 62) => {
                    // e8g8
                    self.remove_piece(PieceType::BlackKing, 60);
                    self.remove_piece(PieceType::BlackRook, 63);
                    self.add_piece(PieceType::BlackKing, 62);
                    self.add_piece(PieceType::BlackRook, 61);
                }
                (Turn::BLACK, 58) => {
                    // e8c8
                    self.remove_piece(PieceType::BlackKing, 60);
                    self.remove_piece(PieceType::BlackRook, 56);
                    self.add_piece(PieceType::BlackKing, 58);
                    self.add_piece(PieceType::BlackRook, 59);
                }
                _ => unreachable!(),
            }

            self.switch_turn();
            self.hash = self.compute_hash();
            self.occupied = self.get_all_bits();
            self.piece_at = self.generate_piece_at();

            return undo;
        }

        /* -----------------------------
            En passant capture
        ----------------------------- */
        if mv.is_en_passant() {
            match self.turn {
                Turn::WHITE => {
                    self.remove_piece(PieceType::BlackPawn, to - 8);
                    self.remove_piece(PieceType::WhitePawn, from);
                    self.add_piece(PieceType::WhitePawn, to);
                }
                Turn::BLACK => {
                    self.remove_piece(PieceType::WhitePawn, to + 8);
                    self.remove_piece(PieceType::BlackPawn, from);
                    self.add_piece(PieceType::BlackPawn, to);
                }
            }

            self.switch_turn();
            self.hash = self.compute_hash();

            self.occupied = self.get_all_bits();
            self.piece_at = self.generate_piece_at();

            return undo;
        }

        /* -----------------------------
            Normal capture
        ----------------------------- */
        if let Some(p) = captured {
            self.remove_piece(p, to);
        }

        /* -----------------------------
            Promotion
        ----------------------------- */
        if piece == PieceType::WhitePawn && to >= 56 {
            self.remove_piece(PieceType::WhitePawn, from);
            self.add_piece(PieceType::WhiteQueen, to);
        } else if piece == PieceType::BlackPawn && to <= 7 {
            self.remove_piece(PieceType::BlackPawn, from);
            self.add_piece(PieceType::BlackQueen, to);
        } else {
            self.remove_piece(piece, from);
            self.add_piece(piece, to);
        }

        /* -----------------------------
            Set en passant square
        ----------------------------- */
        if piece == PieceType::WhitePawn && from / 8 == 1 && to / 8 == 3 {
            self.en_passant = Some(from + 8);
        }
        if piece == PieceType::BlackPawn && from / 8 == 6 && to / 8 == 4 {
            self.en_passant = Some(from - 8);
        }

        self.switch_turn();
        self.hash = self.compute_hash();
        self.occupied = self.get_all_bits();
        self.piece_at = self.generate_piece_at();

        undo
    } //

    pub fn unmake_move(&mut self, unmake_move: UnMakeMove) {
        // self.bitboards = unmake_move.bitboards;

        if unmake_move.is_castling {
            match (self.opposite_turn(), unmake_move.to) {
                (Turn::WHITE, 6) => {
                    // e1g1
                    self.remove_piece(PieceType::WhiteKing, 6);
                    self.remove_piece(PieceType::WhiteRook, 5);
                    self.add_piece(PieceType::WhiteKing, 4);
                    self.add_piece(PieceType::WhiteRook, 7);
                }
                (Turn::WHITE, 2) => {
                    // e1c1
                    self.remove_piece(PieceType::WhiteKing, 2);
                    self.remove_piece(PieceType::WhiteRook, 3);
                    self.add_piece(PieceType::WhiteKing, 4);
                    self.add_piece(PieceType::WhiteRook, 0);
                }
                (Turn::BLACK, 62) => {
                    // e8g8
                    self.remove_piece(PieceType::BlackKing, 62);
                    self.remove_piece(PieceType::BlackRook, 61);
                    self.add_piece(PieceType::BlackKing, 60);
                    self.add_piece(PieceType::BlackRook, 63);
                }
                (Turn::BLACK, 58) => {
                    // e8c8
                    self.remove_piece(PieceType::BlackKing, 58);
                    self.remove_piece(PieceType::BlackRook, 59);
                    self.add_piece(PieceType::BlackKing, 60);
                    self.add_piece(PieceType::BlackRook, 56);
                }
                _ => (),
            }
        } else if unmake_move.is_en_passant {
            match self.turn {
                Turn::BLACK => {
                    self.remove_piece(PieceType::WhitePawn, unmake_move.to);
                    self.add_piece(PieceType::WhitePawn, unmake_move.from);
                    self.add_piece(PieceType::BlackPawn, unmake_move.to - 8);
                }
                Turn::WHITE => {
                    self.remove_piece(PieceType::BlackPawn, unmake_move.to);
                    self.add_piece(PieceType::BlackPawn, unmake_move.from);
                    self.add_piece(PieceType::WhitePawn, unmake_move.to + 8);
                }
            }
        } else if unmake_move.piece == PieceType::WhitePawn && unmake_move.to >= 56 {
            self.remove_piece(PieceType::WhiteQueen, unmake_move.to);
            self.add_piece(PieceType::WhitePawn, unmake_move.from);
            if let Some(captured) = unmake_move.captured {
                self.add_piece(captured, unmake_move.to);
            }
        } else if unmake_move.piece == PieceType::BlackPawn && unmake_move.to <= 7 {
            self.remove_piece(PieceType::BlackQueen, unmake_move.to);
            self.add_piece(PieceType::BlackPawn, unmake_move.from);
            if let Some(captured) = unmake_move.captured {
                self.add_piece(captured, unmake_move.to);
            }
        } else {
            self.remove_piece(unmake_move.piece, unmake_move.to);
            self.add_piece(unmake_move.piece, unmake_move.from);

            if let Some(piece) = unmake_move.captured {
                self.add_piece(piece, unmake_move.to);
            }
        }

        self.switch_turn();
        self.castling = unmake_move.castling;
        self.en_passant = unmake_move.en_passant;
        self.occupied = unmake_move.occupied;
        self.hash = unmake_move.hash;
        self.piece_at = self.generate_piece_at();
    } //
} //

mod tests {
    use crate::board::{PieceType, move_gen::extract_bits};

    #[test]
    fn test() {
        let mut board = super::Board::new();
        board.load_from_fen("r1b3r1/1pkq1nbP/p2ppB2/2nP4/P1p2NPP/1P6/N1PKP3/2RQ1BR1 w - - 1 20");
        let moves = board.generate_moves();
        println!(
            "{:?}",
            moves.iter().map(|mv| mv.to_uci()).collect::<Vec<String>>()
        );
    }

    #[test]
    fn test2() {
        let mut board = super::Board::new();
        board.load_from_fen("1r1qkbnr/pb1ppppp/n7/1Pp5/2pP4/B6P/P3PPP1/RN1QKBNR w KQk c6 0 9");
        board.generate_moves();
        dbg!(board.to_fen());
        dbg!(
            extract_bits(board.bitboards.0[PieceType::WhitePawn as usize].0),
            extract_bits(board.bitboards.0[PieceType::BlackPawn as usize].0)
        );
    }
}
