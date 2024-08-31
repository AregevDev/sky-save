use crate::offsets::active::{moves, pokemon};
use crate::{pmd_to_string, EncodingError, IqMapBits};
use bitvec::prelude::*;

#[derive(Debug)]
pub struct ActiveMove<'a> {
    pub data: &'a BitSlice<u8, Lsb0>,
}

impl<'a> ActiveMove<'a> {
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

    pub fn sealed(&self) -> bool {
        self.data[moves::SEALED]
    }

    pub fn id(&self) -> u16 {
        self.data[moves::ID].load_le()
    }

    pub fn pp(&self) -> u8 {
        self.data[moves::PP].load_le()
    }

    pub fn power_boost(&self) -> u8 {
        self.data[moves::POWER_BOOST].load_le()
    }
}

#[derive(Debug)]
pub struct ActivePokemon<'a> {
    pub data: &'a BitSlice<u8, Lsb0>,
}

impl<'a> ActivePokemon<'a> {
    pub fn valid(&self) -> bool {
        self.data[pokemon::VALID]
    }

    pub fn level(&self) -> u8 {
        self.data[pokemon::LEVEL].load_le()
    }

    pub fn met_at(&self) -> u32 {
        self.data[pokemon::MET_AT].load_le()
    }

    pub fn met_floor(&self) -> u16 {
        self.data[pokemon::MET_FLOOR].load_le()
    }

    pub fn iq(&self) -> u16 {
        self.data[pokemon::IQ].load_le()
    }

    pub fn roaster_number(&self) -> u16 {
        self.data[pokemon::ROASTER_NUMBER].load_le()
    }

    pub fn id(&self) -> u16 {
        self.data[pokemon::ID].load_le()
    }

    pub fn current_hp(&self) -> u16 {
        self.data[pokemon::CURRENT_HP].load_le()
    }

    pub fn max_hp(&self) -> u16 {
        self.data[pokemon::MAX_HP].load_le()
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

    pub fn exp(&self) -> u64 {
        self.data[pokemon::EXP].load_le()
    }

    pub fn moves(&self) -> [ActiveMove; 4] {
        [
            ActiveMove {
                data: &self.data[pokemon::MOVE_1],
            },
            ActiveMove {
                data: &self.data[pokemon::MOVE_2],
            },
            ActiveMove {
                data: &self.data[pokemon::MOVE_3],
            },
            ActiveMove {
                data: &self.data[pokemon::MOVE_4],
            },
        ]
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

    pub fn name(&self) -> Result<String, EncodingError> {
        let bits = &self.data[pokemon::NAME];
        let mut bytes = bits.to_owned();
        bytes.force_align();

        pmd_to_string(bytes.into_vec().as_slice())
    }
}
