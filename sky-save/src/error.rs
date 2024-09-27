//! The library's error type.

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

    #[error(
        "Invalid save checksum in neither primary or backup save blocks:\n\
        Primary: expected {pri_expected:?}, Found: {pri_found:?}\n\
        Backup: expected {bak_expected:?}, Found: {bak_found:?}"
    )]
    InvalidChecksum {
        pri_expected: [u8; 4],
        pri_found: [u8; 4],
        bak_expected: [u8; 4],
        bak_found: [u8; 4],
    },
}

#[derive(Debug, Error)]
pub enum EncodingError {
    #[error("Invalid PMD character: {0}")]
    InvalidPmdCharacter(String),
    #[error("PMD String must not exceed 10 characters")]
    InvalidPmdStringLen,
}
