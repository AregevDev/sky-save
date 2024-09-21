use crate::offsets::active::{moves, pokemon};
use crate::{IqMapBits, PmdString};
use bitvec::prelude::*;

pub type ActivePokemonBits = BitArr!(for 546, in u8, Lsb0);
pub type ActiveMoveBits = BitArr!(for 29, in u8, Lsb0);

#[derive(Debug)]
pub struct ActiveMove(pub ActiveMoveBits);

impl ActiveMove {
    pub fn valid(&self) -> bool {
        self.0[moves::VALID]
    }

    pub fn linked(&self) -> bool {
        self.0[moves::LINKED]
    }

    pub fn switched(&self) -> bool {
        self.0[moves::SWITCHED]
    }

    pub fn set(&self) -> bool {
        self.0[moves::SET]
    }

    pub fn sealed(&self) -> bool {
        self.0[moves::SEALED]
    }

    pub fn id(&self) -> u16 {
        self.0[moves::ID].load_le()
    }

    pub fn pp(&self) -> u8 {
        self.0[moves::PP].load_le()
    }

    pub fn power_boost(&self) -> u8 {
        self.0[moves::POWER_BOOST].load_le()
    }
}

#[derive(Debug)]
pub struct ActivePokemon(pub ActivePokemonBits);

impl ActivePokemon {
    pub fn valid(&self) -> bool {
        self.0[pokemon::VALID]
    }

    pub fn level(&self) -> u8 {
        self.0[pokemon::LEVEL].load_le()
    }

    pub fn met_at(&self) -> u8 {
        self.0[pokemon::MET_AT].load_le()
    }

    pub fn met_floor(&self) -> u8 {
        self.0[pokemon::MET_FLOOR].load_le()
    }

    pub fn iq(&self) -> u16 {
        self.0[pokemon::IQ].load_le()
    }

    pub fn roaster_number(&self) -> u16 {
        self.0[pokemon::ROASTER_NUMBER].load_le()
    }

    pub fn id(&self) -> u16 {
        self.0[pokemon::ID].load_le()
    }

    pub fn current_hp(&self) -> u16 {
        self.0[pokemon::CURRENT_HP].load_le()
    }

    pub fn max_hp(&self) -> u16 {
        self.0[pokemon::MAX_HP].load_le()
    }

    pub fn attack(&self) -> u8 {
        self.0[pokemon::ATTACK].load_le()
    }

    pub fn sp_attack(&self) -> u8 {
        self.0[pokemon::SP_ATTACK].load_le()
    }

    pub fn defense(&self) -> u8 {
        self.0[pokemon::DEFENSE].load_le()
    }

    pub fn sp_defense(&self) -> u8 {
        self.0[pokemon::SP_DEFENSE].load_le()
    }

    pub fn exp(&self) -> u32 {
        self.0[pokemon::EXP].load_le()
    }

    pub fn moves(&self) -> [ActiveMove; 4] {
        let mut moves: [ActiveMoveBits; 4] = [bitarr!(u8, Lsb0; 0; 29); 4];
        let view = [
            &self.0[pokemon::MOVE_1],
            &self.0[pokemon::MOVE_2],
            &self.0[pokemon::MOVE_3],
            &self.0[pokemon::MOVE_4],
        ];

        moves
            .iter_mut()
            .zip(view)
            .map(|(bits, view)| {
                (*bits)[0..29].copy_from_bitslice(view);
                ActiveMove(*bits)
            })
            .collect::<Vec<ActiveMove>>()
            .try_into()
            .unwrap()
    }

    pub fn iq_map(&self) -> IqMapBits {
        let mut map: IqMapBits = bitarr!(u8, Lsb0; 0; 69);
        let view = &self.0[pokemon::IQ_MAP];
        map[0..69].copy_from_bitslice(view);

        map
    }

    pub fn tactic(&self) -> u8 {
        self.0[pokemon::TACTIC].load_le()
    }

    pub fn name(&self) -> PmdString {
        let bits = &self.0[pokemon::NAME];
        let mut bytes = bits.to_owned();
        bytes.force_align();

        PmdString::from(bytes.into_vec().as_slice())
    }
}
