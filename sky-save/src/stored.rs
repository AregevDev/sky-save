use crate::offsets::stored::{moves, pokemon};
use crate::PmdString;
use bitvec::field::BitField;
use bitvec::prelude::*;
use bitvec::{bitarr, BitArr};

pub type StoredPokemonBits = BitArr!(for 362, in u8, Lsb0);
pub type StoredMoveBits = BitArr!(for 21, in u8, Lsb0);
pub type IqMapBits = BitArr!(for 69, in u8, Lsb0);

#[derive(Debug)]
pub struct StoredMove(pub StoredMoveBits);

impl StoredMove {
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

    pub fn id(&self) -> u16 {
        self.0[moves::ID].load_le()
    }

    pub fn power_boost(&self) -> u8 {
        self.0[moves::POWER_BOOST].load_le()
    }
}

#[derive(Debug)]
pub struct StoredPokemon(pub StoredPokemonBits);

impl StoredPokemon {
    pub fn valid(&self) -> bool {
        self.0[pokemon::VALID]
    }

    pub fn level(&self) -> u8 {
        self.0[pokemon::LEVEL].load_le()
    }

    pub fn id(&self) -> u16 {
        self.0[pokemon::ID].load_le()
    }

    pub fn met_at(&self) -> u8 {
        self.0[pokemon::MET_AT].load_le()
    }

    pub fn met_floor(&self) -> u8 {
        self.0[pokemon::MET_FLOOR].load_le()
    }

    pub fn evolved_at(&self) -> (u8, u8) {
        (
            self.0[pokemon::EVOLVED_AT_1].load_le(),
            self.0[pokemon::EVOLVED_AT_2].load_le(),
        )
    }

    pub fn iq(&self) -> u16 {
        self.0[pokemon::IQ].load_le()
    }

    pub fn hp(&self) -> u16 {
        self.0[pokemon::HP].load_le()
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

    pub fn iq_map(&self) -> IqMapBits {
        let mut map: IqMapBits = bitarr!(u8, Lsb0; 0; 69);
        let view = &self.0[pokemon::IQ_MAP];
        map[0..69].copy_from_bitslice(view);

        map
    }

    pub fn tactic(&self) -> u8 {
        self.0[pokemon::TACTIC].load_le()
    }

    pub fn moves(&self) -> [StoredMove; 4] {
        let mut moves: [StoredMoveBits; 4] = [bitarr!(u8, Lsb0; 0; 21); 4];
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
                (*bits)[0..21].copy_from_bitslice(view);
                StoredMove(*bits)
            })
            .collect::<Vec<StoredMove>>()
            .try_into()
            .unwrap()
    }

    pub fn name(&self) -> PmdString {
        let bits = &self.0[pokemon::NAME];
        let mut bytes = bits.to_owned();
        bytes.force_align();

        PmdString::from(bytes.into_vec().as_slice())
    }
}
