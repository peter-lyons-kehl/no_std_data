//! no_std heapless (bare metal/embedded-friendly) implementation
#![no_std]

use core::fmt::{self, Debug, Formatter};
use utils::{checks, DnaTrait, OurResult, RnaTrait};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Dna<'a>(&'a str);

#[derive(Clone, Copy)]
pub enum Rna<'a> {
    GivenNucleotides(&'a str), // RNA nucleotides
    // Original DNA nucleotides, but *not* transformed. Instead, it will
    // generate RNA nucleotides on the fly by iterating when the consumer calls
    // `PartialEq::eq(...)` on `self`.
    DnaBased(&'a str),
}

impl<'a> DnaTrait<'a, Rna<'a>> for Dna<'a> {
    fn new(dna: &'a str) -> OurResult<Self> {
        checks::check_dna(dna)?;
        Ok(Self(dna))
    }

    fn into_rna(&self) -> Rna<'a> {
        match self {
            Dna(dna) => Rna::DnaBased(dna),
        }
    }
}

impl<'a> RnaTrait<'a> for Rna<'a> {
    fn new(rna: &'a str) -> OurResult<Self> {
        checks::check_rna_str(rna)?;
        Ok(Self::GivenNucleotides(rna))
    }
}

impl<'a> Rna<'a> {
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
impl<'a> Eq for Rna<'a> {}

impl<'a> Debug for Rna<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Rna(\"")?;
        match self {
            Rna::GivenNucleotides(rna) => {
                write!(f, "{rna}")?;
            }
            Rna::DnaBased(dna) => {
                // Compared to ../../no_std-no_heap-slices-iterator (TODO fix <--) here we
                // don't have self.iter(). So we map dna to rna here:
                dna.chars()
                    .map(utils::dna_to_rna)
                    .try_for_each(|c| write!(f, "{c}"))?;
            }
        }
        write!(f, "\")")
    }
}
