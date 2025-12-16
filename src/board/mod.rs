pub mod board;
mod constants;
mod engine;
pub mod move_gen;
pub mod rook_magic;
pub mod bishop_magic;
mod zobrist;


pub use board::Board;

#[derive(Copy, Clone)]
pub struct TTEntry {
    pub key: u64,  // full zobrist key
    pub depth: i8, // remaining depth
    pub score: i32,
}
pub struct TranspositionTable {
    table: Vec<Option<TTEntry>>,
    mask: usize,
}

pub enum Bound {
    Upper,
    Lower,
    Exact,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
} //

impl PieceType {
    #[inline]
    pub fn piece_index(&self) -> usize {
        match self {
            PieceType::WhitePawn => 0,
            PieceType::WhiteKnight => 1,
            PieceType::WhiteBishop => 2,
            PieceType::WhiteRook => 3,
            PieceType::WhiteQueen => 4,
            PieceType::WhiteKing => 5,
            PieceType::BlackPawn => 6,
            PieceType::BlackKnight => 7,
            PieceType::BlackBishop => 8,
            PieceType::BlackRook => 9,
            PieceType::BlackQueen => 10,
            PieceType::BlackKing => 11,
        }
    }
}

// #[derive(Debug, Copy, Clone)]
// pub struct Move {
//     pub from: u64,
//     pub to: u64,
//     piece_type: PieceType,
//     capture: bool,
// } //

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Move(u32);

impl Move {
    #[inline(always)]
    pub fn new(from: u8, to: u8, piece: PieceType, capture: bool) -> Self {
        let mut m = from as u32;
        m |= (to as u32) << 6;
        m |= (piece as u32) << 12;
        m |= (capture as u32) << 16;
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
}

pub struct UnMakeMove {
    bitboards: BitBoards,
    occupied: BitBoard,
    hash: u64,
}

impl UnMakeMove {
    pub fn new(bitboards: BitBoards, occupied: BitBoard, hash: u64) -> UnMakeMove {
        UnMakeMove {
            bitboards,
            occupied,
            hash,
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
pub struct BitBoards {
    // white
    pub white_pawns: BitBoard,
    pub white_knights: BitBoard,
    pub white_bishops: BitBoard,
    pub white_rooks: BitBoard,
    pub white_queens: BitBoard,
    pub white_king: BitBoard,
    //black
    pub black_pawns: BitBoard,
    pub black_knights: BitBoard,
    pub black_bishops: BitBoard,
    pub black_rooks: BitBoard,
    pub black_queens: BitBoard,
    pub black_king: BitBoard,
} //

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
    } //

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
    } //

    pub fn get_mut(&mut self, piece: PieceType) -> &mut u64 {
        match piece {
            PieceType::WhitePawn => &mut self.white_pawns.0,
            PieceType::WhiteKnight => &mut self.white_knights.0,
            PieceType::WhiteBishop => &mut self.white_bishops.0,
            PieceType::WhiteRook => &mut self.white_rooks.0,
            PieceType::WhiteQueen => &mut self.white_queens.0,
            PieceType::WhiteKing => &mut self.white_king.0,

            PieceType::BlackPawn => &mut self.black_pawns.0,
            PieceType::BlackKnight => &mut self.black_knights.0,
            PieceType::BlackBishop => &mut self.black_bishops.0,
            PieceType::BlackRook => &mut self.black_rooks.0,
            PieceType::BlackQueen => &mut self.black_queens.0,
            PieceType::BlackKing => &mut self.black_king.0,
        }
    } //

    #[inline(always)]
    pub fn get(&self, piece: PieceType) -> u64 {
        match piece {
            PieceType::WhitePawn => self.white_pawns.0,
            PieceType::WhiteKnight => self.white_knights.0,
            PieceType::WhiteBishop => self.white_bishops.0,
            PieceType::WhiteRook => self.white_rooks.0,
            PieceType::WhiteQueen => self.white_queens.0,
            PieceType::WhiteKing => self.white_king.0,

            PieceType::BlackPawn => self.black_pawns.0,
            PieceType::BlackKnight => self.black_knights.0,
            PieceType::BlackBishop => self.black_bishops.0,
            PieceType::BlackRook => self.black_rooks.0,
            PieceType::BlackQueen => self.black_queens.0,
            PieceType::BlackKing => self.black_king.0,
        }
    } //
} //

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum GameState {
    CheckMate,
    StaleMate,
    InProgress,
}
