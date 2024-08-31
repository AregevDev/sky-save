use crate::offsets::stored::{moves, pokemon};
use crate::{pmd_to_string, EncodingError};
use bitvec::field::BitField;
use bitvec::prelude::*;
use bitvec::{bitarr, BitArr};

pub type IqMapBits = BitArr!(for 69, in u8, Lsb0);

#[derive(Debug)]
pub struct StoredMove<'a> {
    pub data: &'a BitSlice<u8, Lsb0>,
}

impl<'a> StoredMove<'a> {
    pub fn valid(&self) -> bool {
        self.data[moves::VALID]
    }

    pub fn linked(&self) -> bool {
        self.data[moves::LINKED]
    }

    pub fn switched(&self) -> bool {
        self.data[moves::SWITCHED]
    }

    pub fn set(&self) -> bool {
        self.data[moves::SET]
    }

    pub fn id(&self) -> u16 {
        self.data[moves::ID].load_le()
    }

    pub fn power_boost(&self) -> u8 {
        self.data[moves::POWER_BOOST].load_le()
    }
}

#[derive(Debug)]
pub struct StoredPokemon<'a> {
    pub data: &'a BitSlice<u8, Lsb0>,
}

impl<'a> StoredPokemon<'a> {
    pub fn valid(&self) -> bool {
        self.data[pokemon::VALID]
    }

    pub fn level(&self) -> u8 {
        self.data[pokemon::LEVEL].load_le()
    }

    pub fn id(&self) -> u16 {
        self.data[pokemon::ID].load_le()
    }

    pub fn met_at(&self) -> u8 {
        self.data[pokemon::MET_AT].load_le()
    }

    pub fn met_floor(&self) -> u8 {
        self.data[pokemon::MET_FLOOR].load_le()
    }

    pub fn evolved_at(&self) -> (u8, u8) {
        (
            self.data[pokemon::EVOLVED_AT_1].load_le(),
            self.data[pokemon::EVOLVED_AT_2].load_le(),
        )
    }

    pub fn iq(&self) -> u16 {
        self.data[pokemon::IQ].load_le()
    }

    pub fn hp(&self) -> u16 {
        self.data[pokemon::HP].load_le()
    }

    pub fn attack(&self) -> u8 {
        self.data[pokemon::ATTACK].load_le()
    }

    pub fn sp_attack(&self) -> u8 {
        self.data[pokemon::SP_ATTACK].load_le()
    }

    pub fn defense(&self) -> u8 {
        self.data[pokemon::DEFENSE].load_le()
    }

    pub fn sp_defense(&self) -> u8 {
        self.data[pokemon::SP_DEFENSE].load_le()
    }

    pub fn exp(&self) -> u32 {
        self.data[pokemon::EXP].load_le()
    }

    pub fn iq_map(&self) -> IqMapBits {
        let mut map: IqMapBits = bitarr!(u8, Lsb0; 0; 69);
        let view = &self.data[pokemon::IQ_MAP];
        map.copy_from_bitslice(view);

        map
    }

    pub fn tactic(&self) -> u8 {
        self.data[pokemon::TACTIC].load_le()
    }

    pub fn moves(&self) -> [StoredMove<'a>; 4] {
        [
            StoredMove {
                data: &self.data[pokemon::MOVE_1],
            },
            StoredMove {
                data: &self.data[pokemon::MOVE_2],
            },
            StoredMove {
                data: &self.data[pokemon::MOVE_3],
            },
            StoredMove {
                data: &self.data[pokemon::MOVE_4],
            },
        ]
    }

    pub fn name(&self) -> Result<String, EncodingError> {
        let bits = &self.data[pokemon::NAME];
        let mut bytes = bits.to_owned();
        bytes.force_align();

        pmd_to_string(bytes.into_vec().as_slice())
    }
}
