use std::ops::Range;

pub const ACTIVE_PKM_BIT_LEN: usize = 546;
pub const ACTIVE_PKM_COUNT: usize = 4;
pub const ACTIVE_PKM_BITS: Range<usize> =
    0x83D9 * 8 + 1..(0x83D9 * 8 + 1 + ACTIVE_PKM_BIT_LEN * ACTIVE_PKM_COUNT);

pub mod pokemon {
    use std::ops::Range;

    pub const VALID: usize = 0;
    // Bits 1 to 4 are unknown.
    pub const LEVEL: Range<usize> = 5..12;
    pub const MET_AT: Range<usize> = 12..20;
    pub const MET_FLOOR: Range<usize> = 20..27;
    // Bit 27 is unknown,
    pub const IQ: Range<usize> = 28..38;
    pub const ROASTER_NUMBER: Range<usize> = 38..48;
    // Bits 48 to 69 are unknown.
    pub const ID: Range<usize> = 70..81;
    pub const CURRENT_HP: Range<usize> = 81..91;
    pub const MAX_HP: Range<usize> = 91..101;
    pub const ATTACK: Range<usize> = 101..109;
    pub const SP_ATTACK: Range<usize> = 109..117;
    pub const DEFENSE: Range<usize> = 117..125;
    pub const SP_DEFENSE: Range<usize> = 125..133;
    pub const EXP: Range<usize> = 133..157;
    pub const MOVE_1: Range<usize> = 157..186;
    pub const MOVE_2: Range<usize> = 186..215;
    pub const MOVE_3: Range<usize> = 215..244;
    pub const MOVE_4: Range<usize> = 244..273;
    // bits 275 to 377 are unknown
    pub const IQ_MAP: Range<usize> = 378..447;
    pub const TACTIC: Range<usize> = 447..451;
    // bits 451 to 465 are unknown
    pub const NAME: Range<usize> = 466..546;
}

pub mod moves {
    use std::ops::Range;

    pub const VALID: usize = 0;
    pub const LINKED: usize = 1;
    pub const SWITCHED: usize = 2;
    pub const SET: usize = 3;
    pub const SEALED: usize = 4;
    pub const ID: Range<usize> = 5..15;
    pub const PP: Range<usize> = 15..22;
    pub const POWER_BOOST: Range<usize> = 22..29;
}
