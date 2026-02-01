/* =========================================================
PUBLIC API
========================================================= */

#[inline(always)]
pub fn rook_attacks(square: usize, occupied: u64) -> u64 {
    unsafe {
        let occ = occupied & ROOK_MASKS[square];
        let index = (occ.wrapping_mul(ROOK_MAGICS[square])) >> ROOK_SHIFTS[square];
        ROOK_ATTACKS[square][index as usize]
    }
}

pub fn init_rook_magics() {
    for sq in 0..64 {
        let mask = rook_mask(sq);
        unsafe {
            ROOK_MASKS[sq] = mask;
        }

        let bits = extract_bits(mask);
        let subset_count = 1usize << bits.len();

        for i in 0..subset_count {
            let blockers = blockers_from_index(i, &bits);
            let attacks = rook_attacks_on_the_fly(sq, blockers);

            let index = ((blockers & mask).wrapping_mul(ROOK_MAGICS[sq])) >> ROOK_SHIFTS[sq];

            unsafe {
                ROOK_ATTACKS[sq][index as usize] = attacks;
            }
        }
    }
}

/* =========================================================
INTERNAL STORAGE
========================================================= */

const ROOK_ATTACK_TABLE_SIZE: usize = 4096;

static mut ROOK_MASKS: [u64; 64] = [0; 64];
static mut ROOK_ATTACKS: [[u64; ROOK_ATTACK_TABLE_SIZE]; 64] = [[0; ROOK_ATTACK_TABLE_SIZE]; 64];

/* =========================================================
MAGIC NUMBERS (VERIFIED)
========================================================= */

const ROOK_MAGICS: [u64; 64] = [
    0x8a80104000800020,
    0x140002000100040,
    0x2801880a0017001,
    0x100081001000420,
    0x200020010080420,
    0x3001c0002010008,
    0x8480008002000100,
    0x2080088004402900,
    0x800098204000,
    0x2024401000200040,
    0x100802000801000,
    0x120800800801000,
    0x208808088000400,
    0x2802200800400,
    0x2200800100020080,
    0x801000060821100,
    0x80044006422000,
    0x100808020004000,
    0x12108a0010204200,
    0x140848010000802,
    0x481828014002800,
    0x8094004002004100,
    0x4010040010010802,
    0x20008806104,
    0x100400080208000,
    0x2040002120081000,
    0x21200680100081,
    0x20100080080080,
    0x2000a00200410,
    0x20080800400,
    0x80088400100102,
    0x80004600042881,
    0x4040008040800020,
    0x440003000200801,
    0x4200011004500,
    0x188020010100100,
    0x14800401802800,
    0x2080040080800200,
    0x124080204001001,
    0x200046502000484,
    0x480400080088020,
    0x1000422010034000,
    0x30200100110040,
    0x100021010009,
    0x2002080100110004,
    0x202008004008002,
    0x20020004010100,
    0x2048440040820001,
    0x101002200408200,
    0x40802000401080,
    0x4008142004410100,
    0x2060820c0120200,
    0x1001004080100,
    0x20c020080040080,
    0x2935610830022400,
    0x44440041009200,
    0x280001040802101,
    0x2100190040002085,
    0x80c0084100102001,
    0x4024081001000421,
    0x20030a0244872,
    0x12001008414402,
    0x2006104900a0804,
    0x1004081002402,
];

const ROOK_SHIFTS: [u32; 64] = [
    52, 53, 53, 53, 53, 53, 53, 52, 53, 54, 54, 54, 54, 54, 54, 53, 53, 54, 54, 54, 54, 54, 54, 53,
    53, 54, 54, 54, 54, 54, 54, 53, 53, 54, 54, 54, 54, 54, 54, 53, 53, 54, 54, 54, 54, 54, 54, 53,
    53, 54, 54, 54, 54, 54, 54, 53, 52, 53, 53, 53, 53, 53, 53, 52,
];

/* =========================================================
MASK GENERATION
========================================================= */

fn rook_mask(square: usize) -> u64 {
    let rank = square / 8;
    let file = square % 8;
    let mut mask = 0u64;

    // north
    for r in rank + 1..7 {
        mask |= 1u64 << (r * 8 + file);
    }

    // south
    for r in (1..rank).rev() {
        mask |= 1u64 << (r * 8 + file);
    }

    // east
    for f in file + 1..7 {
        mask |= 1u64 << (rank * 8 + f);
    }

    // west
    for f in (1..file).rev() {
        mask |= 1u64 << (rank * 8 + f);
    }

    mask
}


/* =========================================================
SUBSET ENUMERATION
========================================================= */

fn extract_bits(mask: u64) -> Vec<usize> {
    let mut bits = Vec::new();
    let mut m = mask;
    while m != 0 {
        let sq = m.trailing_zeros() as usize;
        bits.push(sq);
        m &= m - 1;
    }
    bits
}

fn blockers_from_index(index: usize, bits: &[usize]) -> u64 {
    let mut blockers = 0u64;
    for (i, &sq) in bits.iter().enumerate() {
        if (index >> i) & 1 != 0 {
            blockers |= 1u64 << sq;
        }
    }
    blockers
}

/* =========================================================
REFERENCE ATTACK GENERATOR (CORRECTNESS)
========================================================= */

fn rook_attacks_on_the_fly(square: usize, blockers: u64) -> u64 {
    let mut attacks = 0u64;
    let rank = square / 8;
    let file = square % 8;

    for r in rank + 1..8 {
        let sq = r * 8 + file;
        attacks |= 1u64 << sq;
        if blockers & (1u64 << sq) != 0 {
            break;
        }
    }

    for r in (0..rank).rev() {
        let sq = r * 8 + file;
        attacks |= 1u64 << sq;
        if blockers & (1u64 << sq) != 0 {
            break;
        }
    }

    for f in file + 1..8 {
        let sq = rank * 8 + f;
        attacks |= 1u64 << sq;
        if blockers & (1u64 << sq) != 0 {
            break;
        }
    }

    for f in (0..file).rev() {
        let sq = rank * 8 + f;
        attacks |= 1u64 << sq;
        if blockers & (1u64 << sq) != 0 {
            break;
        }
    }

    attacks
}
