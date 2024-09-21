use crate::consts::MIN_SAVE_LEN;
use crate::error::SaveError;
use crate::offsets::save::{BACKUP_SAVE, PRIMARY_SAVE};
use crate::offsets::{active, general, save, stored};
use crate::{ActivePokemon, ActivePokemonBits, PmdString, StoredPokemon, StoredPokemonBits};
use bitvec::bitarr;
use bitvec::field::BitField;
use bitvec::order::Lsb0;
use bitvec::slice::BitSlice;
use bitvec::view::BitView;
use std::fs;
use std::ops::Range;
use std::path::Path;

fn checksum(data: &[u8], data_range: Range<usize>) -> [u8; 4] {
    (data[data_range]
        .chunks(4)
        .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap())) // Safe, four bytes.O
        .fold(0u64, |acc, u| acc + u as u64) as u32)
        .to_le_bytes()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum ActiveSaveBlock {
    Primary = PRIMARY_SAVE.start,
    Backup = BACKUP_SAVE.start,
}

#[derive(Debug)]
pub struct SkySave {
    pub data: Vec<u8>,
    pub active_save_block: ActiveSaveBlock,
    pub quicksave_valid: bool,
}

impl SkySave {
    fn load_save_slice(&self, range: Range<usize>) -> &[u8] {
        &self.data[range.start + self.active_save_block as usize
            ..range.end + self.active_save_block as usize]
    }

    fn load_save_bits(&self, range: Range<usize>) -> &BitSlice<u8, Lsb0> {
        &self.data.view_bits()[range.start + self.active_save_block as usize * 8
            ..range.end + self.active_save_block as usize * 8]
    }

    /// Load and validates the save data from a slice of bytes.
    /// The save data is validated by checking its length and calculating the checksums.
    /// The save file is divided into three blocks: primary, backup, and quicksave.
    /// For each block, the first four bytes are the checksum, and it is calculated as follows:
    /// - Convert every four bytes, from start to end, to unsigned 32-bit integers. And then sum them together.
    /// - Truncate the result to a 32-bit integer.
    /// - Convert the result to little-endian bytes.
    /// - Compare with bytes 0 to 3 to check for validity.
    pub fn from_slice<S: AsRef<[u8]>>(data: S) -> Result<Self, SaveError> {
        let data = data.as_ref();

        if data.len() < MIN_SAVE_LEN {
            return Err(SaveError::InvalidSize);
        }

        let pri_read: [u8; 4] = data[save::PRIMARY_READ_CHECKSUM].try_into().unwrap(); // Safe, four bytes.
        let backup_read: [u8; 4] = data[save::BACKUP_READ_CHECKSUM].try_into().unwrap(); // Safe, four bytes.
        let quick_read: [u8; 4] = data[save::QUICKSAVE_READ_CHECKSUM].try_into().unwrap(); // Safe, four bytes.

        // 0xB6A isn't divisible by 4. We end up with a reminder of 2 bytes and need to count for them.
        let pri_sum = checksum(data, save::PRIMARY_CHECKSUM);
        let backup_sum = checksum(data, save::BACKUP_CHECKSUM);
        let quick_sum = checksum(data, save::QUICKSAVE_CHECKSUM);

        let pri_matches = pri_sum == pri_read;
        let backup_matches = backup_sum == backup_read;
        let quick_matches = quick_sum == quick_read;

        if !pri_matches && !backup_matches {
            return Err(SaveError::InvalidChecksum {
                pri_expected: pri_read,
                pri_found: pri_sum,
                bak_expected: backup_read,
                bak_found: backup_sum,
            });
        }

        Ok(SkySave {
            data: data.as_ref().to_vec(),
            active_save_block: if pri_matches {
                ActiveSaveBlock::Primary
            } else {
                ActiveSaveBlock::Backup
            },
            quicksave_valid: quick_matches,
        })
    }

    /// Loads save data from file.
    pub fn open<P: AsRef<Path>>(filename: P) -> Result<Self, SaveError> {
        let data = fs::read(filename).map_err(SaveError::Io)?;
        Self::from_slice(&data)
    }

    pub fn team_name(&self) -> PmdString {
        let bytes = self.load_save_slice(general::TEAM_NAME);
        PmdString::from(bytes)
    }

    pub fn held_money(&self) -> u32 {
        let bits = &self
            .load_save_slice(general::HELD_MONEY)
            .view_bits::<Lsb0>()[6..30];
        bits.load_le::<u32>()
    }

    pub fn sp_episode_held_money(&self) -> u32 {
        let bits = &self
            .load_save_slice(general::SP_EPISODE_HELD_MONEY)
            .view_bits::<Lsb0>()[6..30];
        bits.load_le::<u32>()
    }

    pub fn stored_money(&self) -> u32 {
        let bits = &self
            .load_save_slice(general::STORED_MONEY)
            .view_bits::<Lsb0>()[6..30];
        bits.load_le::<u32>()
    }

    pub fn number_of_adventurers(&self) -> i32 {
        let bytes = self.load_save_slice(general::NUMBER_OF_ADVENTURERS);
        i32::from_le_bytes(bytes.try_into().unwrap()) // Safe, four bytes.
    }

    pub fn explorer_rank(&self) -> u32 {
        let bytes = self.load_save_slice(general::EXPLORER_RANK);
        u32::from_le_bytes(bytes.try_into().unwrap()) // Safe, four bytes.
    }

    pub fn stored_pokemon(&self) -> Box<[StoredPokemon]> {
        let bits = &self.data.view_bits::<Lsb0>()[stored::STORED_PKM_BITS];
        bits.chunks(stored::STORED_PKM_BIT_LEN)
            .map(|c| {
                let mut data: StoredPokemonBits = bitarr!(u8, Lsb0; 0; 362);
                data[0..362].copy_from_bitslice(c);
                StoredPokemon(data)
            })
            .collect()
    }

    pub fn active_pokemon(&self) -> Box<[ActivePokemon]> {
        let bits: &BitSlice<u8> = self.load_save_bits(active::ACTIVE_PKM_BITS);
        bits.chunks(active::ACTIVE_PKM_BIT_LEN)
            .map(|c| {
                let mut data: ActivePokemonBits = bitarr!(u8, Lsb0; 0; 546);
                data[0..546].copy_from_bitslice(c);
                ActivePokemon(data)
            })
            .collect()
    }
}
