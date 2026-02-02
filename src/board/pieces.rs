use crate::board::bishop_magic::bishop_attacks;
use crate::board::constants::{
    EG_BISHOP_TABLE, EG_KING_TABLE, EG_KNIGHT_TABLE, EG_PAWN_TABLE, EG_QUEEN_TABLE, EG_ROOK_TABLE,
    KNIGHTS_ATTACK_TABLE, MG_BISHOP_TABLE, MG_KING_TABLE, MG_KNIGHT_TABLE, MG_PAWN_TABLE,
    MG_QUEEN_TABLE, MG_ROOK_TABLE, PST,
};
use crate::board::rook_magic::rook_attacks;

const PIECE_VALUE: [i32; 12] = [
    100,  // WhitePawn
    300,  // WhiteKnight
    300,  // WhiteBishop
    500,  // WhiteRook
    900,  // WhiteQueen
    0,    // WhiteKing
    -100, // BlackPawn
    -300, // BlackKnight
    -300, // BlackBishop
    -500, // BlackRook
    -900, // BlackQueen
    0,    // BlackKing
];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PieceType {
    WhitePawn = 0,
    WhiteKnight = 1,
    WhiteBishop = 2,
    WhiteRook = 3,
    WhiteQueen = 4,
    WhiteKing = 5,
    BlackPawn = 6,
    BlackKnight = 7,
    BlackBishop = 8,
    BlackRook = 9,
    BlackQueen = 10,
    BlackKing = 11,
}

impl PieceType {
    #[inline(always)]
    pub fn piece_index(self) -> usize {
        self as usize
    }
    pub fn value(self) -> i32 {
        unsafe { *PIECE_VALUE.get_unchecked(self as usize) }
    }
    pub fn pst_old(&self, square: usize, is_eg: bool) -> i32 {
        if is_eg {
            return match self.piece_index() {
                0 => EG_PAWN_TABLE[square ^ 56],
                1 => EG_KNIGHT_TABLE[square ^ 56],
                2 => EG_BISHOP_TABLE[square ^ 56],
                3 => EG_ROOK_TABLE[square ^ 56],
                4 => EG_QUEEN_TABLE[square ^ 56],
                5 => EG_KING_TABLE[square ^ 56],
                6 => -EG_PAWN_TABLE[square],
                7 => -EG_KNIGHT_TABLE[square],
                8 => -EG_BISHOP_TABLE[square],
                9 => -EG_ROOK_TABLE[square],
                10 => -EG_QUEEN_TABLE[square],
                11 => -EG_KING_TABLE[square],
                _ => 0,
            };
        } else {
            return match self.piece_index() {
                0 => MG_PAWN_TABLE[square ^ 56],
                1 => MG_KNIGHT_TABLE[square ^ 56],
                2 => MG_BISHOP_TABLE[square ^ 56],
                3 => MG_ROOK_TABLE[square ^ 56],
                4 => MG_QUEEN_TABLE[square ^ 56],
                5 => MG_KING_TABLE[square ^ 56],
                6 => -MG_PAWN_TABLE[square],
                7 => -MG_KNIGHT_TABLE[square],
                8 => -MG_BISHOP_TABLE[square],
                9 => -MG_ROOK_TABLE[square],
                10 => -MG_QUEEN_TABLE[square],
                11 => -MG_KING_TABLE[square],
                _ => 0,
            };
        }
    } //

    pub fn pst(self, square: usize, is_eg: bool) -> i32 {
        return PST[is_eg as usize][self.piece_index()][square];
    }

    pub fn mobility_score(self, sq: usize, occupied: u64) -> i32 {
        return match self {
            PieceType::WhiteBishop => 2 * bishop_attacks(sq, occupied).count_ones() as i32,
            PieceType::BlackBishop => -2 * bishop_attacks(sq, occupied).count_ones() as i32,
            PieceType::WhiteRook => 2 * rook_attacks(sq, occupied).count_ones() as i32,
            PieceType::BlackRook => -2 * rook_attacks(sq, occupied).count_ones() as i32,
            PieceType::BlackKnight => -2 * KNIGHTS_ATTACK_TABLE[sq].count_ones() as i32,
            PieceType::WhiteKnight => 2 * KNIGHTS_ATTACK_TABLE[sq].count_ones() as i32,
            PieceType::WhiteQueen => {
                2 * rook_attacks(sq, occupied).count_ones() as i32
                    + 2 * bishop_attacks(sq, occupied).count_ones() as i32
            }
            PieceType::BlackQueen => {
                -2 * rook_attacks(sq, occupied).count_ones() as i32
                    - 2 * bishop_attacks(sq, occupied).count_ones() as i32
            }
            _ => 0,
        };
    }

    #[inline]
    pub fn flip_color(self) -> PieceType {
        let v = self as u8;
        unsafe { std::mem::transmute((v + 6) % 12) }
    }
}
