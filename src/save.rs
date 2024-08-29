use crate::consts::*;
use crate::encoding::pmd_to_string;
use crate::error::SaveError;
use crate::{EncodingError, StoredPokemon};
use bitvec::field::BitField;
use bitvec::order::Lsb0;
use bitvec::slice::BitSlice;
use bitvec::view::BitView;
use std::fs;
use std::ops::Range;
use std::path::Path;

#[derive(Debug)]
pub struct SkySave {
    pub data: Vec<u8>,
}

impl SkySave {
    fn checksum(&self, data_range: Range<usize>) -> [u8; 4] {
        let sum = self.data[data_range]
            .chunks(4)
            .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap())) // Safe, four bytes.
            .fold(0u64, |acc, u| acc + u as u64) as u32;
        sum.to_le_bytes()
    }

    /// Validates the save data by checking its length and calculating the checksums.
    /// The save file is divided into three blocks: primary, backup, and quicksave.
    /// For each block, the first four bytes are the checksum, and it is calculated as follows:
    /// - Convert every four bytes, from start to end, to unsigned 32-bit integers. And then sum them together.
    /// - Truncate the result to a 32-bit integer.
    /// - Convert the result to little-endian bytes.
    /// - Compare with bytes 0 to 3 to check for validity.
    pub fn validate(&self) -> Result<(), SaveError> {
        if self.data.len() < MIN_SAVE_LEN {
            return Err(SaveError::InvalidSize);
        }

        let pri_read: [u8; 4] = self.data[PRIMARY_SAVE_START..PRIMARY_SAVE_START + 4]
            .try_into()
            .unwrap(); // Safe, four bytes.

        let backup_read: [u8; 4] = self.data[BACKUP_SAVE_START..BACKUP_SAVE_START + 4]
            .try_into()
            .unwrap(); // Safe, four bytes.

        let quick_read: [u8; 4] = self.data[QUICKSAVE_START..QUICKSAVE_START + 4]
            .try_into()
            .unwrap(); // Safe, four bytes.

        // 0xB6A isn't divisible by 4. We end up with a reminder of 2 bytes and need to count for them.
        let pri_sum = self.checksum(PRIMARY_SAVE_START + 4..PRIMARY_SAVE_END);
        let backup_sum = self.checksum(BACKUP_SAVE_START + 4..BACKUP_SAVE_END);
        let quick_sum = self.checksum(QUICKSAVE_START + 4..QUICKSAVE_END);

        if pri_sum != pri_read {
            return Err(SaveError::InvalidChecksum {
                block: 0,
                expected: pri_read,
                found: pri_sum,
            });
        }

        if backup_sum != backup_read {
            return Err(SaveError::InvalidChecksum {
                block: 1,
                expected: backup_read,
                found: backup_sum,
            });
        }

        if quick_sum != quick_read {
            return Err(SaveError::InvalidChecksum {
                block: 2,
                expected: quick_read,
                found: quick_sum,
            });
        }

        Ok(())
    }

    /// Load save data from a slice of bytes.
    pub fn from_slice<S: AsRef<[u8]>>(data: S) -> Result<Self, SaveError> {
        let res = SkySave {
            data: data.as_ref().to_vec(),
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

    pub fn stored_pokemon(&self) -> Vec<StoredPokemon> {
        let bits: &BitSlice<u8> = &self.data.view_bits::<Lsb0>()
            [STORED_PKM_START * 8..STORED_PKM_START * 8 + STORED_PKM_BIT_LEN * STORED_PKM_COUNT];
        bits.chunks(STORED_PKM_BIT_LEN)
            .map(|c| StoredPokemon { data: c })
            .collect()
    }
}
