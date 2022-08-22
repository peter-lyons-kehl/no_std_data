//! no_std with heap implementation, but without `Vec` - out of `alloc` it uses
//! `Box` only
#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use core::fmt::{self, Debug, Formatter};
use core::str::Chars;

#[derive(Debug, PartialEq)]
pub struct Dna(&'static str);

pub enum Rna {
    GivenNucleotides(&'static str), // RNA nucleotides
    // Original DNA nucleotides, but *not* transformed.
    // Instead, it will generate RNA nucleotides on the fly by iterating when
    // the consumer calls `PartialEq::eq(...)` on `self`.
    DnaBased(&'static str),
}

pub enum RnaIterator {
    GivenNucleotides(Chars<'static>),
    DnaBased(Chars<'static>),
}

impl Rna {
    pub fn iter_box_dyn(&self) -> Box<dyn Iterator<Item = char>> {
        match *self {
            Rna::GivenNucleotides(rna) => Box::new(rna.chars()),

            Rna::DnaBased(dna) => Box::new(dna.chars().map(shared::dna_to_rna)),
        }
    }
}

impl Iterator for RnaIterator {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            RnaIterator::DnaBased(chars) => {
                let dna = chars.next();
                dna.map_or(None, |dna| Some(shared::dna_to_rna(dna)))
            }
            RnaIterator::GivenNucleotides(chars) => chars.next(),
        }
    }
}

impl Dna {
    /** On error return Err with a 0-based index of the first incorrect character. */
    pub fn new(dna: &'static str) -> Result<Self, usize> {
        match shared::check_dna(dna) {
            Ok(()) => Ok(Self(dna)),
            Err(i) => Err(i),
        }
    }

    pub fn into_rna(self) -> Rna {
        match self {
            Dna(dna) => Rna::DnaBased(dna),
        }
    }
}

impl PartialEq for Rna {
    fn eq(&self, other: &Self) -> bool {
        self.iter_box_dyn().eq(other.iter_box_dyn())
    }
}

impl Debug for Rna {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "RNA {{")?;
        match self {
            Rna::GivenNucleotides(rna) => {
                write!(f, "GivenNucleotides {{{rna}}}")?;
            }
            Rna::DnaBased(dna) => {
                write!(f, "DnaBased {{{dna}}} which translates to ")?;
                self.iter_box_dyn().try_for_each(|c| write!(f, "{c}"))?;
            }
        }
        write!(f, "}}")
    }
}

// @TODO
/*
#[cfg(test)]
pub mod test {
    use arrform::{arrform, ArrForm};

    #[test]
    #[allow(unused_must_use)]
    fn test_rna_given_nucleotides_debug() {
        super::Dna::new("GCTA").map(|dna| {
            let rna = dna.into_rna();
            let rna_af = arrform!(64, "{:?}", rna);
            assert_eq!(
                "RNA {DnaBased {GCTA} which translates to CGAU}",
                rna_af.as_str()
            );
        });
    }
}*/

impl Rna {
    /** On error return Err with a 0-based index of the first incorrect character. */
    pub fn new(rna: &'static str) -> Result<Self, usize> {
        match shared::check_rna(rna) {
            Ok(()) => Ok(Self::GivenNucleotides(rna)),
            Err(i) => Err(i),
        }
    }
}
