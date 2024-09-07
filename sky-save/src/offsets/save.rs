use std::ops::Range;

/// Save checksums
pub const PRIMARY_SAVE: Range<usize> = 0x0..0xB65C;
pub const PRIMARY_READ_CHECKSUM: Range<usize> = PRIMARY_SAVE.start..PRIMARY_SAVE.start + 4;
pub const PRIMARY_CHECKSUM: Range<usize> = PRIMARY_SAVE.start + 4..PRIMARY_SAVE.end;

pub const BACKUP_SAVE: Range<usize> = 0xC800..0x17E5C;
pub const BACKUP_READ_CHECKSUM: Range<usize> = BACKUP_SAVE.start..BACKUP_SAVE.start + 4;
pub const BACKUP_CHECKSUM: Range<usize> = BACKUP_SAVE.start + 4..BACKUP_SAVE.end;

pub const QUICKSAVE: Range<usize> = 0x19000..0x1E800;
pub const QUICKSAVE_READ_CHECKSUM: Range<usize> = QUICKSAVE.start..QUICKSAVE.start + 4;
pub const QUICKSAVE_CHECKSUM: Range<usize> = QUICKSAVE.start + 4..QUICKSAVE.end;
