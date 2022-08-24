#![no_std]

use core::fmt::{self, Debug, Formatter};
use core::str;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Dna<'a>(&'a str);

/// This can't derive, neither implement, [`Clone`]. Why? Because mutable reference (contained in
/// this type) can't be cloned.
pub enum Rna<'a> {
    GivenNucleotides(&'a str),
    MutableNucleotides { rna: &'a mut [u8], len: usize },
}

//@TODO
//pub struct RnaShared<'a> {}

impl<'a> Dna<'a> {
    pub fn new(dna: &'a str) -> utils::Result<Self> {
        utils::check_dna(dna)?;
        Ok(Self(dna))
    }

    pub fn into_rna<'s>(self, storage: &'s mut [u8]) -> Rna
    where
        's: 'a,
    {
        Rna::new_from_iter_and_storage(self.0.chars().map(utils::dna_to_rna), storage).expect("RNA")
    }
}

impl<'a> Rna<'a> {
    pub fn new(rna: &'a str) -> utils::Result<Self> {
        utils::check_rna_str(rna)?;
        Ok(Self::GivenNucleotides(rna))
    }

    fn new_from_iter_and_storage<'s>(
        rna_iter: impl Iterator<Item = char>,
        storage: &'s mut [u8],
    ) -> utils::Result<Self>
    where
        's: 'a,
    {
        let mut len = 0usize;
        for c in rna_iter {
            storage[len] = c as u8;
            len += 1;
        }
        let result = Self::MutableNucleotides { rna: storage, len };
        // This would not work for Unicode in general.
        utils::check_rna_char_iter(result.bytes().iter().map(|&b| b as char))?;
        Ok(result)
    }

    fn bytes(&self) -> &[u8] {
        match self {
            Self::GivenNucleotides(rna) => rna.as_bytes(),
            Self::MutableNucleotides { rna, len } => &rna[..*len],
        }
    }
}

impl<'a> PartialEq for Rna<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.bytes() == other.bytes()
    }
}
/// Not necessary, but valid.
impl<'a> Eq for Rna<'a> {}

impl<'a> Debug for Rna<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "RNA {{{}}}",
            str::from_utf8(self.bytes()).expect("UTF-8 encoded string")
        )
    }
}

#[cfg(test)]
pub mod test {
    extern crate alloc;
    use alloc::format;

    #[test]
    #[allow(unused_must_use)]
    fn test_rna_given_nucleotides_debug() {
        super::Dna::new("GCTA").map(|dna| {
            // Many use cases need an extra variable!
            let mut storage = [0u8; 4];

            let rna = dna.into_rna(&mut storage);
            let rna_dbg = format!("{:?}", rna);
            assert_eq!("RNA {CGAU}", rna_dbg.as_str());
        });
    }
}
