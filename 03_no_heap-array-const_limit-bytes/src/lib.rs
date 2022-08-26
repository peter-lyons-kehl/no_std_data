//! no_std heapless (bare metal/embedded-friendly)
#![no_std]

use core::fmt::{self, Debug, Formatter};
use core::str;

const MAX_NUM_RNA_NUCLEOTIDES: usize = 12;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Dna<'a>(&'a str);
use utils::{checks, DnaTrait, OurResult, RnaTrait};

#[derive(Default)]
pub struct Rna {
    // New to Rust? u8 type is an unsigned 8 bit integer, also used to represent a byte.
    rna: [u8; MAX_NUM_RNA_NUCLEOTIDES],
    len: usize,
}

impl<'a> Dna<'a> {
    pub fn new(dna: &'a str) -> OurResult<Self> {
        checks::check_dna(dna)?;
        Ok(Self(dna))
    }

    pub fn into_rna(&self) -> Rna {
        Rna::new_from_iter(self.0.chars().map(utils::dna_to_rna)).expect("RNA")
    }
}

impl Rna {
    /// Create a new [`Rna`] instance with given RNA nucleotides. If `rna` is valid, return  
    /// [`Some(Rna)`](Some<Rna>) containing the new instance. On error return [`Err`] with a 0-based
    /// index of the first incorrect character.
    pub fn new<'a>(rna: &'a str) -> OurResult<Self> {
        Self::new_from_iter(rna.chars())
    }

    fn new_from_iter(rna_iter: impl Iterator<Item = char>) -> OurResult<Self> {
        let mut result = Rna::default();
        for c in rna_iter {
            result.rna[result.len] = c as u8;
            result.len += 1;
        }
        // This would not work for Unicode in general.
        checks::check_rna_str(result.as_str())?;
        Ok(result)
    }

    fn as_str(&self) -> &str {
        str::from_utf8(&self.rna[..self.len]).expect("UTF-8 encoded string of RNA nucleotides")
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
    use super::OurResult;
    use alloc::format;

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
