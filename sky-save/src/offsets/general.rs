use std::ops::Range;

/// General save data
pub const TEAM_NAME: Range<usize> = 0x994E..0x9958;
pub const HELD_MONEY: Range<usize> = 0x990C..0x9910;
pub const SP_EPISODE_HELD_MONEY: Range<usize> = 0x990F..0x9913;
pub const STORED_MONEY: Range<usize> = 0x9915..0x9919;
pub const EXPLORER_RANK: Range<usize> = 0x9958..0x995C;
pub const NUMBER_OF_ADVENTURERS: Range<usize> = 0x8B70..0x8B74;
