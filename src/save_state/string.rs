use std::fmt::{Debug, Display, Write};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PokeString<const N: usize>([u8; N]);

impl<const N: usize> PokeString<N> {
    pub fn new(data: [u8; N]) -> Self {
        Self(data)
    }

    pub fn iter(&self) -> std::iter::Copied<std::slice::Iter<'_, u8>> {
        self.into_iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<const N: usize> AsRef<[u8]> for PokeString<N> {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<'a, const N: usize> IntoIterator for &'a PokeString<N> {
    type Item = u8;
    type IntoIter = std::iter::Copied<std::slice::Iter<'a, u8>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().copied()
    }
}

impl<const N: usize> Display for PokeString<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ch in &self.0 {
            match ch {
                0x50 => break,

                0x4a => f.write_str("𝔭𝔪")?,
                0x54 => f.write_str("POKé")?,
                0x56 => f.write_str("……")?,
                0x5b => f.write_str("PC")?,
                0x5c => f.write_str("TM")?,
                0x5d => f.write_str("TRAINER")?,
                0x5e => f.write_str("ROCKET")?,

                0x69 => f.write_char('𝐕')?,
                0x6a => f.write_char('𝐒')?,
                0x6d => f.write_char(':')?,
                0x72 => f.write_char('“')?,
                0x73 => f.write_char('”')?,
                0x74 => f.write_char('·')?,
                0x75 => f.write_char('…')?,

                0x79 => f.write_char('┌')?,
                0x7a => f.write_char('─')?,
                0x7b => f.write_char('┐')?,
                0x7c => f.write_char('│')?,
                0x7d => f.write_char('└')?,
                0x7e => f.write_char('┘')?,
                0x7f => f.write_char(' ')?,

                0x80..=0x99 => f.write_char((b'A' + (ch - 0x80)) as char)?,

                0x9a => f.write_char('(')?,
                0x9b => f.write_char(')')?,
                0x9c => f.write_char(':')?,
                0x9d => f.write_char(';')?,
                0x9e => f.write_char('[')?,
                0x9f => f.write_char(']')?,

                0xa0..=0xb9 => f.write_char((b'a' + (ch - 0xa0)) as char)?,

                0xc0 => f.write_char('Ä')?,
                0xc1 => f.write_char('Ö')?,
                0xc2 => f.write_char('Ü')?,
                0xc3 => f.write_char('ä')?,
                0xc4 => f.write_char('ö')?,
                0xc5 => f.write_char('ü')?,

                0xd0 => f.write_str("'d")?,
                0xd1 => f.write_str("'l")?,
                0xd2 => f.write_str("'m")?,
                0xd3 => f.write_str("'r")?,
                0xd4 => f.write_str("'s")?,
                0xd5 => f.write_str("'t")?,
                0xd6 => f.write_str("'v")?,

                0xdf => f.write_char('←')?,
                0xe0 => f.write_char('\'')?,
                0xe1 => f.write_char('𝔭')?,
                0xe2 => f.write_char('𝔪')?,
                0xe3 => f.write_char('-')?,

                0xe6 => f.write_char('?')?,
                0xe7 => f.write_char('!')?,
                0xe8 => f.write_char('.')?,
                0xe9 => f.write_char('&')?,

                0xea => f.write_char('é')?,
                0xeb => f.write_char('→')?,
                0xec => f.write_char('▷')?,
                0xed => f.write_char('▶')?,
                0xee => f.write_char('▼')?,
                0xef => f.write_char('♂')?,
                0xf0 => f.write_char('¥')?,
                0xf1 => f.write_char('×')?,
                0xf2 => f.write_char('.')?,
                0xf3 => f.write_char('/')?,
                0xf4 => f.write_char(',')?,
                0xf5 => f.write_char('♀')?,

                0xf6..=0xff => f.write_char((b'0' + (ch - 0xf6)) as char)?,

                _ => f.write_char('�')?,
            }
        }

        Ok(())
    }
}

impl<const N: usize> Debug for PokeString<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PokeString<{N}> {:?}", format!("{}", self))
    }
}
