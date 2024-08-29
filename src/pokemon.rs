use crate::{pmd_to_string, EncodingError};
use bitvec::field::BitField;
use bitvec::prelude::Lsb0;
use bitvec::slice::BitSlice;
use bitvec::{bitarr, BitArr};
use std::ops::Range;

pub type IqMapBits = BitArr!(for 69, in u8, Lsb0);

pub const VALID: usize = 0;
pub const LEVEL: Range<usize> = 1..8;
pub const ID: Range<usize> = 8..19;
pub const MET_AT: Range<usize> = 19..27;
pub const MET_FLOOR: Range<usize> = 27..34;
pub const EVOLVED_AT_1: Range<usize> = 35..42;
pub const EVOLVED_AT_2: Range<usize> = 42..49;
pub const IQ: Range<usize> = 49..59;
pub const HP: Range<usize> = 59..69;
pub const ATTACK: Range<usize> = 69..77;
pub const SP_ATTACK: Range<usize> = 77..85;
pub const DEFENSE: Range<usize> = 85..93;
pub const SP_DEFENSE: Range<usize> = 93..101;
pub const EXP: Range<usize> = 101..125;
pub const IQ_MAP: Range<usize> = 125..194;
pub const TACTIC: Range<usize> = 194..198;
pub const MOVE_1: Range<usize> = 198..219;
pub const MOVE_2: Range<usize> = 219..240;
pub const MOVE_3: Range<usize> = 240..261;
pub const MOVE_4: Range<usize> = 261..282;
pub const NAME: Range<usize> = 282..362;

#[derive(Debug)]
pub struct StoredPokemon<'a> {
    pub data: &'a BitSlice<u8, Lsb0>,
}

impl<'a> StoredPokemon<'a> {
    pub fn valid(&self) -> bool {
        self.data[VALID]
    }

    pub fn level(&self) -> u8 {
        self.data[LEVEL].load_le::<u8>()
    }

    pub fn id(&self) -> u16 {
        self.data[ID].load_le::<u16>()
    }

    pub fn met_at(&self) -> u8 {
        self.data[MET_AT].load_le::<u8>()
    }

    pub fn met_floor(&self) -> u8 {
        self.data[MET_FLOOR].load_le::<u8>()
    }

    pub fn evolved_at(&self) -> (u8, u8) {
        (
            self.data[EVOLVED_AT_1].load_le::<u8>(),
            self.data[EVOLVED_AT_2].load_le::<u8>(),
        )
    }

    pub fn iq(&self) -> u16 {
        self.data[IQ].load_le::<u16>()
    }

    pub fn hp(&self) -> u16 {
        self.data[HP].load_le::<u16>()
    }

    pub fn attack(&self) -> u8 {
        self.data[ATTACK].load_le::<u8>()
    }

    pub fn sp_attack(&self) -> u8 {
        self.data[SP_ATTACK].load_le::<u8>()
    }

    pub fn defense(&self) -> u8 {
        self.data[DEFENSE].load_le::<u8>()
    }

    pub fn sp_defense(&self) -> u8 {
        self.data[SP_DEFENSE].load_le::<u8>()
    }

    pub fn exp(&self) -> u32 {
        self.data[EXP].load_le::<u32>()
    }

    pub fn iq_map(&self) -> IqMapBits {
        let mut map: IqMapBits = bitarr!(u8, Lsb0; 0; 69);
        let view = &self.data[IQ_MAP];
        map.copy_from_bitslice(view);

        map
    }

    pub fn tactic(&self) -> u8 {
        self.data[TACTIC].load_le::<u8>()
    }

    pub fn name(&self) -> Result<String, EncodingError> {
        let bits = &self.data[NAME];
        let mut bytes = bits.to_owned();
        bytes.force_align();

        pmd_to_string(bytes.into_vec().as_slice())
    }
}
