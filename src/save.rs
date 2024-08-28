use crate::consts::*;
use crate::encoding::pmd_to_string;
use crate::error::SaveError;
use crate::EncodingError;
use bitvec::field::BitField;
use bitvec::order::Lsb0;
use bitvec::view::BitView;
use std::fs;
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
        let chk = self.data[PRIMARY_SAVE_START + 4..PRIMARY_SAVE_END]
            .chunks(4)
            .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap())) // Safe, four bytes.
            .fold(0u64, |acc, u| acc + u as u64) as u32;

        let calc = chk.to_le_bytes();
        let block0: [u8; 4] = self.data[BACKUP_SAVE_START..BACKUP_SAVE_START + 4].try_into().unwrap(); // Safe, four bytes.
        let block1: [u8; 4] = self.data
            [BACKUP_SAVE_START..BACKUP_SAVE_START + 4]
            .try_into()
            .unwrap(); // Safe, four bytes.

        if calc != block0 {
            return Err(SaveError::InvalidChecksum {
                block: 0,
                expected: block0,
                found: calc,
            });
        } else if calc != block1 {
            return Err(SaveError::InvalidChecksum {
                block: 1,
                expected: block1,
                found: calc,
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

    pub fn team_name(&self) -> Result<String, EncodingError> {
        let bytes = &self.data[TEAM_NAME_START..TEAM_NAME_END];
        pmd_to_string(bytes)
    }

    pub fn held_money(&self) -> u32 {
        let bits = &self.data[HELD_MONEY_START..HELD_MONEY_END].view_bits::<Lsb0>()[6..30];
        bits.load_le::<u32>()
    }

    pub fn sp_episode_held_money(&self) -> u32 {
        let bits = &self.data[SP_EPISODE_HELD_MONEY_START..SP_EPISODE_HELD_MONEY_END]
            .view_bits::<Lsb0>()[6..30];
        bits.load_le::<u32>()
    }

    pub fn stored_money(&self) -> u32 {
        let bits = &self.data[STORED_MONEY_START..STORED_MONEY_END].view_bits::<Lsb0>()[6..30];
        bits.load_le::<u32>()
    }

    pub fn number_of_adventurers(&self) -> i32 {
        let bytes = &self.data[NUMBER_OF_ADVENTURERS_START..NUMBER_OF_ADVENTURERS_END];
        i32::from_le_bytes(bytes.try_into().unwrap()) // Safe, four bytes.
    }

    pub fn explorer_rank(&self) -> u32 {
        let bytes = &self.data[EXPLORER_RANK_START..EXPLORER_RANK_END];
        u32::from_le_bytes(bytes.try_into().unwrap()) // Safe, four bytes.
    }
}
