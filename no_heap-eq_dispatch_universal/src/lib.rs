#![allow(unused)]
//! no_std heapless (bare metal/embedded-friendly) implementation
#![no_std]

use core::fmt::{self, Debug, Formatter};

#[derive(Debug, PartialEq)]
pub struct Dna<'a>(&'a str);

pub enum Rna<'a> {
    GivenNucleotides(&'a str), // RNA nucleotides
    // Original DNA nucleotides, but *not* transformed. Instead, it will
    // generate RNA nucleotides on the fly by iterating when the consumer calls
    // `PartialEq::eq(...)` on `self`.
    DnaBased(&'a str),
}

impl<'a> Dna<'a> {
    /** On error return Err with a 0-based index of the first incorrect character. */
    pub fn new(dna: &'a str) -> Result<Self, usize> {
        match shared::check_dna(dna) {
            Ok(()) => Ok(Self(dna)),
            Err(i) => Err(i),
        }
    }

    pub fn into_rna(self) -> Rna<'a> {
        match self {
            Dna(dna) => Rna::DnaBased(dna),
        }
    }
}

impl<'a> Rna<'a> {
    /** On error return Err with a 0-based index of the first incorrect character. */
    pub fn new(rna: &'a str) -> Result<Self, usize> {
        match shared::check_rna(rna) {
            Ok(()) => Ok(Self::GivenNucleotides(rna)),
            Err(i) => Err(i),
        }
    }

    fn with_chars_universal<P, C, R>(&self, param: P, closure: C) -> R
    where
        C: Fn(&mut dyn Iterator<Item = char>, P) -> R,
    {
        match self {
            Rna::GivenNucleotides(rna) => closure(&mut rna.chars(), param),
            Rna::DnaBased(dna) => closure(&mut dna.chars().map(shared::dna_to_rna), param),
        }
    }
}

impl<'a> PartialEq for Rna<'a> {
    fn eq(&self, other: &Self) -> bool {
        fn inner(
            iter_one: &mut dyn Iterator<Item = char>,
            iter_two: &mut dyn Iterator<Item = char>,
        ) -> bool {
            iter_one.eq(iter_two)
        }

        self.with_chars_universal(
            other,
            |self_chars: &mut dyn Iterator<Item = char>, other: &Self| {
                other.with_chars_universal(
                    self_chars,
                    |other_chars: &mut dyn Iterator<Item = char>,
                     self_chars: &mut dyn Iterator<Item = char>| {
                        other_chars.eq(self_chars)
                    },
                )
            },
        )
    }
}

impl<'a> Debug for Rna<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "RNA {{")?;
        match self {
            Rna::GivenNucleotides(rna) => {
                write!(f, "GivenNucleotides {{{rna}}}")?;
            }
            Rna::DnaBased(dna) => {
                write!(f, "DnaBased {{{dna}}} which translates to ")?;
                // Compared to ../../no_std-no_heap-slices-iterator here we
                // don't have self.iter(). So we map dna to rna here:
                dna.chars()
                    .map(shared::dna_to_rna)
                    .try_for_each(|c| write!(f, "{c}"))?;
            }
        }
        write!(f, "}}")
    }
}

#[cfg(test)]
pub mod test {
    // TODO bookmark & Embed Presentation: `arrform` is an heapless no_std
    // alternative to format!(...). New to Rust? Exclamation mark indicates a
    // macro invocation

    // Emb. Presentation: However, your unit tests can use full `std`:
    extern crate alloc;
    use alloc::format;

    #[test]
    #[allow(unused_must_use)]
    fn test_rna_given_nucleotides_debug() {
        super::Dna::new("GCTA").map(|dna| {
            let rna = dna.into_rna();
            let rna_dbg = format!("{:?}", rna);
            assert_eq!(
                "RNA {DnaBased {GCTA} which translates to CGAU}",
                rna_dbg.as_str()
            );
        });
    }
}
