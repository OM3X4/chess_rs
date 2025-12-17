use crate::board::UnMakeMove;
use crate::board::bishop_magic::bishop_attacks;
use crate::board::rook_magic::rook_attacks;
use smallvec::SmallVec;

use super::constants::{
    BLACK_PAWN_ATTACKS, KING_ATTACK_TABLE, KNIGHTS_ATTACK_TABLE, SQUARE_RAYS, WHITE_PAWN_ATTACKS,
};
use super::constants::{FILE_A, FILE_H, RANK_2, RANK_7};
use super::zobrist::{Z_PIECE, Z_SIDE};
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
                moves.push(Move::new(from as u8, to as u8, piece_type, capture));
            }
        }
    } //

    #[inline(always)]
    pub fn generate_king_moves(&self, moves: &mut SmallVec<[Move; 256]>) {
        // let mut moves = Vec::new();
        let enemy_bits = self.get_enemy_pieces().0;
        let allay_bits = self.get_allay_pieces().0;

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

        let from = king.trailing_zeros() as u64;
        if from > 63 {
            return;
        }

        let mut attacks = (KING_ATTACK_TABLE[from as usize]) & !allay_bits;

        while attacks != 0 {
            let to = attacks.trailing_zeros() as u64;
            attacks &= attacks - 1;
            let capture = (enemy_bits & (1u64 << to)) != 0;
            moves.push(Move::new(from as u8, to as u8, piece_type, capture));
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
            if from < 55 && (blockers & 1u64 << from + 8) == 0 {
                moves.push(Move::new(
                    from as u8,
                    (from + 8) as u8,
                    PieceType::WhitePawn,
                    false,
                ));
                if (((1u64 << from) & RANK_2) != 0) && (blockers & 1u64 << (from + 16)) == 0 {
                    moves.push(Move::new(
                        from as u8,
                        (from + 16) as u8,
                        PieceType::WhitePawn,
                        false,
                    ));
                }
            }

            // attacks
            let attacks_bb = WHITE_PAWN_ATTACKS[from as usize];
            let mut attacks = attacks_bb & enemy_pieces_bb.0;
            while attacks != 0 {
                let to = attacks.trailing_zeros() as u64;
                attacks &= attacks - 1;
                moves.push(Move::new(from as u8, to as u8, PieceType::WhitePawn, true));
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
            if from >= 8 && (blockers & 1u64 << (from - 8)) == 0 {
                moves.push(Move::new(
                    from as u8,
                    (from - 8) as u8,
                    PieceType::WhitePawn,
                    false,
                ));
                if (((1u64 << from) & RANK_7) != 0) && (blockers & (1u64 << (from - 16))) == 0 {
                    moves.push(Move::new(
                        from as u8,
                        (from - 16) as u8,
                        PieceType::WhitePawn,
                        false,
                    ));
                }
            }
            // attacks
            let attacks_bb = BLACK_PAWN_ATTACKS[from as usize];
            let mut attacks = attacks_bb & enemy_pieces_bb.0;
            while attacks != 0 {
                let to = attacks.trailing_zeros() as u64;
                attacks &= attacks - 1;
                moves.push(Move::new(from as u8, to as u8, PieceType::WhitePawn, true));
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
                moves.push(Move::new(from as u8, to as u8, piece_type, capture));
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
                moves.push(Move::new(from as u8, to as u8, piece_type, capture));
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
                moves.push(Move::new(from as u8, to as u8, piece_type, capture));
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
                moves.push(Move::new(from as u8, to as u8, piece_type, capture));
            }
        }
    } //

    #[inline(always)]
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
            if !is_king_in_check_now {
                if ((1u64 << mv.from()) & SQUARE_RAYS[king_square]) == 0 && mv.piece() != king_type
                {
                    legal_moves.push(mv);
                    continue;
                }
            } else {
                if (1u64 << mv.to())
                    & (SQUARE_RAYS[king_square] | KNIGHTS_ATTACK_TABLE[king_square])
                    == 0
                    && mv.piece() != king_type
                {
                    continue;
                }
            }
            let unmake_move = self.make_move(mv);
            self.switch_turn();

            let is_illegal = self.is_king_in_check(self.turn);

            if !is_illegal {
                legal_moves.push(mv);
            }

            self.unmake_move(unmake_move);
            self.switch_turn();
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
                let mask = BLACK_PAWN_ATTACKS[king_square as usize];
                if enemy_pawns & mask != 0 {
                    return true;
                }
            }
            Turn::WHITE => {
                let mask = WHITE_PAWN_ATTACKS[king_square as usize];
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

        let captured = self.piece_at[to as usize];

        let undo = UnMakeMove {
            from,
            to,
            piece: mv.piece(),
            captured,
            hash: self.hash,
            occupied: self.occupied,
        };

        if let Some(p) = captured {
            self.remove_piece(p, to);
        }

        self.remove_piece(mv.piece(), from);
        self.add_piece(mv.piece(), to);

        self.switch_turn();
        self.hash ^= *Z_SIDE;

        undo
    }//

    pub fn unmake_move(&mut self, unmake_move: UnMakeMove) {
        // self.bitboards = unmake_move.bitboards;
        self.remove_piece(unmake_move.piece, unmake_move.to);
        self.add_piece(unmake_move.piece, unmake_move.from);

        if let Some(piece) = self.piece_at[unmake_move.to as usize] {
            self.remove_piece(piece, unmake_move.to);
        }

        self.switch_turn();
        self.hash = unmake_move.hash;
        self.occupied = unmake_move.occupied;
    } //

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut board = Board::new();
        board.load_from_fen("1rbk1bnr/pp3ppp/1Pp1p3/3p1P2/5N1q/2NQ2P1/1PP1P2P/R1B1KB1R w");
        // board.load_from_fen("1rbk1bnr/pp3ppp/1Pp1p3/3Q1P2/5N1q/2N3P1/1PP1P2P/R1B1KB1R w");

        dbg!(board.is_king_in_check(Turn::WHITE));

        let moves = board.generate_moves();
        for mv in moves {
            // println!("{:?} , {:?}", mv.from , mv.to);
        }
    } //


    #[test]
    fn test_rook_magic() {
        let mut board = Board::new();
        board.load_from_fen("1rbk1bnr/pp3ppp/1Pp1p3/3p1P2/5N1q/2NQ2P1/1PP1P2P/R1B1KB1R w");
        let attacks = rook_attacks(0, board.occupied.0);
        dbg!(extract_bits(attacks));
    }
}
