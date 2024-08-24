/// File size must be at least 128Kib
pub const MIN_SAVE_LEN: usize = 131072;

/// The two main save blocks are 50Kib each.
pub const SAVE_BLOCK_SIZE: usize = 51200;

/// Checksum calculation end offset.
pub const SAVE_CHECKSUM_END: usize = 0xB65A;
