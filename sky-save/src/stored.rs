//! Handles loading and storing the stored Pokémon.

use crate::offsets::stored::{moves, pokemon, STORED_MOVE_BIT_LEN, STORED_PKM_BIT_LEN};
use crate::PmdString;
use bitvec::prelude::*;
use bitvec::BitArr;

pub type IqMapBits = BitArr!(for 69, in u8, Lsb0);

/// A static `BitArray` representing the bits of a `StoredPokemon`.
pub type StoredPokemonBits = BitArr!(for STORED_PKM_BIT_LEN, in u8, Lsb0);
/// A static `BitArray` representing the bits of a `StoredMove`.
pub type StoredMoveBits = BitArr!(for STORED_MOVE_BIT_LEN, in u8, Lsb0);

/// Represents each of the four moves in a `StoredPokemon`.
#[derive(Debug, Default, Clone)]
pub struct StoredMove {
    pub valid: bool,
    pub linked: bool,
    pub switched: bool,
    pub set: bool,
    pub id: u16,
    pub power_boost: u8,
}

impl StoredMove {
    pub fn from_bitslice(bits: &BitSlice<u8, Lsb0>) -> Self {
        Self {
            valid: bits[moves::VALID],
            linked: bits[moves::LINKED],
            switched: bits[moves::SWITCHED],
            set: bits[moves::SET],
            id: bits[moves::ID].load_le(),
            power_boost: bits[moves::POWER_BOOST].load_le(),
        }
    }

    pub fn to_bits(&self) -> StoredMoveBits {
        let mut bits = bitarr![u8, Lsb0; 0; STORED_MOVE_BIT_LEN];

        bits.set(moves::VALID, self.valid);
        bits.set(moves::LINKED, self.linked);
        bits.set(moves::SWITCHED, self.switched);
        bits.set(moves::SET, self.set);
        bits[moves::ID].store_le(self.id);
        bits[moves::POWER_BOOST].store_le(self.power_boost);

        bits
    }
}

/// Represents a recruited Pokémon in Chimecho's Assembly.
/// Holds information that isn't critical in dungeon mode.
#[derive(Debug, Default, Clone)]
pub struct StoredPokemon {
    pub valid: bool,
    pub level: u8,
    pub id: u16,
    pub met_at: u8,
    pub met_floor: u8,
    pub unknown: bool,
    pub evolved_at_1: u8,
    pub evolved_at_2: u8,
    pub iq: u16,
    pub hp: u16,
    pub attack: u8,
    pub sp_attack: u8,
    pub defense: u8,
    pub sp_defense: u8,
    pub exp: u32,
    pub iq_map: IqMapBits,
    pub tactic: u8,
    pub move_1: StoredMove,
    pub move_2: StoredMove,
    pub move_3: StoredMove,
    pub move_4: StoredMove,
    pub name: PmdString,
}

impl StoredPokemon {
    pub fn from_bitslice(value: &BitSlice<u8, Lsb0>) -> Self {
        let mut iq: IqMapBits = bitarr![u8, Lsb0; 0; 69];
        iq[0..69].copy_from_bitslice(&value[pokemon::IQ_MAP]);

        let name_bytes = &value[pokemon::NAME];

        Self {
            valid: value[pokemon::VALID],
            level: value[pokemon::LEVEL].load_le(),
            id: value[pokemon::ID].load_le(),
            met_at: value[pokemon::MET_AT].load_le(),
            met_floor: value[pokemon::MET_FLOOR].load_le(),
            unknown: value[pokemon::UNKNOWN],
            evolved_at_1: value[pokemon::EVOLVED_AT_1].load_le(),
            evolved_at_2: value[pokemon::EVOLVED_AT_2].load_le(),
            iq: value[pokemon::IQ].load_le(),
            hp: value[pokemon::HP].load_le(),
            attack: value[pokemon::ATTACK].load_le(),
            sp_attack: value[pokemon::SP_ATTACK].load_le(),
            defense: value[pokemon::DEFENSE].load_le(),
            sp_defense: value[pokemon::SP_DEFENSE].load_le(),
            exp: value[pokemon::EXP].load_le(),
            iq_map: iq,
            tactic: value[pokemon::TACTIC].load_le(),
            move_1: StoredMove::from_bitslice(&value[pokemon::MOVE_1]),
            move_2: StoredMove::from_bitslice(&value[pokemon::MOVE_2]),
            move_3: StoredMove::from_bitslice(&value[pokemon::MOVE_3]),
            move_4: StoredMove::from_bitslice(&value[pokemon::MOVE_4]),
            name: PmdString::from(name_bytes),
        }
    }

    pub fn to_bits(&self) -> StoredPokemonBits {
        let mut bits = bitarr![u8, Lsb0; 0; STORED_PKM_BIT_LEN];

        bits.set(pokemon::VALID, self.valid);
        bits[pokemon::LEVEL].store_le(self.level);
        bits[pokemon::ID].store_le(self.id);
        bits[pokemon::MET_AT].store_le(self.met_at);
        bits[pokemon::MET_FLOOR].store_le(self.met_floor);
        bits.set(34, self.unknown);
        bits[pokemon::EVOLVED_AT_1].store_le(self.evolved_at_1);
        bits[pokemon::EVOLVED_AT_2].store_le(self.evolved_at_2);
        bits[pokemon::IQ].store_le(self.iq);
        bits[pokemon::HP].store_le(self.hp);
        bits[pokemon::ATTACK].store_le(self.attack);
        bits[pokemon::SP_ATTACK].store_le(self.sp_attack);
        bits[pokemon::DEFENSE].store_le(self.defense);
        bits[pokemon::SP_DEFENSE].store_le(self.sp_defense);
        bits[pokemon::EXP].store_le(self.exp);
        bits[pokemon::IQ_MAP].copy_from_bitslice(&self.iq_map[0..69]);
        bits[pokemon::TACTIC].store_le(self.tactic);
        bits[pokemon::MOVE_1].copy_from_bitslice(&self.move_1.to_bits()[0..STORED_MOVE_BIT_LEN]);
        bits[pokemon::MOVE_2].copy_from_bitslice(&self.move_2.to_bits()[0..STORED_MOVE_BIT_LEN]);
        bits[pokemon::MOVE_3].copy_from_bitslice(&self.move_3.to_bits()[0..STORED_MOVE_BIT_LEN]);
        bits[pokemon::MOVE_4].copy_from_bitslice(&self.move_4.to_bits()[0..STORED_MOVE_BIT_LEN]);
        bits[pokemon::NAME].copy_from_bitslice(self.name.to_save_bytes().view_bits());

        bits
    }
}
