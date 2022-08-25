#![no_std]

use core::fmt::{self, Debug, Formatter};
use core::str;
use utils::OurResult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Dna<'a>(&'a str);

/// This can't derive, neither implement, [`Clone`]. Why? Because mutable reference (contained in
/// this type) can't be cloned.
pub enum Rna<'a> {
    GivenNucleotides(&'a str),
    /// The characters in the byte slice represent, or will represent, RNA.
    MutableNucleotides {
        /// The whole/available storage.
        rna: &'a mut [u8],
        /// Length of the valid subslice (used storage).
        len: usize,
    },
}

impl<'a> Dna<'a> {
    pub fn new(dna: &'a str) -> OurResult<Self> {
        utils::check_dna(dna)?;
        Ok(Self(dna))
    }

    pub fn into_rna<'s>(&self, storage: &'s mut [u8]) -> Rna
    where
        's: 'a,
    {
        Rna::new_from_iter_and_storage(self.0.chars().map(utils::dna_to_rna), storage).expect("RNA")
    }
}

impl<'a> Rna<'a> {
    pub fn new(rna: &'a str) -> OurResult<Self> {
        utils::check_rna_str(rna)?;
        Ok(Self::GivenNucleotides(rna))
    }

    // Not Unicode-friendly.
    fn new_from_iter_and_storage<'s>(
        rna_iter: impl Iterator<Item = char>,
        storage: &'s mut [u8],
    ) -> OurResult<Self>
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
        utils::check_rna_str(result.as_str())?;
        Ok(result)
    }

    fn as_str(&self) -> &str {
        match self {
            Self::GivenNucleotides(rna) => rna,
            Self::MutableNucleotides { rna, len } => {
                str::from_utf8(&rna[..*len]).expect("UTF-8 encoded string of RNA nucleotides")
            }
        }
    }
}

impl<'a> PartialEq for Rna<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
/// Not necessary, but valid.
impl<'a> Eq for Rna<'a> {}

impl<'a> Debug for Rna<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        // @TODO all other: Change RNA -> Rna
        write!(f, "Rna {{{}}}", self.as_str())
    }
}

impl<'l, 'r> PartialEq<&Rna<'r>> for Rna<'l> {
    fn eq(&self, other: &&Rna<'r>) -> bool {
        self.as_str() == other.as_str()
    }
}
impl<'l, 'r> PartialEq<Rna<'r>> for &Rna<'l> {
    fn eq(&self, other: &Rna<'r>) -> bool {
        self.as_str() == other.as_str()
    }
}

#[cfg(test)]
pub mod test {
    extern crate alloc;
    use super::{Dna, Rna};
    use alloc::format;

    #[test]
    fn test_rna_given_nucleotides_debug() {
        Dna::new("GCTA").map(|dna| {
            // Many use cases need an extra variable!
            let mut storage = [0u8; 4];

            let rna = dna.into_rna(&mut storage);
            let rna_dbg = format!("{:?}", rna);
            assert_eq!("Rna {CGAU}", rna_dbg.as_str());
        });
    }

    #[test]
    fn test_rna_shared() {
        let rna = Rna::new("CGAU").unwrap();

        let dna = Dna::new("GCTA").unwrap();
        let mut dna_transformed_storage = [0u8; 4];
        let dna_transformed = dna.into_rna(&mut dna_transformed_storage);

        assert_eq!(rna, dna_transformed);
        assert_eq!(dna_transformed, rna);

        let rna_ref = &rna;
        assert_eq!(rna, rna_ref);
        assert_eq!(rna_ref, rna);

        assert_eq!(rna_ref, dna_transformed);
        assert_eq!(dna_transformed, rna_ref);

        let dna_transformed_ref = &dna_transformed;
        assert_eq!(rna, dna_transformed_ref);
        assert_eq!(dna_transformed_ref, rna);

        assert_eq!(rna_ref, dna_transformed_ref);
        assert_eq!(dna_transformed_ref, rna_ref);
    }
}
