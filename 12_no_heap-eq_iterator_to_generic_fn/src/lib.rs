#![allow(unused)]
//! no_std heapless (bare metal/embedded-friendly) implementation
#![no_std]

use core::fmt::{self, Debug, Formatter};
use utils::OurResult;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Dna<'a>(&'a str);

#[derive(Clone, Copy)]
pub enum Rna<'a> {
    GivenNucleotides(&'a str), // RNA nucleotides
    // Original DNA nucleotides, but *not* transformed. Instead, it will
    // generate RNA nucleotides on the fly by iterating when the consumer calls
    // `PartialEq::eq(...)` on `self`.
    DnaBased(&'a str),
}

impl<'a> Dna<'a> {
    /** On error return Err with a 0-based index of the first incorrect character. */
    pub fn new(dna: &'a str) -> OurResult<Self> {
        utils::check_dna(dna)?;
        Ok(Self(dna))
    }

    pub fn into_rna(&self) -> Rna<'a> {
        match self {
            Dna(dna) => Rna::DnaBased(dna),
        }
    }
}

impl<'a> Rna<'a> {
    /** On error return Err with a 0-based index of the first incorrect character. */
    pub fn new(rna: &'a str) -> OurResult<Self> {
        utils::check_rna_str(rna)?;
        Ok(Self::GivenNucleotides(rna))
    }

    fn eq_iterate_other<I>(&self, other_rna_chars: I) -> bool
    where
        I: Iterator<Item = char>,
    {
        match self {
            Rna::GivenNucleotides(rna) => rna.chars().eq(other_rna_chars),
            Rna::DnaBased(dna) => dna.chars().map(utils::dna_to_rna).eq(other_rna_chars),
        }
    }
}

impl<'a> PartialEq for Rna<'a> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::GivenNucleotides(rna) => other.eq_iterate_other(rna.chars()),
            Self::DnaBased(dna) => other.eq_iterate_other(dna.chars().map(utils::dna_to_rna)),
        }
    }
}

impl<'a> Debug for Rna<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "RNA {{")?;
        match self {
            Rna::GivenNucleotides(rna) => {
                write!(f, "{rna}")?;
            }
            Rna::DnaBased(dna) => {
                // Compared to ../../no_std-no_heap-slices-iterator here we
                // don't have self.iter(). So we map dna to rna here:
                dna.chars()
                    .map(utils::dna_to_rna)
                    .try_for_each(|c| write!(f, "{c}"))?;
            }
        }
        write!(f, "}}")
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
