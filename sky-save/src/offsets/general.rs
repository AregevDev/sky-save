//! General save data offsets

use std::ops::Range;

pub const TEAM_NAME: Range<usize> = 0x994E..0x9958;
pub const HELD_MONEY_BITS: Range<usize> = 0x990C * 8 + 6..0x990C * 8 + 6 + 24;
pub const SP_EPISODE_HELD_MONEY_BITS: Range<usize> = 0x990F * 8 + 6..0x990F * 8 + 6 + 24;
pub const STORED_MONEY_BITS: Range<usize> = 0x9915 * 8 + 6..0x9915 * 8 + 6 + 24;
pub const EXPLORER_RANK: Range<usize> = 0x9958..0x995C;
pub const NUMBER_OF_ADVENTURERS: Range<usize> = 0x8B70..0x8B74;
