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
            Some(c) if c == '[' => pmd as char,
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
        PmdChar::from_sequence(pmd.as_str()).unwrap()
    }
}

/// A string represented by the PMD character encoding, backed by an `ArrayVec`.
/// Save file strings (team names, pokemon names) have 10 byte memory location.
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
        self.0.iter().enumerate().fold([0; 10], |mut result, (i, c)| {
            result[i] = c.pmd;
            result
        })
    }
}

/// Converts a PMD string to a UTF-8 string.
/// Does not ignore null bytes.
impl Display for PmdString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0.iter().map(|&c| c.utf8).collect::<String>()
        )
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
                    result.0.try_push(PmdChar { utf8: pmd as char, pmd }).map_err(|_| {
                        EncodingError::InvalidPmdStringLen
                    })?;
                }
                _ => {
                    let mut buf = [0; 4];
                    let seq = c.encode_utf8(&mut buf);
                    let pmd = pmd_seq_to_byte(seq)?;
                    result.0.try_push(PmdChar { utf8: c, pmd }).map_err(|_| {
                        EncodingError::InvalidPmdStringLen
                    })?;
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

fn byte_to_pmd_seq(byte: u8) -> Result<String, EncodingError> {
    match byte {
        0x00 => Ok("[END]".into()),
        0x01 => Ok("[$01]".into()),
        0x02 => Ok("[$02]".into()),
        0x03 => Ok("[$03]".into()),
        0x04 => Ok("[$04]".into()),
        0x05 => Ok("[$05]".into()),
        0x06 => Ok("[$06]".into()),
        0x07 => Ok("[$07]".into()),
        0x08 => Ok("[$08]".into()),
        0x09 => Ok("[$09]".into()),
        0x0A => Ok("[$0A]".into()),
        0x0B => Ok("[$0B]".into()),
        0x0C => Ok("[$0C]".into()),
        0x0D => Ok("[$0D]".into()),
        0x0E => Ok("[$0E]".into()),
        0x0F => Ok("[$0F]".into()),
        0x10 => Ok("[$10]".into()),
        0x11 => Ok("[$11]".into()),
        0x12 => Ok("[$12]".into()),
        0x13 => Ok("[$13]".into()),
        0x14 => Ok("[$14]".into()),
        0x15 => Ok("[$15]".into()),
        0x16 => Ok("[$16]".into()),
        0x17 => Ok("[$17]".into()),
        0x18 => Ok("[$18]".into()),
        0x19 => Ok("[$19]".into()),
        0x1A => Ok("[$1A]".into()),
        0x1B => Ok("[$1B]".into()),
        0x1C => Ok("[$1C]".into()),
        0x1D => Ok("[$1D]".into()),
        0x1E => Ok("[$1E]".into()),
        0x1F => Ok("[$1F]".into()),
        0x20 => Ok(" ".into()),
        0x21 => Ok("!".into()),
        0x22 => Ok("\"".into()),
        0x23 => Ok("#".into()),
        0x24 => Ok("$".into()),
        0x25 => Ok("%".into()),
        0x26 => Ok("&".into()),
        0x27 => Ok("'".into()),
        0x28 => Ok("(".into()),
        0x29 => Ok(")".into()),
        0x2A => Ok("*".into()),
        0x2B => Ok("+".into()),
        0x2C => Ok(",".into()),
        0x2D => Ok("-".into()),
        0x2E => Ok(".".into()),
        0x2F => Ok("/".into()),
        0x30 => Ok("0".into()),
        0x31 => Ok("1".into()),
        0x32 => Ok("2".into()),
        0x33 => Ok("3".into()),
        0x34 => Ok("4".into()),
        0x35 => Ok("5".into()),
        0x36 => Ok("6".into()),
        0x37 => Ok("7".into()),
        0x38 => Ok("8".into()),
        0x39 => Ok("9".into()),
        0x3A => Ok(":".into()),
        0x3B => Ok(";".into()),
        0x3C => Ok("<".into()),
        0x3D => Ok("=".into()),
        0x3E => Ok(">".into()),
        0x3F => Ok("?".into()),
        0x40 => Ok("@".into()),
        0x41 => Ok("A".into()),
        0x42 => Ok("B".into()),
        0x43 => Ok("C".into()),
        0x44 => Ok("D".into()),
        0x45 => Ok("E".into()),
        0x46 => Ok("F".into()),
        0x47 => Ok("G".into()),
        0x48 => Ok("H".into()),
        0x49 => Ok("I".into()),
        0x4A => Ok("J".into()),
        0x4B => Ok("K".into()),
        0x4C => Ok("L".into()),
        0x4D => Ok("M".into()),
        0x4E => Ok("N".into()),
        0x4F => Ok("O".into()),
        0x50 => Ok("P".into()),
        0x51 => Ok("Q".into()),
        0x52 => Ok("R".into()),
        0x53 => Ok("S".into()),
        0x54 => Ok("T".into()),
        0x55 => Ok("U".into()),
        0x56 => Ok("V".into()),
        0x57 => Ok("W".into()),
        0x58 => Ok("X".into()),
        0x59 => Ok("Y".into()),
        0x5A => Ok("Z".into()),
        0x5B => Ok("[$5B]".into()),
        0x5C => Ok("\\".into()),
        0x5D => Ok("]".into()),
        0x5E => Ok("^".into()),
        0x5F => Ok("_".into()),
        0x60 => Ok("`".into()),
        0x61 => Ok("a".into()),
        0x62 => Ok("b".into()),
        0x63 => Ok("c".into()),
        0x64 => Ok("d".into()),
        0x65 => Ok("e".into()),
        0x66 => Ok("f".into()),
        0x67 => Ok("g".into()),
        0x68 => Ok("h".into()),
        0x69 => Ok("i".into()),
        0x6A => Ok("j".into()),
        0x6B => Ok("k".into()),
        0x6C => Ok("l".into()),
        0x6D => Ok("m".into()),
        0x6E => Ok("n".into()),
        0x6F => Ok("o".into()),
        0x70 => Ok("p".into()),
        0x71 => Ok("q".into()),
        0x72 => Ok("r".into()),
        0x73 => Ok("s".into()),
        0x74 => Ok("t".into()),
        0x75 => Ok("u".into()),
        0x76 => Ok("v".into()),
        0x77 => Ok("w".into()),
        0x78 => Ok("x".into()),
        0x79 => Ok("y".into()),
        0x7A => Ok("z".into()),
        0x7B => Ok("{".into()),
        0x7C => Ok("|".into()),
        0x7D => Ok("}".into()),
        0x7E => Ok("[$7E]".into()),
        0x7F => Ok("[$7F]".into()),
        0x80 => Ok("€".into()),
        0x81 => Ok("[$81]".into()),
        0x82 => Ok("[$82]".into()),
        0x83 => Ok("[$83]".into()),
        0x84 => Ok("[$84]".into()),
        0x85 => Ok("…".into()),
        0x86 => Ok("†".into()),
        0x87 => Ok("[$87]".into()),
        0x88 => Ok("ˆ".into()),
        0x89 => Ok("‰".into()),
        0x8A => Ok("Š".into()),
        0x8B => Ok("‹".into()),
        0x8C => Ok("Œ".into()),
        0x8D => Ok("[e]".into()),
        0x8E => Ok("Ž".into()),
        0x8F => Ok("[è]".into()),
        0x90 => Ok("•".into()),
        0x91 => Ok("‘".into()),
        0x92 => Ok("’".into()),
        0x93 => Ok("“".into()),
        0x94 => Ok("”".into()),
        0x95 => Ok("•".into()),
        0x96 => Ok("[er]".into()),
        0x97 => Ok("[re]".into()),
        0x98 => Ok("~".into()),
        0x99 => Ok("™".into()),
        0x9A => Ok("š".into()),
        0x9B => Ok("›".into()),
        0x9C => Ok("œ".into()),
        0x9D => Ok("•".into()),
        0x9E => Ok("ž".into()),
        0x9F => Ok("Ÿ".into()),
        0xA0 => Ok(" ".into()),
        0xA1 => Ok("¡".into()),
        0xA2 => Ok("¢".into()),
        0xA3 => Ok("£".into()),
        0xA4 => Ok("¤".into()),
        0xA5 => Ok("¥".into()),
        0xA6 => Ok("¦".into()),
        0xA7 => Ok("§".into()),
        0xA8 => Ok("¨".into()),
        0xA9 => Ok("©".into()),
        0xAA => Ok("ª".into()),
        0xAB => Ok("«".into()),
        0xAC => Ok("¬".into()),
        0xAD => Ok("\u{00AD}".into()),
        0xAE => Ok("®".into()),
        0xAF => Ok("¯".into()),
        0xB0 => Ok("°".into()),
        0xB1 => Ok("±".into()),
        0xB2 => Ok("²".into()),
        0xB3 => Ok("³".into()),
        0xB4 => Ok("´".into()),
        0xB5 => Ok("µ".into()),
        0xB6 => Ok("¶".into()),
        0xB7 => Ok("„".into()),
        0xB8 => Ok("‚".into()),
        0xB9 => Ok("¹".into()),
        0xBA => Ok("º".into()),
        0xBB => Ok("»".into()),
        0xBC => Ok("←".into()),
        0xBD => Ok("♂".into()),
        0xBE => Ok("♀".into()),
        0xBF => Ok("¿".into()),
        0xC0 => Ok("À".into()),
        0xC1 => Ok("Á".into()),
        0xC2 => Ok("Â".into()),
        0xC3 => Ok("Ã".into()),
        0xC4 => Ok("Ä".into()),
        0xC5 => Ok("Å".into()),
        0xC6 => Ok("Æ".into()),
        0xC7 => Ok("Ç".into()),
        0xC8 => Ok("È".into()),
        0xC9 => Ok("É".into()),
        0xCA => Ok("Ê".into()),
        0xCB => Ok("Ë".into()),
        0xCC => Ok("Ì".into()),
        0xCD => Ok("Í".into()),
        0xCE => Ok("Î".into()),
        0xCF => Ok("Ï".into()),
        0xD0 => Ok("Ð".into()),
        0xD1 => Ok("Ñ".into()),
        0xD2 => Ok("Ò".into()),
        0xD3 => Ok("Ó".into()),
        0xD4 => Ok("Ô".into()),
        0xD5 => Ok("Õ".into()),
        0xD6 => Ok("Ö".into()),
        0xD7 => Ok("×".into()),
        0xD8 => Ok("Ø".into()),
        0xD9 => Ok("Ù".into()),
        0xDA => Ok("Ú".into()),
        0xDB => Ok("Û".into()),
        0xDC => Ok("Ü".into()),
        0xDD => Ok("Ý".into()),
        0xDE => Ok("Þ".into()),
        0xDF => Ok("ß".into()),
        0xE0 => Ok("à".into()),
        0xE1 => Ok("á".into()),
        0xE2 => Ok("â".into()),
        0xE3 => Ok("ã".into()),
        0xE4 => Ok("ä".into()),
        0xE5 => Ok("å".into()),
        0xE6 => Ok("æ".into()),
        0xE7 => Ok("ç".into()),
        0xE8 => Ok("è".into()),
        0xE9 => Ok("é".into()),
        0xEA => Ok("ê".into()),
        0xEB => Ok("ë".into()),
        0xEC => Ok("ì".into()),
        0xED => Ok("í".into()),
        0xEE => Ok("î".into()),
        0xEF => Ok("ï".into()),
        0xF0 => Ok("ð".into()),
        0xF1 => Ok("ñ".into()),
        0xF2 => Ok("ò".into()),
        0xF3 => Ok("ó".into()),
        0xF4 => Ok("ô".into()),
        0xF5 => Ok("õ".into()),
        0xF6 => Ok("ö".into()),
        0xF7 => Ok("÷".into()),
        0xF8 => Ok("ø".into()),
        0xF9 => Ok("ù".into()),
        0xFA => Ok("ú".into()),
        0xFB => Ok("û".into()),
        0xFC => Ok("ü".into()),
        0xFD => Ok("ý".into()),
        0xFE => Ok("þ".into()),
        0xFF => Ok("ÿ".into()),
    }
}

#[test]
fn test_round_trip() {
    let seq = ["A", "b", "c", "[er]"];
    let bytes = seq
        .iter()
        .map(|&c| pmd_seq_to_byte(c).unwrap())
        .collect::<Vec<u8>>();

    assert_eq!(
        bytes
            .iter()
            .map(|&b| byte_to_pmd_seq(b).unwrap())
            .collect::<Vec<String>>(),
        seq
    );
}

#[test]
#[should_panic]
fn test_invalid_sequence() {
    let invalid = "{BLH]";
    PmdChar::from_sequence(invalid).unwrap();
}

#[test]
fn test_pmd_string_parse() {
    let str = "Oak[END]";
    PmdString::try_from(str).unwrap();
}
