pub mod board;
mod constants;
mod engine;
mod move_gen;

pub use board::Board;

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

#[derive(Debug, Copy, Clone)]
pub struct Move {
    pub from: u64,
    pub to: u64,
    piece_type: PieceType,
    capture: bool,
} //

pub struct UnMakeMove {
    bitboards: BitBoards,
    occupied: BitBoard,
    hash: u64
}

impl UnMakeMove {
    pub fn new(bitboards: BitBoards, occupied: BitBoard, hash: u64) -> UnMakeMove {
        UnMakeMove { bitboards, occupied, hash }
    }
}

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
        };
    }
} //

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
} //

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum GameState {
    CheckMate,
    StaleMate,
    InProgress,
}
