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
    /** On error return Err with a 0-based index of the first incorrect character. */
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
    /** On error return Err with a 0-based index of the first incorrect character. */
    fn new(rna: &'a str) -> OurResult<Self> {
        checks::check_rna_str(rna)?;
        Ok(Self::GivenNucleotides(rna))
    }
}

impl<'a> PartialEq for Rna<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (&self, other) {
            // Even though the left and right iterators in the following `match`
            // branches are all `impl Iterator<Item = char>`, they are of
            // different types, hence not storable in the same variables. So we
            // need to handle them separately.
            (Self::GivenNucleotides(self_rna), Self::GivenNucleotides(other_rna)) => {
                self_rna.chars().eq(other_rna.chars())
            }
            (Self::GivenNucleotides(self_rna), Self::DnaBased(other_dna)) => self_rna
                .chars()
                .eq(other_dna.chars().map(utils::dna_to_rna)),
            (Self::DnaBased(self_dna), Self::GivenNucleotides(other_rna)) => self_dna
                .chars()
                .map(utils::dna_to_rna)
                .eq(other_rna.chars()),
            (Self::DnaBased(self_dna), Self::DnaBased(other_dna)) => {
                // No need to map both of them - their DNA must be the same, too
                self_dna.chars().eq(other_dna.chars())
            }
        }
    }
}
impl<'a> Eq for Rna<'a> {}

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
    // TODO bookmark & Embed Presentation: `arrform` is an heapless no_std
    // alternative to format!(...). New to Rust? Exclamation mark indicates a
    // macro invocation

    // Emb. Presentation: However, your unit tests can use full `std`:
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
