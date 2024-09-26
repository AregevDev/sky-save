use std::ops::Range;

/// Stored Pokemon
pub const STORED_PKM_BIT_LEN: usize = 362;
pub const STORED_PKM_COUNT: usize = 720;
pub const STORED_PKM_BITS: Range<usize> =
    0x464 * 8..(0x464 * 8 + STORED_PKM_BIT_LEN * STORED_PKM_COUNT);

pub mod pokemon {
    use std::ops::Range;

    pub const VALID: usize = 0;
    pub const LEVEL: Range<usize> = 1..8;
    pub const ID: Range<usize> = 8..19;
    pub const MET_AT: Range<usize> = 19..27;
    pub const MET_FLOOR: Range<usize> = 27..34;
    pub const UNKNOWN: usize = 34;
    pub const EVOLVED_AT_1: Range<usize> = 35..42;
    pub const EVOLVED_AT_2: Range<usize> = 42..49;
    pub const IQ: Range<usize> = 49..59;
    pub const HP: Range<usize> = 59..69;
    pub const ATTACK: Range<usize> = 69..77;
    pub const SP_ATTACK: Range<usize> = 77..85;
    pub const DEFENSE: Range<usize> = 85..93;
    pub const SP_DEFENSE: Range<usize> = 93..101;
    pub const EXP: Range<usize> = 101..125;
    pub const IQ_MAP: Range<usize> = 125..194;
    pub const TACTIC: Range<usize> = 194..198;
    pub const MOVE_1: Range<usize> = 198..219;
    pub const MOVE_2: Range<usize> = 219..240;
    pub const MOVE_3: Range<usize> = 240..261;
    pub const MOVE_4: Range<usize> = 261..282;
    pub const NAME: Range<usize> = 282..362;
}

pub mod moves {
    use std::ops::Range;

    pub const VALID: usize = 0;
    pub const LINKED: usize = 1;
    pub const SWITCHED: usize = 2;
    pub const SET: usize = 3;
    pub const ID: Range<usize> = 4..14;
    pub const POWER_BOOST: Range<usize> = 14..21;
}
