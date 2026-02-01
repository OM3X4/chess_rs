/* =========================================================
PUBLIC API
========================================================= */

#[inline(always)]
pub fn bishop_attacks(square: usize, occupied: u64) -> u64 {
    unsafe {
        let occ = occupied & BISHOP_MASKS[square];
        let index = (occ.wrapping_mul(BISHOP_MAGICS[square])) >> BISHOP_SHIFTS[square];
        BISHOP_ATTACKS[square][index as usize]
    }
}

pub fn init_bishop_magics() {
    for sq in 0..64 {
        let mask = bishop_mask(sq);
        unsafe {
            BISHOP_MASKS[sq] = mask;
        }

        let bits = extract_bits(mask);
        let subset_count = 1usize << bits.len();

        for i in 0..subset_count {
            let blockers = blockers_from_index(i, &bits);
            let attacks = bishop_attacks_on_the_fly(sq, blockers);

            let index = ((blockers & mask).wrapping_mul(BISHOP_MAGICS[sq])) >> BISHOP_SHIFTS[sq];

            unsafe {
                BISHOP_ATTACKS[sq][index as usize] = attacks;
            }
        }
    }
}

/* =========================================================
INTERNAL STORAGE
========================================================= */

const BISHOP_ATTACK_TABLE_SIZE: usize = 512;

static mut BISHOP_MASKS: [u64; 64] = [0; 64];
static mut BISHOP_ATTACKS: [[u64; BISHOP_ATTACK_TABLE_SIZE]; 64] = [[0; BISHOP_ATTACK_TABLE_SIZE]; 64];

/* =========================================================
MAGIC NUMBERS (VERIFIED)
========================================================= */

const BISHOP_MAGICS: [u64; 64] = [
    0x40040844404084,
    0x2004208a004208,
    0x10190041080202,
    0x108060845042010,
    0x581104180800210,
    0x2112080446200010,
    0x1080820820060210,
    0x3c0808410220200,
    0x4050404440404,
    0x21001420088,
    0x24d0080801082102,
    0x1020a0a020400,
    0x40308200402,
    0x4011002100800,
    0x401484104104005,
    0x801010402020200,
    0x400210c3880100,
    0x404022024108200,
    0x810018200204102,
    0x4002801a02003,
    0x85040820080400,
    0x810102c808880400,
    0xe900410884800,
    0x8002020480840102,
    0x220200865090201,
    0x2010100a02021202,
    0x152048408022401,
    0x20080002081110,
    0x4001001021004000,
    0x800040400a011002,
    0xe4004081011002,
    0x1c004001012080,
    0x8004200962a00220,
    0x8422100208500202,
    0x2000402200300c08,
    0x8646020080080080,
    0x80020a0200100808,
    0x2010004880111000,
    0x623000a080011400,
    0x42008c0340209202,
    0x209188240001000,
    0x400408a884001800,
    0x110400a6080400,
    0x1840060a44020800,
    0x90080104000041,
    0x201011000808101,
    0x1a2208080504f080,
    0x8012020600211212,
    0x500861011240000,
    0x180806108200800,
    0x4000020e01040044,
    0x300000261044000a,
    0x802241102020002,
    0x20906061210001,
    0x5a84841004010310,
    0x4010801011c04,
    0xa010109502200,
    0x4a02012000,
    0x500201010098b028,
    0x8040002811040900,
    0x28000010020204,
    0x6000020202d0240,
    0x8918844842082200,
    0x4010011029020020,
];

const BISHOP_SHIFTS: [u32; 64] = [
    58, 59, 59, 59, 59, 59, 59, 58,
    59, 59, 59, 59, 59, 59, 59, 59,
    59, 59, 57, 57, 57, 57, 59, 59,
    59, 59, 57, 55, 55, 57, 59, 59,
    59, 59, 57, 55, 55, 57, 59, 59,
    59, 59, 57, 57, 57, 57, 59, 59,
    59, 59, 59, 59, 59, 59, 59, 59,
    58, 59, 59, 59, 59, 59, 59, 58,
];

/* =========================================================
MASK GENERATION
========================================================= */

fn bishop_mask(square: usize) -> u64 {
    let rank = square / 8;
    let file = square % 8;
    let mut mask = 0u64;

    // northeast
    let mut r = rank + 1;
    let mut f = file + 1;
    while r < 7 && f < 7 {
        mask |= 1u64 << (r * 8 + f);
        r += 1;
        f += 1;
    }

    // northwest
    let mut r = rank + 1;
    let mut f = file as i32 - 1;
    while r < 7 && f > 0 {
        mask |= 1u64 << (r * 8 + f as usize);
        r += 1;
        f -= 1;
    }

    // southeast
    let mut r = rank as i32 - 1;
    let mut f = file + 1;
    while r > 0 && f < 7 {
        mask |= 1u64 << (r as usize * 8 + f);
        r -= 1;
        f += 1;
    }

    // southwest
    let mut r = rank as i32 - 1;
    let mut f = file as i32 - 1;
    while r > 0 && f > 0 {
        mask |= 1u64 << (r as usize * 8 + f as usize);
        r -= 1;
        f -= 1;
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

fn bishop_attacks_on_the_fly(square: usize, blockers: u64) -> u64 {
    let mut attacks = 0u64;
    let rank = square / 8;
    let file = square % 8;

    // northeast
    let mut r = rank + 1;
    let mut f = file + 1;
    while r < 8 && f < 8 {
        let sq = r * 8 + f;
        attacks |= 1u64 << sq;
        if blockers & (1u64 << sq) != 0 {
            break;
        }
        r += 1;
        f += 1;
    }

    // northwest
    let mut r = rank + 1;
    let mut f = file as i32 - 1;
    while r < 8 && f >= 0 {
        let sq = r * 8 + f as usize;
        attacks |= 1u64 << sq;
        if blockers & (1u64 << sq) != 0 {
            break;
        }
        r += 1;
        f -= 1;
    }

    // southeast
    let mut r = rank as i32 - 1;
    let mut f = file + 1;
    while r >= 0 && f < 8 {
        let sq = r as usize * 8 + f;
        attacks |= 1u64 << sq;
        if blockers & (1u64 << sq) != 0 {
            break;
        }
        r -= 1;
        f += 1;
    }

    // southwest
    let mut r = rank as i32 - 1;
    let mut f = file as i32 - 1;
    while r >= 0 && f >= 0 {
        let sq = r as usize * 8 + f as usize;
        attacks |= 1u64 << sq;
        if blockers & (1u64 << sq) != 0 {
            break;
        }
        r -= 1;
        f -= 1;
    }

    attacks
}