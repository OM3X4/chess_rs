pub mod bishop_magic;
pub mod board;
mod constants;
mod engine;
pub mod move_gen;
pub mod rook_magic;
mod test;
mod zobrist;

pub use board::Board;

use crate::board::constants::{BISHOPS_BONUS, KING_BONUS, PAWNS_BONUS, QUEENS_BONUS, ROOK_BONUS};

#[derive(Copy, Clone)]

pub struct TTEntry {
    pub key: u64,  // full zobrist
    pub depth: i8, // remaining depth
    pub bound: Bound,
    pub score: i32, // normalized score
}
pub struct TranspositionTable {
    table: Vec<Option<TTEntry>>,
    mask: usize,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Bound {
    Exact = 0,
    Lower = 1,
    Upper = 2,
}

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
    pub fn pst(&self, square: u8) -> i32 {
        match self.piece_index() {
            0 => PAWNS_BONUS[square as usize],
            1 => KING_BONUS[square as usize],
            2 => BISHOPS_BONUS[square as usize],
            3 => ROOK_BONUS[square as usize],
            4 => QUEENS_BONUS[square as usize],
            5 => KING_BONUS[square as usize],
            6 => PAWNS_BONUS[square as usize ^ 56],
            7 => KING_BONUS[square as usize ^ 56],
            8 => BISHOPS_BONUS[square as usize ^ 56],
            9 => ROOK_BONUS[square as usize ^ 56],
            10 => QUEENS_BONUS[square as usize ^ 56],
            11 => KING_BONUS[square as usize ^ 56],
            _ => 0,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Move(u32);

impl Move {
    #[inline(always)]
    pub fn new(
        from: u8,
        to: u8,
        piece: PieceType,
        capture: bool,
        castling: bool,
        promotion: bool,
        en_passant: bool,
    ) -> Self {
        let mut m = from as u32;
        m |= (to as u32) << 6;
        m |= (piece as u32) << 12;
        m |= (capture as u32) << 16;
        m |= (castling as u32) << 17;
        m |= (promotion as u32) << 18;
        m |= (en_passant as u32) << 19;
        Move(m)
    }

    #[inline(always)]
    pub fn from(self) -> u8 {
        (self.0 & 0b111111) as u8
    }

    #[inline(always)]
    pub fn to(self) -> u8 {
        ((self.0 >> 6) & 0b111111) as u8
    }

    #[inline(always)]
    pub fn piece(self) -> PieceType {
        unsafe { std::mem::transmute(((self.0 >> 12) & 0b1111) as u8) }
    }

    #[inline(always)]
    pub fn is_capture(self) -> bool {
        ((self.0 >> 16) & 1) != 0
    }
    #[inline(always)]
    pub fn to_uci(self) -> String {
        let from = self.from();
        let to = self.to();

        let file_from = (from & 7) as u8;
        let rank_from = (from >> 3) as u8;

        let file_to = (to & 7) as u8;
        let rank_to = (to >> 3) as u8;

        let mut s = String::with_capacity(4);
        s.push((b'a' + file_from) as char);
        s.push((b'1' + rank_from) as char);
        s.push((b'a' + file_to) as char);
        s.push((b'1' + rank_to) as char);

        s
    } //
    #[inline(always)]
    pub fn from_uci(uci: &str, board: &Board) -> Move {
        let bytes = uci.as_bytes();

        debug_assert!(bytes.len() >= 4);

        let file_from = bytes[0] - b'a';
        let rank_from = bytes[1] - b'1';
        let from = rank_from * 8 + file_from;

        let file_to = bytes[2] - b'a';
        let rank_to = bytes[3] - b'1';
        let to: u8 = rank_to * 8 + file_to;

        if let Some(en_passant) = board.en_passant {
            let required_piece = board.

            if en_passant == to {
                let piece = board.piece_at[from as usize].unwrap();

                return Move::new(from, to, piece, true, false, false, true);
            }
        }

        let capture = board.piece_at[to as usize].is_some();
        let piece = board.piece_at[from as usize].unwrap();

        if piece == PieceType::BlackKing || piece == PieceType::WhiteKing {
            if from.abs_diff(to) == 2 {
                return Move::new(from, to, piece, false, true, false, false);
            }
        }

        Move::new(from, to, piece, capture, false, false, false)
    } //

    #[inline(always)]
    pub fn is_castling(self) -> bool {
        ((self.0 >> 17) & 1) != 0
    } //
    #[inline(always)]
    pub fn is_en_passant(self) -> bool {
        ((self.0 >> 19) & 1) != 0
    }
} //

pub struct UnMakeMove {
    from: u8,
    to: u8,
    piece: PieceType,
    captured: Option<PieceType>,
    occupied: BitBoard,
    is_castling: bool,
    is_en_passant: bool,
    hash: u64,
    castling: u8,
    en_passant: Option<u8>,
    eval: i32,
}

impl UnMakeMove {
    pub fn new(
        from: u8,
        to: u8,
        piece: PieceType,
        captured: Option<PieceType>,
        is_castling: bool,
        is_en_passant: bool,
        occupied: BitBoard,
        hash: u64,
        castling: u8,
        en_passant: Option<u8>,
        eval: i32,
    ) -> UnMakeMove {
        UnMakeMove {
            from,
            to,
            piece,
            captured,
            occupied,
            hash,
            is_castling,
            is_en_passant,
            castling,
            en_passant,
            eval,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BitBoard(u64);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Turn {
    WHITE,
    BLACK,
} //

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BitBoards([BitBoard; 12]);

impl BitBoards {
    pub fn default() -> BitBoards {
        BitBoards([
            BitBoard(0x000000000000FF00), // white pawns
            BitBoard(0x0000000000000042), // white knights
            BitBoard(0x0000000000000024), // white bishops
            BitBoard(0x0000000000000081), // white rooks
            BitBoard(0x0000000000000008), // white queens
            BitBoard(0x0000000000000010), // white king
            BitBoard(0x00FF000000000000), // black pawns
            BitBoard(0x4200000000000000), // black knights
            BitBoard(0x2400000000000000), // black bishops
            BitBoard(0x8100000000000000), // black rooks
            BitBoard(0x0800000000000000), // black queens
            BitBoard(0x1000000000000000), // black king
        ])
    } //

    pub fn zero() -> BitBoards {
        BitBoards([BitBoard(0); 12])
    } //
} //

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum GameState {
    CheckMate,
    StaleMate,
    InProgress,
}
