use crate::board::Move;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Bound {
    Exact = 0,
    Lower = 1,
    Upper = 2,
}

#[derive(Copy, Clone)]
pub struct TTEntry {
    pub key: u64,  // full zobrist
    pub depth: i8, // remaining depth
    pub bound: Bound,
    pub score: i32, // normalized score
    pub best_move: Move,
}

#[derive(Clone)]
pub struct TranspositionTable {
    table: Vec<Option<TTEntry>>,
    mask: usize,
}

impl TranspositionTable {
    pub fn new(size_pow2: usize) -> Self {
        let size = 1usize << size_pow2;
        Self {
            table: vec![None; size],
            mask: size - 1,
        }
    } //

    #[inline(always)]
    fn index(&self, key: u64) -> usize {
        (key as usize) & self.mask
    } //

    #[inline(always)]
    pub fn probe(&self, key: u64) -> Option<TTEntry> {
        let entry = self.table[self.index(key)]?;

        if entry.key != key {
            return None;
        }

        return Some(entry);
    } //

    #[inline(always)]
    pub fn store(
        &mut self,
        key: u64,
        depth: i8,
        score: i32,
        alpha: i32,
        beta: i32,
        best_move: Move,
        all_searched: bool,
    ) {
        let bound = if !all_searched {
            if score <= alpha {
                Bound::Upper
            } else {
                Bound::Lower
            }
        } else {
            if score <= alpha {
                Bound::Upper
            } else if score >= beta {
                Bound::Lower
            } else {
                Bound::Exact
            }
        };

        let idx = self.index(key);

        match self.table[idx] {
            None => {
                self.table[idx] = Some(TTEntry {
                    key,
                    depth,
                    score,
                    bound,
                    best_move,
                });
            }
            Some(old) if depth >= old.depth => {
                self.table[idx] = Some(TTEntry {
                    key,
                    depth,
                    score,
                    bound,
                    best_move,
                });
            }
            _ => {}
        }
    } //
} //
