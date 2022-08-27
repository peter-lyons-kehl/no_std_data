//! no_std heapless (bare metal/embedded-friendly)
#![no_std]

use core::fmt::{self, Debug, Formatter};
use core::str;
use utils::api_tests_mut::RnaTraitMut;
use utils::{checks, DnaTrait, OurResult, RnaTrait};

const MAX_NUM_RNA_NUCLEOTIDES: usize = 12;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Dna<'a>(&'a str);

#[derive(Default)]
pub struct Rna {
    // New to Rust? u8 type is an unsigned 8 bit integer, also used to represent a byte.
    rna: [u8; MAX_NUM_RNA_NUCLEOTIDES],
    len: usize,
}

impl<'a> DnaTrait<'a, Rna> for Dna<'a> {
    fn new(dna: &'a str) -> OurResult<Self> {
        checks::check_dna(dna)?;
        Ok(Self(dna))
    }

    fn into_rna(&self) -> Rna {
        Rna::new_from_iter(self.0.chars().map(utils::dna_to_rna)).expect("RNA")
    }
}

impl<'a> RnaTrait<'a> for Rna {
    /// Create a new [`Rna`] instance with given RNA nucleotides. If `rna` is valid, return  
    /// [`Some(Rna)`](Some<Rna>) containing the new instance. On error return [`Err`] with a 0-based
    /// index of the first incorrect character.
    fn new(rna: &'a str) -> OurResult<Self> {
        Self::new_from_iter(rna.chars())
    }
}

impl Rna {
    fn set_from_iter_impl(&mut self, rna_iter: impl Iterator<Item = char>) -> OurResult<()> {
        let mut char_to_utf8 = [0u8; 4];
        let mut len = 0usize;
        for c in rna_iter {
            let utf8 = c.encode_utf8(&mut char_to_utf8[..]);
            for i in 0..utf8.len() {
                self.rna[len] = char_to_utf8[i];
                len += 1;
            }
        }
        self.len = len;
        checks::check_rna_str(self.as_str())?;
        Ok(())
    }
    fn new_from_iter(rna_iter: impl Iterator<Item = char>) -> OurResult<Self> {
        let mut result = Rna::default();
        result.set_from_iter_impl(rna_iter)?;
        Ok(result)
    }

    // @TODO This could be stored & accessed through a Cell. Cell is fine, since we don't have
    // threads on no_std anyway.
    fn as_str(&self) -> &str {
        str::from_utf8(&self.rna[..self.len]).expect("UTF-8 encoded string of RNA nucleotides")
    }
}

impl<'a> RnaTraitMut<'a> for Rna {
    fn set_from_iter(&mut self, iter: &mut dyn Iterator<Item = char>) -> OurResult<()> {
        // This wouldn't compile without the extra .map() or some other chaining.
        self.set_from_iter_impl(iter.map(core::convert::identity))
    }
}

impl PartialEq for Rna {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for Rna {}

impl Debug for Rna {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "RNA {{{}}}", self.as_str())
    }
}

impl Clone for Rna {
    fn clone(&self) -> Self {
        let mut rna = [u8::default(); MAX_NUM_RNA_NUCLEOTIDES];
        for i in 0..self.len {
            rna[i] = self.rna[i];
        }
        Self { rna, len: self.len }
    }
}

#[cfg(test)]
pub mod test {
    extern crate alloc;
    use alloc::format;
    use utils::{DnaTrait, OurResult, RnaTrait};

    #[test]
    fn test_rna_given_nucleotides_debug() -> OurResult<()> {
        let rna = super::Rna::new("CGAU")?;
        let rna_dbg = format!("{:?}", rna);
        assert_eq!("RNA {CGAU}", rna_dbg);
        Ok(())
    }

    #[test]
    fn test_rna_from_dna_debug() -> OurResult<()> {
        let dna = super::Dna::new("GCTA")?;
        let rna = dna.into_rna();
        let rna_dbg = format!("{:?}", rna);
        assert_eq!("RNA {CGAU}", rna_dbg);
        Ok(())
    }
}
