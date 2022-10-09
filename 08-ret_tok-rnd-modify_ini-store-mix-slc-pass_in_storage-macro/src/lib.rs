//! This crate doesn't implement utils::{DnaTrait, RnaTrait}, because the function signature of
//! [`Dna::into_rna`] here is different - it needs an extra parameter (storage slice).
#![no_std]

use core::str;
use utils::{self, checks, OurResult};

#[macro_export]
macro_rules! into_rna {
    ($dna:expr, $storage:expr) => {
        // We generate a block {...}, and the last expression is the result Rna instance.
        {
            // @TODO vairable hygiene  - 2x
            //
            // NOT using the following, because then the result would be tied to its lifetime:
            //
            // let tmp_storage = $storage;
            //
            // TODO change to: dna.prepare_sto..($storage)
            let len = Dna::prepare_storage_from_dna($dna, &mut $storage);
            Rna::from_prepared_storage(&$storage, len)
        }
    };
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Dna<'a>(&'a str);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rna<'a>(&'a str);

impl<'a> Dna<'a> {
    pub fn new(dna: &'a str) -> OurResult<Self> {
        checks::check_dna(dna)?;
        Ok(Self(dna))
    }

    /// TODO doc.
    /// Return the UTF-8 length.
    pub fn prepare_storage_from_dna<'s>(&self, storage: &'s mut [u8]) -> usize {
        utils::char_iter_to_bytes(storage, self.0.chars().map(utils::dna_to_rna))
    }
}

impl<'a> Rna<'a> {
    pub fn new(rna: &'a str) -> OurResult<Self> {
        checks::check_rna_str(rna)?;
        Ok(Self(rna))
    }

    /// This takes an immutable storage that was previously prepared by
    /// [`Dna::prepare_storage_from_dna`]. This separation allows us to use shared slice (instead of
    /// a mutable slice) here.
    pub fn from_prepared_storage<'s>(prepared_storage: &'s [u8], len: usize) -> Self
    where
        's: 'a,
    {
        let result = Self(
            str::from_utf8(&prepared_storage[..len])
                .expect("UTF-8 encoded string of RNA nucleotides"),
        );
        // This would not work for Unicode in general.
        checks::check_rna_str(result.as_str()).expect("RNA string");
        result
    }

    fn as_str(&self) -> &str {
        self.0
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
    use super::{into_rna, Dna, Rna};

    /// Testing that equality is defined for references - because we can't share instances of this
    /// type in any other way.
    #[test]
    fn test_rna_shared() {
        let rna = Rna::new("CGAU").unwrap();

        let dna = Dna::new("GCTA").unwrap();
        let mut dna_transformed_storage = [0u8; 4];
        let dna_transformed = into_rna!(&dna, dna_transformed_storage);

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
