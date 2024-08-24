use crate::consts::{MIN_SAVE_LEN, SAVE_BLOCK_SIZE, SAVE_CHECKSUM_END};
use crate::error::SaveError;
use std::fs;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
pub struct SkySave {
    pub data: Vec<u8>,
}

impl SkySave {
    /// Checks if the save data is valid by checking the data length and calculates the checksum.
    /// This function is called when loading the save data from bytes.
    /// The checksum is stored in bytes 0 to 3 and computed as follows:
    /// - Convert every four bytes, from byte 4 to byte 46684, to unsigned 32-bit integers. And then sum them together.
    /// - Truncate the result to a 32-bit integer.
    /// - Convert the result to little-endian bytes.
    /// - Compare with bytes 0 to 3 to check for validity.
    pub fn validate(&self) -> Result<(), SaveError> {
        if self.data.len() < MIN_SAVE_LEN {
            return Err(SaveError::InvalidSize);
        }

        // 0xB6A isn't divisible by 4. We end up with a reminder of 2 bytes and need to count for them.
        let chk = self.data[4..SAVE_CHECKSUM_END + 2]
            .chunks(4)
            .enumerate()
            .map(|chunk| u32::from_le_bytes(chunk.1.try_into().unwrap()))
            .fold(0u64, |acc, u| acc + u as u64) as u32;

        let calc = chk.to_le_bytes();
        let block0: [u8; 4] = self.data[0..4].try_into().unwrap();
        let block1: [u8; 4] = self.data[SAVE_BLOCK_SIZE..SAVE_BLOCK_SIZE + 4]
            .try_into()
            .unwrap(); // Safe, slice is always four bytes long.

        if calc != block0 {
            return Err(SaveError::InvalidChecksum {
                block: 0,
                expected: block0,
                actual: calc,
            });
        } else if calc != block1 {
            return Err(SaveError::InvalidChecksum {
                block: 1,
                expected: block1,
                actual: calc,
            });
        }

        Ok(())
    }

    /// Load save data a slice of bytes.
    pub fn from_slice(data: &[u8]) -> Result<Self, SaveError> {
        let res = SkySave {
            data: data.to_vec(),
        };
        res.validate()?;

        Ok(res)
    }

    /// Loads save data from file.
    pub fn open<P: AsRef<Path>>(filename: P) -> Result<Self, SaveError> {
        let data = fs::read(filename).map_err(SaveError::Io)?;
        Self::from_slice(&data)
    }
}
