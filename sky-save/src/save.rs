//! Handles loading and storing the save data.

use crate::error::SaveError;
use crate::offsets::{active, general, save, stored};
use crate::{ActivePokemon, PmdString, StoredPokemon};
use arrayvec::ArrayVec;
use bitvec::bitarr;
use bitvec::field::BitField;
use bitvec::order::Lsb0;
use bitvec::slice::BitSlice;
use bitvec::view::BitView;
use std::fs;
use std::ops::Range;
use std::path::Path;

/// File size must be at least 128Kib.
const MIN_SAVE_LEN: usize = 0x20000;

fn checksum(data: &[u8], data_range: Range<usize>) -> [u8; 4] {
    (data[data_range]
        .chunks(4)
        .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap())) // Safe, four bytes.O
        .fold(0u64, |acc, u| acc + u as u64) as u32)
        .to_le_bytes()
}

fn load_save_slice(data: &[u8], active_save_block: ActiveSaveBlock, range: Range<usize>) -> &[u8] {
    &data[range.start + active_save_block as usize..range.end + active_save_block as usize]
}

fn store_save_slice(
    data: &mut [u8],
    active_save_block: ActiveSaveBlock,
    range: Range<usize>,
    value: &[u8],
) {
    data[range.start + active_save_block as usize..range.end + active_save_block as usize]
        .copy_from_slice(value);
}

fn load_save_bits(
    data: &BitSlice<u8, Lsb0>,
    active_save_block: ActiveSaveBlock,
    range: Range<usize>,
) -> &BitSlice<u8, Lsb0> {
    &data[range.start + active_save_block as usize * 8..range.end + active_save_block as usize * 8]
}

fn store_save_bits(
    data: &mut BitSlice<u8, Lsb0>,
    active_save_block: ActiveSaveBlock,
    range: Range<usize>,
    value: &BitSlice<u8, Lsb0>,
) {
    data[range.start + active_save_block as usize * 8..range.end + active_save_block as usize * 8]
        .copy_from_bitslice(value);
}

/// The current active save block.
/// Holds it's start offset.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum ActiveSaveBlock {
    Primary = save::PRIMARY_SAVE.start,
    Backup = save::BACKUP_SAVE.start,
}

/// Holds general information about the saved game.
#[derive(Debug)]
pub struct General {
    pub team_name: PmdString,
    pub held_money: u32,
    pub sp_episode_held_money: u32,
    pub stored_money: u32,
    pub number_of_adventures: i32,
    pub explorer_rank: u32,
}

impl General {
    fn load(data: &[u8], active_save_block: ActiveSaveBlock) -> Self {
        let team_name = load_save_slice(data, active_save_block, general::TEAM_NAME);
        let held_money = load_save_bits(
            data.view_bits(),
            active_save_block,
            general::HELD_MONEY_BITS,
        );
        let sp_episode_held_money = load_save_bits(
            data.view_bits(),
            active_save_block,
            general::SP_EPISODE_HELD_MONEY_BITS,
        );
        let stored_money = load_save_bits(
            data.view_bits(),
            active_save_block,
            general::STORED_MONEY_BITS,
        );
        let number_of_adventures =
            load_save_slice(data, active_save_block, general::NUMBER_OF_ADVENTURERS)
                .try_into()
                .unwrap();
        let explorer_rank = load_save_slice(data, active_save_block, general::EXPLORER_RANK)
            .try_into()
            .unwrap();

        Self {
            team_name: PmdString::from(team_name),
            held_money: held_money.load_le(),
            sp_episode_held_money: sp_episode_held_money.load_le(),
            stored_money: stored_money.load_le(),
            number_of_adventures: i32::from_le_bytes(number_of_adventures),
            explorer_rank: u32::from_le_bytes(explorer_rank),
        }
    }

    fn save(&self, data: &mut [u8], active_save_block: ActiveSaveBlock) {
        store_save_slice(
            data,
            active_save_block,
            general::TEAM_NAME,
            self.team_name.to_save_bytes().as_slice(),
        );

        store_save_bits(
            data.view_bits_mut(),
            active_save_block,
            general::HELD_MONEY_BITS,
            &self.held_money.to_le_bytes().view_bits::<Lsb0>()[0..24],
        );
        store_save_bits(
            data.view_bits_mut(),
            active_save_block,
            general::SP_EPISODE_HELD_MONEY_BITS,
            &self.sp_episode_held_money.to_le_bytes().view_bits::<Lsb0>()[0..24],
        );
        store_save_bits(
            data.view_bits_mut(),
            active_save_block,
            general::STORED_MONEY_BITS,
            &self.stored_money.to_le_bytes().view_bits::<Lsb0>()[0..24],
        );
        store_save_slice(
            data,
            active_save_block,
            general::NUMBER_OF_ADVENTURERS,
            &self.number_of_adventures.to_le_bytes(),
        );
        store_save_slice(
            data,
            active_save_block,
            general::EXPLORER_RANK,
            &self.explorer_rank.to_le_bytes(),
        );
    }
}

/// The main structure of `sky-save`.
/// Contains the save data bytes and every structure the library parses.
/// Selectively loads data from the `active_save_block`.
#[derive(Debug)]
pub struct SkySave {
    pub data: Vec<u8>,
    pub active_save_block: ActiveSaveBlock,
    pub quicksave_valid: bool,

    pub general: General,
    pub stored_pokemon: ArrayVec<StoredPokemon, 550>,
    pub active_pokemon: ArrayVec<ActivePokemon, 4>,
}

impl SkySave {
    /// Load and validates the save data from a slice of bytes.
    /// The save data is validated by checking its length and calculating the checksums.
    /// The save file is divided into three blocks: primary, backup, and quicksave.
    /// For each block, the first four bytes are the checksum, and it is calculated as follows:
    /// - Convert every four bytes, from start to end, to unsigned 32-bit integers. And then sum them together.
    /// - Truncate the result to a 32-bit integer.
    /// - Convert the result to little-endian bytes.
    /// - Compare with bytes 0 to 3 to check for validity.
    ///
    /// After validation, every structure is parsed from the save data.
    pub fn from_slice<S: AsRef<[u8]>>(data: S) -> Result<Self, SaveError> {
        let data = data.as_ref();

        if data.len() < MIN_SAVE_LEN {
            return Err(SaveError::InvalidSize);
        }

        let pri_read: [u8; 4] = data[save::PRIMARY_READ_CHECKSUM].try_into().unwrap(); // Safe, four bytes.
        let backup_read: [u8; 4] = data[save::BACKUP_READ_CHECKSUM].try_into().unwrap(); // Safe, four bytes.
        let quick_read: [u8; 4] = data[save::QUICKSAVE_READ_CHECKSUM].try_into().unwrap(); // Safe, four bytes.

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

        let active_save_block = if pri_matches {
            ActiveSaveBlock::Primary
        } else {
            ActiveSaveBlock::Backup
        };

        let general = General::load(data, active_save_block);
        let bits = load_save_bits(data.view_bits(), active_save_block, stored::STORED_PKM_BITS);

        let stored_pokemon: ArrayVec<StoredPokemon, 550> = bits
            .chunks(stored::STORED_PKM_BIT_LEN)
            .map(StoredPokemon::from_bitslice)
            .collect();

        let bits = load_save_bits(data.view_bits(), active_save_block, active::ACTIVE_PKM_BITS);
        let active_pokemon: ArrayVec<ActivePokemon, 4> = bits
            .chunks(active::ACTIVE_PKM_BIT_LEN)
            .map(ActivePokemon::from_bitslice)
            .collect();

        Ok(SkySave {
            data: data.to_vec(),
            active_save_block,
            quicksave_valid: quick_matches,
            general,
            stored_pokemon,
            active_pokemon,
        })
    }

    /// Loads save data from a file.
    pub fn open<P: AsRef<Path>>(filename: P) -> Result<Self, SaveError> {
        let data = fs::read(filename).map_err(SaveError::Io)?;
        Self::from_slice(&data)
    }

    /// Recalculates the checksums for each save block.
    /// Writes the checksums to the save data.
    pub fn fix_checksums(&mut self) {
        let pri_sum = checksum(&self.data, save::PRIMARY_CHECKSUM);
        let backup_sum = checksum(&self.data, save::BACKUP_CHECKSUM);
        let quick_sum = checksum(&self.data, save::QUICKSAVE_CHECKSUM);

        self.data[save::PRIMARY_READ_CHECKSUM].copy_from_slice(&pri_sum);
        self.data[save::BACKUP_READ_CHECKSUM].copy_from_slice(&backup_sum);
        self.data[save::QUICKSAVE_READ_CHECKSUM].copy_from_slice(&quick_sum);
    }

    /// Saves all changes to `data`. Recalculates the checksums and writes to a file.
    pub fn save<P: AsRef<Path>>(&mut self, filename: P) -> Result<(), SaveError> {
        let active_range = match self.active_save_block {
            ActiveSaveBlock::Primary => save::PRIMARY_SAVE,
            ActiveSaveBlock::Backup => save::BACKUP_SAVE,
        };

        let backup = match self.active_save_block {
            ActiveSaveBlock::Primary => save::BACKUP_SAVE.start,
            ActiveSaveBlock::Backup => save::PRIMARY_SAVE.start,
        };

        self.general.save(&mut self.data, self.active_save_block);

        // Saving does not allocate on the heap.
        let stored = self
            .stored_pokemon
            .iter()
            .map(StoredPokemon::to_bits)
            .enumerate()
            .fold(
                bitarr![u8, Lsb0; 0; stored::STORED_PKM_BIT_LEN * stored::STORED_PKM_COUNT],
                |mut acc, (idx, a)| {
                    acc[idx * stored::STORED_PKM_BIT_LEN..(idx + 1) * stored::STORED_PKM_BIT_LEN]
                        .copy_from_bitslice(&a[0..stored::STORED_PKM_BIT_LEN]);
                    acc
                },
            );

        store_save_bits(
            self.data.view_bits_mut(),
            self.active_save_block,
            stored::STORED_PKM_BITS,
            &stored.as_bitslice()[0..stored::STORED_PKM_BIT_LEN * stored::STORED_PKM_COUNT],
        );

        let active = self
            .active_pokemon
            .iter()
            .map(ActivePokemon::to_bits)
            .enumerate()
            .fold(
                bitarr![u8, Lsb0; 0; active::ACTIVE_PKM_BIT_LEN * active::ACTIVE_PKM_COUNT],
                |mut acc, (idx, a)| {
                    acc[idx * active::ACTIVE_PKM_BIT_LEN..(idx + 1) * active::ACTIVE_PKM_BIT_LEN]
                        .copy_from_bitslice(&a[0..active::ACTIVE_PKM_BIT_LEN]);
                    acc
                },
            );

        store_save_bits(
            self.data.view_bits_mut(),
            self.active_save_block,
            active::ACTIVE_PKM_BITS,
            active.as_bitslice(),
        );

        self.data.copy_within(active_range, backup);
        self.fix_checksums();

        fs::write(filename, &self.data).map_err(SaveError::Io)
    }
}
