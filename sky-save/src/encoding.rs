//! This module handles strings represented by the PMD character encoding.
//! Save file strings have one byte per character and are encoded using a custom character encoding, which is a mix of ASCII, Unicode and special sequences.
//! Special sequences are wrapped in square brackets.
//!
//! Example: The byte representation for the sequence `Abcd[END]` will be `[0x41, 0x62, 0x63, 0x64, 0x00]`.
//! The character `[` is not a valid PMD character, making parsing special sequences easier.
//!
//! See <https://projectpokemon.org/home/docs/mystery-dungeon-nds/explorers-of-sky-save-structure-r62> for more information.

use crate::EncodingError;
use arrayvec::ArrayVec;
use bitvec::order::Lsb0;
use bitvec::prelude::BitSlice;
use std::fmt::Display;

/// A single PMD-encoded character.
/// Holds both the PMD encoded byte and its UTF-8 representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct PmdChar {
    /// The PMD encoded byte.
    pub pmd: u8,
    /// The UTF-8 representation of the character.
    pub utf8: char,
}

impl PmdChar {
    /// Parses a single character or a special sequence into a `PmdChar`.
    pub fn from_sequence(seq: &str) -> Result<Self, EncodingError> {
        let pmd = pmd_seq_to_byte(seq)?;

        let utf8 = match seq.chars().next() {
            Some('[') => pmd as char,
            Some(c) => c,
            _ => unreachable!(), // Safe, seq is valid and not empty at this point.
        };

        Ok(PmdChar { pmd, utf8 })
    }

    /// Converts a PMD character to its sequence representation.
    pub fn to_sequence(&self) -> String {
        byte_to_pmd_seq(self.pmd).unwrap().to_string()
    }
}

/// Converts a PMD-encoded byte to a PMD character.
impl From<u8> for PmdChar {
    fn from(value: u8) -> Self {
        let pmd = byte_to_pmd_seq(value).unwrap();
        PmdChar::from_sequence(pmd).unwrap()
    }
}

/// A string represented by the PMD character encoding, backed by an `ArrayVec`.
/// Save file strings (team names, Pokémon names) have 10 byte memory location.
/// The game stops displaying the strings when it reaches a null byte.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PmdString(ArrayVec<PmdChar, 10>);

impl PmdString {
    pub fn new() -> Self {
        Self(ArrayVec::new())
    }

    /// Converts the string to a sequence of PMD characters.
    pub fn to_sequence(&self) -> String {
        self.0.iter().map(|&c| c.to_sequence()).collect()
    }

    /// Converts to a 10-byte array of PMD encoded bytes.
    pub fn to_save_bytes(&self) -> [u8; 10] {
        self.0
            .iter()
            .enumerate()
            .fold([0; 10], |mut result, (i, c)| {
                result[i] = c.pmd;
                result
            })
    }

    pub fn to_string_until_nul(&self) -> String {
        self.0
            .iter()
            .map_while(|&c| (c.pmd != 0).then_some(c.utf8))
            .collect()
    }
}

/// Converts a PMD string to a UTF-8 string.
/// Does not ignore null bytes.
impl Display for PmdString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|&c| c.utf8).collect::<String>())
    }
}

impl Default for PmdString {
    fn default() -> Self {
        PmdString::new()
    }
}

/// Converts a PMD-encoded byte slice to a `PmdString`.
impl From<&[u8]> for PmdString {
    fn from(value: &[u8]) -> Self {
        let mut result = PmdString::new();
        for &b in value {
            result.0.push(PmdChar::from(b));
        }

        result
    }
}

impl From<&BitSlice<u8, Lsb0>> for PmdString {
    fn from(value: &BitSlice<u8, Lsb0>) -> Self {
        let mut bv = value.to_bitvec();
        bv.force_align();

        PmdString::from(&bv.into_vec().as_slice()[0..10])
    }
}

/// Converts a PMD-encoded string to a byte vector.
/// Does not ignore null bytes.
/// Does not fill the vector to 10 bytes.
impl From<PmdString> for Vec<u8> {
    fn from(value: PmdString) -> Self {
        value.0.iter().map(|c| c.pmd).collect::<Vec<_>>()
    }
}

/// Parses a sequence of PMD characters to a `PmdString`.
impl TryFrom<&str> for PmdString {
    type Error = EncodingError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut result = PmdString::new();
        let mut chars_iter = value.chars().peekable();

        while let Some(c) = chars_iter.next() {
            match c {
                '[' => {
                    let seq: String = chars_iter.by_ref().take_while(|&c| c != ']').collect();
                    let pmd = pmd_seq_to_byte(&format!("[{}]", seq))?;
                    result
                        .0
                        .try_push(PmdChar {
                            utf8: pmd as char,
                            pmd,
                        })
                        .map_err(|_| EncodingError::InvalidPmdStringLen)?;
                }
                _ => {
                    let mut buf = [0; 4];
                    let seq = c.encode_utf8(&mut buf);
                    let pmd = pmd_seq_to_byte(seq)?;
                    result
                        .0
                        .try_push(PmdChar { utf8: c, pmd })
                        .map_err(|_| EncodingError::InvalidPmdStringLen)?;
                }
            }
        }

        Ok(result)
    }
}

fn pmd_seq_to_byte(s: &str) -> Result<u8, EncodingError> {
    match s {
        "[END]" => Ok(0x00),
        "[$01]" => Ok(0x01),
        "[$02]" => Ok(0x02),
        "[$03]" => Ok(0x03),
        "[$04]" => Ok(0x04),
        "[$05]" => Ok(0x05),
        "[$06]" => Ok(0x06),
        "[$07]" => Ok(0x07),
        "[$08]" => Ok(0x08),
        "[$09]" => Ok(0x09),
        "[$0A]" => Ok(0x0A),
        "[$0B]" => Ok(0x0B),
        "[$0C]" => Ok(0x0C),
        "[$0D]" => Ok(0x0D),
        "[$0E]" => Ok(0x0E),
        "[$0F]" => Ok(0x0F),
        "[$10]" => Ok(0x10),
        "[$11]" => Ok(0x11),
        "[$12]" => Ok(0x12),
        "[$13]" => Ok(0x13),
        "[$14]" => Ok(0x14),
        "[$15]" => Ok(0x15),
        "[$16]" => Ok(0x16),
        "[$17]" => Ok(0x17),
        "[$18]" => Ok(0x18),
        "[$19]" => Ok(0x19),
        "[$1A]" => Ok(0x1A),
        "[$1B]" => Ok(0x1B),
        "[$1C]" => Ok(0x1C),
        "[$1D]" => Ok(0x1D),
        "[$1E]" => Ok(0x1E),
        "[$1F]" => Ok(0x1F),
        " " => Ok(0x20),
        "!" => Ok(0x21),
        "\"" => Ok(0x22),
        "#" => Ok(0x23),
        "$" => Ok(0x24),
        "%" => Ok(0x25),
        "&" => Ok(0x26),
        "'" => Ok(0x27),
        "(" => Ok(0x28),
        ")" => Ok(0x29),
        "*" => Ok(0x2A),
        "+" => Ok(0x2B),
        "," => Ok(0x2C),
        "-" => Ok(0x2D),
        "." => Ok(0x2E),
        "/" => Ok(0x2F),
        "0" => Ok(0x30),
        "1" => Ok(0x31),
        "2" => Ok(0x32),
        "3" => Ok(0x33),
        "4" => Ok(0x34),
        "5" => Ok(0x35),
        "6" => Ok(0x36),
        "7" => Ok(0x37),
        "8" => Ok(0x38),
        "9" => Ok(0x39),
        ":" => Ok(0x3A),
        ";" => Ok(0x3B),
        "<" => Ok(0x3C),
        "=" => Ok(0x3D),
        ">" => Ok(0x3E),
        "?" => Ok(0x3F),
        "@" => Ok(0x40),
        "A" => Ok(0x41),
        "B" => Ok(0x42),
        "C" => Ok(0x43),
        "D" => Ok(0x44),
        "E" => Ok(0x45),
        "F" => Ok(0x46),
        "G" => Ok(0x47),
        "H" => Ok(0x48),
        "I" => Ok(0x49),
        "J" => Ok(0x4A),
        "K" => Ok(0x4B),
        "L" => Ok(0x4C),
        "M" => Ok(0x4D),
        "N" => Ok(0x4E),
        "O" => Ok(0x4F),
        "P" => Ok(0x50),
        "Q" => Ok(0x51),
        "R" => Ok(0x52),
        "S" => Ok(0x53),
        "T" => Ok(0x54),
        "U" => Ok(0x55),
        "V" => Ok(0x56),
        "W" => Ok(0x57),
        "X" => Ok(0x58),
        "Y" => Ok(0x59),
        "Z" => Ok(0x5A),
        "[$5B]" => Ok(0x5B),
        "\\" => Ok(0x5C),
        "]" => Ok(0x5D),
        "^" => Ok(0x5E),
        "_" => Ok(0x5F),
        "`" => Ok(0x60),
        "a" => Ok(0x61),
        "b" => Ok(0x62),
        "c" => Ok(0x63),
        "d" => Ok(0x64),
        "e" => Ok(0x65),
        "f" => Ok(0x66),
        "g" => Ok(0x67),
        "h" => Ok(0x68),
        "i" => Ok(0x69),
        "j" => Ok(0x6A),
        "k" => Ok(0x6B),
        "l" => Ok(0x6C),
        "m" => Ok(0x6D),
        "n" => Ok(0x6E),
        "o" => Ok(0x6F),
        "p" => Ok(0x70),
        "q" => Ok(0x71),
        "r" => Ok(0x72),
        "s" => Ok(0x73),
        "t" => Ok(0x74),
        "u" => Ok(0x75),
        "v" => Ok(0x76),
        "w" => Ok(0x77),
        "x" => Ok(0x78),
        "y" => Ok(0x79),
        "z" => Ok(0x7A),
        "{" => Ok(0x7B),
        "|" => Ok(0x7C),
        "}" => Ok(0x7D),
        "[$7E]" => Ok(0x7E),
        "[$7F]" => Ok(0x7F),
        "€" => Ok(0x80),
        "[$81]" => Ok(0x81),
        "[$82]" => Ok(0x82),
        "[$83]" => Ok(0x83),
        "[$84]" => Ok(0x84),
        "…" => Ok(0x85),
        "†" => Ok(0x86),
        "[$87]" => Ok(0x87),
        "ˆ" => Ok(0x88),
        "‰" => Ok(0x89),
        "Š" => Ok(0x8A),
        "‹" => Ok(0x8B),
        "Œ" => Ok(0x8C),
        "[e]" => Ok(0x8D),
        "Ž" => Ok(0x8E),
        "[è]" => Ok(0x8F),
        // "•" => Ok(0x90), // Duplicate
        "‘" => Ok(0x91),
        "’" => Ok(0x92),
        "“" => Ok(0x93),
        "”" => Ok(0x94),
        // "•" => Ok(0x95), // Duplicate
        "[er]" => Ok(0x96),
        "[re]" => Ok(0x97),
        "~" => Ok(0x98),
        "™" => Ok(0x99),
        "š" => Ok(0x9A),
        "›" => Ok(0x9B),
        "œ" => Ok(0x9C),
        "•" => Ok(0x9D),
        "ž" => Ok(0x9E),
        "Ÿ" => Ok(0x9F),
        // " " => Ok(0xA0), // Duplicate
        "¡" => Ok(0xA1),
        "¢" => Ok(0xA2),
        "£" => Ok(0xA3),
        "¤" => Ok(0xA4),
        "¥" => Ok(0xA5),
        "¦" => Ok(0xA6),
        "§" => Ok(0xA7),
        "¨" => Ok(0xA8),
        "©" => Ok(0xA9),
        "ª" => Ok(0xAA),
        "«" => Ok(0xAB),
        "¬" => Ok(0xAC),
        "\u{00AD}" => Ok(0xAD),
        "®" => Ok(0xAE),
        "¯" => Ok(0xAF),
        "°" => Ok(0xB0),
        "±" => Ok(0xB1),
        "²" => Ok(0xB2),
        "³" => Ok(0xB3),
        "´" => Ok(0xB4),
        "µ" => Ok(0xB5),
        "¶" => Ok(0xB6),
        "„" => Ok(0xB7),
        "‚" => Ok(0xB8),
        "¹" => Ok(0xB9),
        "º" => Ok(0xBA),
        "»" => Ok(0xBB),
        "←" => Ok(0xBC),
        "♂" => Ok(0xBD),
        "♀" => Ok(0xBE),
        "¿" => Ok(0xBF),
        "À" => Ok(0xC0),
        "Á" => Ok(0xC1),
        "Â" => Ok(0xC2),
        "Ã" => Ok(0xC3),
        "Ä" => Ok(0xC4),
        "Å" => Ok(0xC5),
        "Æ" => Ok(0xC6),
        "Ç" => Ok(0xC7),
        "È" => Ok(0xC8),
        "É" => Ok(0xC9),
        "Ê" => Ok(0xCA),
        "Ë" => Ok(0xCB),
        "Ì" => Ok(0xCC),
        "Í" => Ok(0xCD),
        "Î" => Ok(0xCE),
        "Ï" => Ok(0xCF),
        "Ð" => Ok(0xD0),
        "Ñ" => Ok(0xD1),
        "Ò" => Ok(0xD2),
        "Ó" => Ok(0xD3),
        "Ô" => Ok(0xD4),
        "Õ" => Ok(0xD5),
        "Ö" => Ok(0xD6),
        "×" => Ok(0xD7),
        "Ø" => Ok(0xD8),
        "Ù" => Ok(0xD9),
        "Ú" => Ok(0xDA),
        "Û" => Ok(0xDB),
        "Ü" => Ok(0xDC),
        "Ý" => Ok(0xDD),
        "Þ" => Ok(0xDE),
        "ß" => Ok(0xDF),
        "à" => Ok(0xE0),
        "á" => Ok(0xE1),
        "â" => Ok(0xE2),
        "ã" => Ok(0xE3),
        "ä" => Ok(0xE4),
        "å" => Ok(0xE5),
        "æ" => Ok(0xE6),
        "ç" => Ok(0xE7),
        "è" => Ok(0xE8),
        "é" => Ok(0xE9),
        "ê" => Ok(0xEA),
        "ë" => Ok(0xEB),
        "ì" => Ok(0xEC),
        "í" => Ok(0xED),
        "î" => Ok(0xEE),
        "ï" => Ok(0xEF),
        "ð" => Ok(0xF0),
        "ñ" => Ok(0xF1),
        "ò" => Ok(0xF2),
        "ó" => Ok(0xF3),
        "ô" => Ok(0xF4),
        "õ" => Ok(0xF5),
        "ö" => Ok(0xF6),
        "÷" => Ok(0xF7),
        "ø" => Ok(0xF8),
        "ù" => Ok(0xF9),
        "ú" => Ok(0xFA),
        "û" => Ok(0xFB),
        "ü" => Ok(0xFC),
        "ý" => Ok(0xFD),
        "þ" => Ok(0xFE),
        "ÿ" => Ok(0xFF),
        _ => Err(EncodingError::InvalidPmdCharacter(s.to_string())),
    }
}

fn byte_to_pmd_seq(byte: u8) -> Result<&'static str, EncodingError> {
    match byte {
        0x00 => Ok("[END]"),
        0x01 => Ok("[$01]"),
        0x02 => Ok("[$02]"),
        0x03 => Ok("[$03]"),
        0x04 => Ok("[$04]"),
        0x05 => Ok("[$05]"),
        0x06 => Ok("[$06]"),
        0x07 => Ok("[$07]"),
        0x08 => Ok("[$08]"),
        0x09 => Ok("[$09]"),
        0x0A => Ok("[$0A]"),
        0x0B => Ok("[$0B]"),
        0x0C => Ok("[$0C]"),
        0x0D => Ok("[$0D]"),
        0x0E => Ok("[$0E]"),
        0x0F => Ok("[$0F]"),
        0x10 => Ok("[$10]"),
        0x11 => Ok("[$11]"),
        0x12 => Ok("[$12]"),
        0x13 => Ok("[$13]"),
        0x14 => Ok("[$14]"),
        0x15 => Ok("[$15]"),
        0x16 => Ok("[$16]"),
        0x17 => Ok("[$17]"),
        0x18 => Ok("[$18]"),
        0x19 => Ok("[$19]"),
        0x1A => Ok("[$1A]"),
        0x1B => Ok("[$1B]"),
        0x1C => Ok("[$1C]"),
        0x1D => Ok("[$1D]"),
        0x1E => Ok("[$1E]"),
        0x1F => Ok("[$1F]"),
        0x20 => Ok(" "),
        0x21 => Ok("!"),
        0x22 => Ok("\""),
        0x23 => Ok("#"),
        0x24 => Ok("$"),
        0x25 => Ok("%"),
        0x26 => Ok("&"),
        0x27 => Ok("'"),
        0x28 => Ok("("),
        0x29 => Ok(")"),
        0x2A => Ok("*"),
        0x2B => Ok("+"),
        0x2C => Ok(","),
        0x2D => Ok("-"),
        0x2E => Ok("."),
        0x2F => Ok("/"),
        0x30 => Ok("0"),
        0x31 => Ok("1"),
        0x32 => Ok("2"),
        0x33 => Ok("3"),
        0x34 => Ok("4"),
        0x35 => Ok("5"),
        0x36 => Ok("6"),
        0x37 => Ok("7"),
        0x38 => Ok("8"),
        0x39 => Ok("9"),
        0x3A => Ok(":"),
        0x3B => Ok(";"),
        0x3C => Ok("<"),
        0x3D => Ok("="),
        0x3E => Ok(">"),
        0x3F => Ok("?"),
        0x40 => Ok("@"),
        0x41 => Ok("A"),
        0x42 => Ok("B"),
        0x43 => Ok("C"),
        0x44 => Ok("D"),
        0x45 => Ok("E"),
        0x46 => Ok("F"),
        0x47 => Ok("G"),
        0x48 => Ok("H"),
        0x49 => Ok("I"),
        0x4A => Ok("J"),
        0x4B => Ok("K"),
        0x4C => Ok("L"),
        0x4D => Ok("M"),
        0x4E => Ok("N"),
        0x4F => Ok("O"),
        0x50 => Ok("P"),
        0x51 => Ok("Q"),
        0x52 => Ok("R"),
        0x53 => Ok("S"),
        0x54 => Ok("T"),
        0x55 => Ok("U"),
        0x56 => Ok("V"),
        0x57 => Ok("W"),
        0x58 => Ok("X"),
        0x59 => Ok("Y"),
        0x5A => Ok("Z"),
        0x5B => Ok("[$5B]"),
        0x5C => Ok("\\"),
        0x5D => Ok("]"),
        0x5E => Ok("^"),
        0x5F => Ok("_"),
        0x60 => Ok("`"),
        0x61 => Ok("a"),
        0x62 => Ok("b"),
        0x63 => Ok("c"),
        0x64 => Ok("d"),
        0x65 => Ok("e"),
        0x66 => Ok("f"),
        0x67 => Ok("g"),
        0x68 => Ok("h"),
        0x69 => Ok("i"),
        0x6A => Ok("j"),
        0x6B => Ok("k"),
        0x6C => Ok("l"),
        0x6D => Ok("m"),
        0x6E => Ok("n"),
        0x6F => Ok("o"),
        0x70 => Ok("p"),
        0x71 => Ok("q"),
        0x72 => Ok("r"),
        0x73 => Ok("s"),
        0x74 => Ok("t"),
        0x75 => Ok("u"),
        0x76 => Ok("v"),
        0x77 => Ok("w"),
        0x78 => Ok("x"),
        0x79 => Ok("y"),
        0x7A => Ok("z"),
        0x7B => Ok("{"),
        0x7C => Ok("|"),
        0x7D => Ok("}"),
        0x7E => Ok("[$7E]"),
        0x7F => Ok("[$7F]"),
        0x80 => Ok("€"),
        0x81 => Ok("[$81]"),
        0x82 => Ok("[$82]"),
        0x83 => Ok("[$83]"),
        0x84 => Ok("[$84]"),
        0x85 => Ok("…"),
        0x86 => Ok("†"),
        0x87 => Ok("[$87]"),
        0x88 => Ok("ˆ"),
        0x89 => Ok("‰"),
        0x8A => Ok("Š"),
        0x8B => Ok("‹"),
        0x8C => Ok("Œ"),
        0x8D => Ok("[e]"),
        0x8E => Ok("Ž"),
        0x8F => Ok("[è]"),
        0x90 => Ok("•"),
        0x91 => Ok("‘"),
        0x92 => Ok("’"),
        0x93 => Ok("“"),
        0x94 => Ok("”"),
        0x95 => Ok("•"),
        0x96 => Ok("[er]"),
        0x97 => Ok("[re]"),
        0x98 => Ok("~"),
        0x99 => Ok("™"),
        0x9A => Ok("š"),
        0x9B => Ok("›"),
        0x9C => Ok("œ"),
        0x9D => Ok("•"),
        0x9E => Ok("ž"),
        0x9F => Ok("Ÿ"),
        0xA0 => Ok(" "),
        0xA1 => Ok("¡"),
        0xA2 => Ok("¢"),
        0xA3 => Ok("£"),
        0xA4 => Ok("¤"),
        0xA5 => Ok("¥"),
        0xA6 => Ok("¦"),
        0xA7 => Ok("§"),
        0xA8 => Ok("¨"),
        0xA9 => Ok("©"),
        0xAA => Ok("ª"),
        0xAB => Ok("«"),
        0xAC => Ok("¬"),
        0xAD => Ok("\u{00AD}"),
        0xAE => Ok("®"),
        0xAF => Ok("¯"),
        0xB0 => Ok("°"),
        0xB1 => Ok("±"),
        0xB2 => Ok("²"),
        0xB3 => Ok("³"),
        0xB4 => Ok("´"),
        0xB5 => Ok("µ"),
        0xB6 => Ok("¶"),
        0xB7 => Ok("„"),
        0xB8 => Ok("‚"),
        0xB9 => Ok("¹"),
        0xBA => Ok("º"),
        0xBB => Ok("»"),
        0xBC => Ok("←"),
        0xBD => Ok("♂"),
        0xBE => Ok("♀"),
        0xBF => Ok("¿"),
        0xC0 => Ok("À"),
        0xC1 => Ok("Á"),
        0xC2 => Ok("Â"),
        0xC3 => Ok("Ã"),
        0xC4 => Ok("Ä"),
        0xC5 => Ok("Å"),
        0xC6 => Ok("Æ"),
        0xC7 => Ok("Ç"),
        0xC8 => Ok("È"),
        0xC9 => Ok("É"),
        0xCA => Ok("Ê"),
        0xCB => Ok("Ë"),
        0xCC => Ok("Ì"),
        0xCD => Ok("Í"),
        0xCE => Ok("Î"),
        0xCF => Ok("Ï"),
        0xD0 => Ok("Ð"),
        0xD1 => Ok("Ñ"),
        0xD2 => Ok("Ò"),
        0xD3 => Ok("Ó"),
        0xD4 => Ok("Ô"),
        0xD5 => Ok("Õ"),
        0xD6 => Ok("Ö"),
        0xD7 => Ok("×"),
        0xD8 => Ok("Ø"),
        0xD9 => Ok("Ù"),
        0xDA => Ok("Ú"),
        0xDB => Ok("Û"),
        0xDC => Ok("Ü"),
        0xDD => Ok("Ý"),
        0xDE => Ok("Þ"),
        0xDF => Ok("ß"),
        0xE0 => Ok("à"),
        0xE1 => Ok("á"),
        0xE2 => Ok("â"),
        0xE3 => Ok("ã"),
        0xE4 => Ok("ä"),
        0xE5 => Ok("å"),
        0xE6 => Ok("æ"),
        0xE7 => Ok("ç"),
        0xE8 => Ok("è"),
        0xE9 => Ok("é"),
        0xEA => Ok("ê"),
        0xEB => Ok("ë"),
        0xEC => Ok("ì"),
        0xED => Ok("í"),
        0xEE => Ok("î"),
        0xEF => Ok("ï"),
        0xF0 => Ok("ð"),
        0xF1 => Ok("ñ"),
        0xF2 => Ok("ò"),
        0xF3 => Ok("ó"),
        0xF4 => Ok("ô"),
        0xF5 => Ok("õ"),
        0xF6 => Ok("ö"),
        0xF7 => Ok("÷"),
        0xF8 => Ok("ø"),
        0xF9 => Ok("ù"),
        0xFA => Ok("ú"),
        0xFB => Ok("û"),
        0xFC => Ok("ü"),
        0xFD => Ok("ý"),
        0xFE => Ok("þ"),
        0xFF => Ok("ÿ"),
    }
}

#[test]
fn test_char_round_trip() {
    let ch = PmdChar::from_sequence("A").unwrap();
    assert_eq!(ch.utf8, 'A');
    assert_eq!(ch.pmd, 0x41);
    let pmd = PmdChar::from(0x41);
    assert_eq!(ch, pmd);
}

#[test]
fn test_char_special_trip() {
    let ch = PmdChar::from_sequence("[er]").unwrap();
    assert_eq!(ch.utf8, '\u{96}');
    assert_eq!(ch.pmd, 0x96);
    let pmd = PmdChar::from(0x96);
    assert_eq!(ch, pmd);
}

#[test]
#[should_panic]

fn test_char_invalid_sequence() {
    PmdChar::from_sequence("[LOL]").unwrap();
}

#[test]
fn test_char_to_sequence() {
    let ch = PmdChar::from(0x8D);
    assert_eq!(ch.to_sequence(), "[e]");
}

#[test]
fn test_pmd_string_parse() {
    let seq = "Oak[END]";
    PmdString::try_from(seq).unwrap();
}

#[test]
#[should_panic]
fn test_pmd_string_invalid_len() {
    PmdString::try_from("OakOakOakOak").unwrap();
}

#[test]
fn test_pmd_string_to_sequence() {
    let pmd = PmdString::from([0x00, 0x00, 0x00, 0xC4, 0x88, 0x7E].as_slice());
    assert_eq!(pmd.to_sequence(), "[END][END][END]Äˆ[$7E]");
}

#[test]
fn test_pmd_string_to_string() {
    let pmd = PmdString::from([0x00, 0x00, 0x00, 0xC4, 0x88, 0x7E].as_slice());
    assert_eq!(pmd.to_string(), "\0\0\0Äˆ~");
}

#[test]
fn test_pmd_string_to_save_bytes() {
    let pmd = PmdString::from([0x00, 0x00, 0x00, 0xC4, 0x88, 0x7E].as_slice());
    assert_eq!(
        pmd.to_save_bytes(),
        [0x00, 0x00, 0x00, 0xC4, 0x88, 0x7E, 0x00, 0x00, 0x00, 0x00]
    );
}

#[test]
fn test_pmd_string_to_vec() {
    let pmd = PmdString::from([0xC4, 0x88, 0x7E].as_slice());
    let vec = Vec::from(pmd);

    assert_eq!(vec.as_slice(), &[0xC4, 0x88, 0x7E]);
}
