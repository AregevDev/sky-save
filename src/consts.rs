/// File size must be at least 128Kib
pub const MIN_SAVE_LEN: usize = 0x20000;

/// The two main save blocks are 50Kib each.
pub const SAVE_BLOCK_SIZE: usize = 0xC800;

/// Save checksums
pub const PRIMARY_SAVE_START: usize = 0x0;
pub const PRIMARY_SAVE_END: usize = 0xB65C;

pub const BACKUP_SAVE_START: usize = 0xC800;
pub const BACKUP_SAVE_END: usize = 0x17E5C;

pub const QUICKSAVE_START: usize = 0x19000;
pub const QUICKSAVE_END: usize = 0x1E800;

/// General save data
pub const TEAM_NAME_START: usize = 0x994E;
pub const TEAM_NAME_END: usize = 0x9958;
pub const HELD_MONEY_START: usize = 0x990C;
pub const HELD_MONEY_END: usize = 0x9910;
pub const SP_EPISODE_HELD_MONEY_START: usize = 0x990F;
pub const SP_EPISODE_HELD_MONEY_END: usize = 0x9913;
pub const STORED_MONEY_START: usize = 0x9915;
pub const STORED_MONEY_END: usize = 0x9919;
pub const EXPLORER_RANK_START: usize = 0x9958;
pub const EXPLORER_RANK_END: usize = 0x995C;
pub const NUMBER_OF_ADVENTURERS_START: usize = 0x8B70;
pub const NUMBER_OF_ADVENTURERS_END: usize = 0x8B74;

/// Stored Pokemon
pub const STORED_PKM_START: usize = 0x464;
pub const STORED_PKM_BIT_LEN: usize = 362;
pub const STORED_PKM_COUNT: usize = 720;
