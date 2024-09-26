use crate::offsets::active::{moves, pokemon, ACTIVE_PKM_BIT_LEN};
use crate::{IqMapBits, PmdString};
use bitvec::prelude::*;

#[derive(Debug)]
pub struct ActiveMove {
    pub valid: bool,
    pub linked: bool,
    pub switched: bool,
    pub set: bool,
    pub sealed: bool,
    pub id: u16,
    pub pp: u8,
    pub power_boost: u8,
}

impl ActiveMove {
    pub fn from_bitslice(value: &BitSlice<u8, Lsb0>) -> Self {
        Self {
            valid: value[moves::VALID],
            linked: value[moves::LINKED],
            switched: value[moves::SWITCHED],
            set: value[moves::SET],
            sealed: value[moves::SEALED],
            id: value[moves::ID].load_le(),
            pp: value[moves::PP].load_le(),
            power_boost: value[moves::POWER_BOOST].load_le(),
        }
    }

    pub fn to_bitvec(&self) -> BitVec<u8, Lsb0> {
        let mut bits = bitvec![u8, Lsb0; 0; 29];
        bits.set(moves::VALID, self.valid);
        bits.set(moves::LINKED, self.linked);
        bits.set(moves::SWITCHED, self.switched);
        bits.set(moves::SET, self.set);
        bits[moves::ID].store_le(self.id);
        bits[moves::PP].store_le(self.pp);
        bits[moves::POWER_BOOST].store_le(self.power_boost);
        bits
    }
}

#[derive(Debug)]
pub struct ActivePokemon {
    pub valid: bool,
    pub unknown_1: u8,
    pub level: u8,
    pub met_at: u8,
    pub met_floor: u8,
    pub unknown_2: bool,
    pub iq: u16,
    pub roaster_number: u16,
    pub unknown_3: u32,
    pub id: u16,
    pub current_hp: u16,
    pub max_hp: u16,
    pub attack: u8,
    pub sp_attack: u8,
    pub defense: u8,
    pub sp_defense: u8,
    pub exp: u32,
    pub move_1: ActiveMove,
    pub move_2: ActiveMove,
    pub move_3: ActiveMove,
    pub move_4: ActiveMove,
    pub unknown_4: u128,
    pub iq_map: IqMapBits,
    pub tactic: u8,
    pub unknown_5: u16,
    pub name: PmdString,
}

impl ActivePokemon {
    pub fn from_bitslice(value: &BitSlice<u8, Lsb0>) -> Self {
        let mut iq: IqMapBits = bitarr!(u8, Lsb0; 0; 69);
        iq[0..69].copy_from_bitslice(&value[pokemon::IQ_MAP]);

        let mut name_bytes = value[pokemon::NAME].to_bitvec();
        name_bytes.force_align();

        Self {
            valid: value[pokemon::VALID],
            unknown_1: value[pokemon::UNKNOWN_1].load_le(),
            level: value[pokemon::LEVEL].load_le(),
            met_at: value[pokemon::MET_AT].load_le(),
            met_floor: value[pokemon::MET_FLOOR].load_le(),
            unknown_2: value[pokemon::UNKNOWN_2],
            iq: value[pokemon::IQ].load_le(),
            roaster_number: value[pokemon::ROASTER_NUMBER].load_le(),
            unknown_3: value[pokemon::UNKNOWN_3].load_le(),
            id: value[pokemon::ID].load_le(),
            current_hp: value[pokemon::CURRENT_HP].load_le(),
            max_hp: value[pokemon::MAX_HP].load_le(),
            attack: value[pokemon::ATTACK].load_le(),
            sp_attack: value[pokemon::SP_ATTACK].load_le(),
            defense: value[pokemon::DEFENSE].load_le(),
            sp_defense: value[pokemon::SP_DEFENSE].load_le(),
            exp: value[pokemon::EXP].load_le(),
            move_1: ActiveMove::from_bitslice(&value[pokemon::MOVE_1]),
            move_2: ActiveMove::from_bitslice(&value[pokemon::MOVE_2]),
            move_3: ActiveMove::from_bitslice(&value[pokemon::MOVE_3]),
            move_4: ActiveMove::from_bitslice(&value[pokemon::MOVE_4]),
            unknown_4: value[pokemon::UNKNOWN_4].load_le(),
            iq_map: iq,
            tactic: value[pokemon::TACTIC].load_le(),
            unknown_5: value[pokemon::UNKNOWN_5].load_le(),
            name: PmdString::from(name_bytes.into_vec().as_slice()),
        }
    }

    pub fn to_bitvec(&self) -> BitVec<u8, Lsb0> {
        let mut bits = BitVec::new();
        bits.resize(ACTIVE_PKM_BIT_LEN, false);

        bits.set(pokemon::VALID, self.valid);
        bits[pokemon::UNKNOWN_1].store_le(self.unknown_1);
        bits[pokemon::LEVEL].store_le(self.level);
        bits[pokemon::MET_AT].store_le(self.met_at);
        bits[pokemon::MET_FLOOR].store_le(self.met_floor);
        bits.set(pokemon::UNKNOWN_2, self.unknown_2);
        bits[pokemon::IQ].store_le(self.iq);
        bits[pokemon::ROASTER_NUMBER].store_le(self.roaster_number);
        bits[pokemon::UNKNOWN_3].store_le(self.unknown_3);
        bits[pokemon::ID].store_le(self.id);
        bits[pokemon::CURRENT_HP].store_le(self.current_hp);
        bits[pokemon::MAX_HP].store_le(self.max_hp);
        bits[pokemon::ATTACK].store_le(self.attack);
        bits[pokemon::SP_ATTACK].store_le(self.sp_attack);
        bits[pokemon::DEFENSE].store_le(self.defense);
        bits[pokemon::SP_DEFENSE].store_le(self.sp_defense);
        bits[pokemon::EXP].store_le(self.exp);
        bits[pokemon::MOVE_1].copy_from_bitslice(self.move_1.to_bitvec().as_bitslice());
        bits[pokemon::MOVE_2].copy_from_bitslice(self.move_2.to_bitvec().as_bitslice());
        bits[pokemon::MOVE_3].copy_from_bitslice(self.move_3.to_bitvec().as_bitslice());
        bits[pokemon::MOVE_4].copy_from_bitslice(self.move_4.to_bitvec().as_bitslice());
        bits[pokemon::UNKNOWN_4].store_le(self.unknown_4);
        bits[pokemon::IQ_MAP].copy_from_bitslice(&self.iq_map[0..69]);
        bits[pokemon::TACTIC].store_le(self.tactic);
        bits[pokemon::UNKNOWN_5].store_le(self.unknown_5);
        bits[pokemon::NAME].copy_from_bitslice(self.name.to_save_bytes().view_bits::<Lsb0>());

        bits
    }
}
