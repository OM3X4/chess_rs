pub mod bishop_magic;
pub mod board;
mod constants;
mod engine;
pub mod move_gen;
pub mod rook_magic;
mod zobrist;

pub use board::Board;

#[derive(Copy, Clone)]

pub struct TTEntry {
    pub key: u64,  // full zobrist
    pub depth: i8, // remaining depth
    pub bound: Bound,
    pub score: i32,      // normalized score
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

const PIECE_VALUE: [i8; 12] = [
    1, // WhitePawn
    3, // WhiteKnight
    3, // WhiteBishop
    5, // WhiteRook
    9, // WhiteQueen
    0, // WhiteKing
    1, // BlackPawn
    3, // BlackKnight
    3, // BlackBishop
    5, // BlackRook
    9, // BlackQueen
    0, // BlackKing
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
    pub fn value(self) -> i8 {
        unsafe { *PIECE_VALUE.get_unchecked(self as usize) }
    }
}

#[derive(Copy, Clone, Debug , PartialEq, Eq)]
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
    pub fn from_uci(uci: &str, piece: PieceType, capture: bool) -> Move {
        let bytes = uci.as_bytes();

        debug_assert!(bytes.len() >= 4);

        let file_from = bytes[0] - b'a';
        let rank_from = bytes[1] - b'1';
        let from = rank_from * 8 + file_from;

        let file_to = bytes[2] - b'a';
        let rank_to = bytes[3] - b'1';
        let to = rank_to * 8 + file_to;

        Move::new(from, to, piece, capture)
    }
} //

pub struct UnMakeMove {
    from: u8,
    to: u8,
    piece: PieceType,
    captured: Option<PieceType>,
    occupied: BitBoard,
    hash: u64,
}

impl UnMakeMove {
    pub fn new(
        from: u8,
        to: u8,
        piece: PieceType,
        captured: Option<PieceType>,
        occupied: BitBoard,
        hash: u64,
    ) -> UnMakeMove {
        UnMakeMove {
            from,
            to,
            piece,
            captured,
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
