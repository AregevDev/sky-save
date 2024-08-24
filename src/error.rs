use std::io;
use thiserror::Error;

#[derive(Debug, Default, Error)]
pub enum SaveError {
    #[default]
    #[error("Unknown error")]
    Unknown,

    #[error("Error loading save file: {0:?}")]
    Io(#[from] io::Error),

    #[error("File size must be at least 128Kib.")]
    InvalidSize,

    #[error("Invalid save checksum in block {block:?}:\nExpected: {expected:?}, Found: {found:?}.")]
    InvalidChecksum {
        block: u8,
        expected: [u8; 4],
        found: [u8; 4],
    },
}
