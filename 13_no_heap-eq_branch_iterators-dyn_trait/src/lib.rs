//! no_std heapless (bare metal/embedded-friendly) implementation
#![no_std]

use core::fmt::{self, Debug, Formatter};
use utils::{checks, DnaTrait, OurResult, RnaTrait};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Dna<'a>(&'a str);

#[derive(Clone, Copy)]
pub enum Rna<'a> {
    GivenNucleotides(&'a str),
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
                self_dna_chars_mapped = dna.chars().map(utils::dna_to_rna);
                self_chars = &mut self_dna_chars_mapped;
            }
        }
        match other {
            Self::GivenNucleotides(rna) => {
                other_rna_chars = rna.chars();
                other_chars = &mut other_rna_chars;
            }
            Self::DnaBased(dna) => {
                other_dna_chars_mapped = dna.chars().map(utils::dna_to_rna);
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
impl<'a> Eq for Rna<'a> {}

impl<'a> Debug for Rna<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "RNA(")?;
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
        write!(f, ")")
    }
}

#[cfg(test)]
pub mod test {
    // TODO bookmark & Embed Presentation: `arrform` is an heapless no_std
    // alternative to format!(...). New to Rust? Exclamation mark indicates a
    // macro invocation

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
