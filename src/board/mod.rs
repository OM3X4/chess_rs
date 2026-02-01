pub mod bishop_magic;
pub mod board;
pub mod constants;
mod engine;
pub mod move_gen;
mod openings;
pub mod rook_magic;
mod zobrist;
pub mod tt;
mod pieces;

use pieces::PieceType;
use std::ops::{Deref , DerefMut};
pub use board::Board;


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Move(u32);

impl Move {
    #[inline(always)]
    pub fn new(
        from: usize,
        to: usize,
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
    pub fn from(self) -> usize {
        (self.0 & 0b111111) as usize
    }

    #[inline(always)]
    pub fn to(self) -> usize {
        ((self.0 >> 6) & 0b111111) as usize
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

        if self.piece() == PieceType::WhitePawn && rank_to == 7 {
            s.push('q');
        } else if self.piece() == PieceType::BlackPawn && rank_to == 0 {
            s.push('q');
        }

        s
    } //
    #[inline(always)]
    pub fn from_uci(uci: &str, board: &Board) -> Move {
        let bytes = uci.as_bytes();

        debug_assert!(bytes.len() >= 4);

        let file_from = (bytes[0] - b'a') as usize;
        let rank_from = (bytes[1] - b'1') as usize;
        let from = rank_from * 8 + file_from;

        let file_to = (bytes[2] - b'a') as usize;
        let rank_to = (bytes[3] - b'1') as usize;
        let to: usize = rank_to * 8 + file_to;

        // Handle en passant
        if let Some(en_passant) = board.en_passant {
            let required_piece = match board.turn {
                Turn::BLACK => PieceType::BlackPawn,
                Turn::WHITE => PieceType::WhitePawn,
            };

            let piece = board.piece_at[from].unwrap();

            if en_passant == to && piece == required_piece {
                return Move::new(from, to, piece, true, false, false, true);
            }
        }

        let capture = board.piece_at[to as usize].is_some();
        let piece = board.piece_at[from as usize].unwrap();

        // Handle castling
        if piece == PieceType::BlackKing || piece == PieceType::WhiteKing {
            if from.abs_diff(to) == 2 {
                return Move::new(from, to, piece, false, true, false, false);
            }
        }

        if uci.len() == 5
            && (piece == PieceType::WhitePawn || piece == PieceType::BlackPawn)
            && (to > 55 || to < 8)
        {
            return Move::new(from, to, piece, capture, false, true, false);
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
    pub fn move_encoded(self) -> u32 {
        self.0
    }
} //

pub struct UnMakeMove {
    from: usize,
    to: usize,
    piece: PieceType,
    captured: Option<PieceType>,
    occupied: BitBoard,
    is_castling: bool,
    is_en_passant: bool,
    hash: u64,
    castling: u8,
    en_passant: Option<usize>,
    mat_eval: i32,
    mg_pst_eval: i32,
    eg_pst_eval: i32,
    mobility_eval: i32,
    last_irreversible_move: usize,
    number_of_pieces: usize,
    number_of_pawns: usize,
} //

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

impl Deref for BitBoards {
    type Target = [BitBoard; 12];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BitBoards {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
