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
}

impl<'a> PartialEq for Rna<'a> {
    fn eq(&self, other: &Self) -> bool {
        // Even though the left and right iterators in the following `match`
        // branches are all `impl Iterator<Item = char>`, they are of
        // different types, hence not storable in the same variables. So we
        // store them separately. But we can store a reference to either, cast as `&dyn
        // Iterator<...>`, and store that `&dyn` in the same variable. We do so for both `self`
        // and `other`.
        let (mut self_rna_chars, mut self_dna_chars_mapped);
        let (mut other_rna_chars, mut other_dna_chars_mapped);
        let self_chars: &mut dyn Iterator<Item = char>;
        let other_chars: &mut dyn Iterator<Item = char>;

        match self {
            Self::GivenNucleotides(rna) => {
                self_rna_chars = rna.chars();
                self_chars = &mut self_rna_chars;
            }
            Self::DnaBased(dna) => {
                self_dna_chars_mapped = dna.chars().map(shared::dna_to_rna);
                self_chars = &mut self_dna_chars_mapped;
            }
        }
        match other {
            Self::GivenNucleotides(rna) => {
                other_rna_chars = rna.chars();
                other_chars = &mut other_rna_chars;
            }
            Self::DnaBased(dna) => {
                other_dna_chars_mapped = dna.chars().map(shared::dna_to_rna);
                other_chars = &mut other_dna_chars_mapped;
            }
        }
        // This &dyn call adds a dynamic dispatch overhead (once for the left side: `self`, and
        // multiple times for the right side: `other`), but the code may be clearer than in
        // ../../no_std-no_heap-eq_branch_iterators-matrix. Especially so if we used the (dynamic)
        // references multiple times.
        self_chars.eq(other_chars)
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
