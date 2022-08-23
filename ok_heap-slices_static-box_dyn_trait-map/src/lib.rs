//! no_std with heap, but without `Vec` or `String` - out of `alloc` it uses `Box` only
#![no_std]
extern crate alloc;

use alloc::boxed::Box;
use core::fmt::{self, Debug, Formatter};

/// DNA (DNA nucleotide sequence).
/// Implementing [`Eq`] is not necessary, but valid.
#[derive(Debug, PartialEq, Eq)]
pub struct Dna(&'static str);

/// RNA (RNA nucleotide sequence).
pub enum Rna {
    /// Represented by given RNA nucleotides. Returned by [`Rna::new`].
    GivenNucleotides(&'static str),
    /// Represented by respective DNA nucleotides, but *not* transformed. Instead, methods of this
    /// type generate RNA nucleotides on the fly by iterating when the consumer calls
    /// [`PartialEq::eq`] or [`Debug::fmt`] on `&self`. See [`Rna::iter`].
    DnaBased(&'static str),
}

impl Dna {
    /// Create a new instance with given DNA nucleotides. On error return [`Err`] with a 0-based
    /// index of the first incorrect character.
    pub fn new(dna: &'static str) -> Result<Self, usize> {
        match shared::check_dna(dna) {
            Ok(()) => Ok(Self(dna)),
            Err(i) => Err(i),
        }
    }

    /// Create a [DNA-based variant of `Rna`](Rna::GivenNucleotides) instance, based on `self`. No
    /// transformation/iteration is done yet - see [`Rna::DnaBased`].
    pub fn into_rna(self) -> Rna {
        match self {
            Dna(dna) => Rna::DnaBased(dna),
        }
    }
}

impl Rna {
    /// Create a new instance with given RNA nucleotides. On error return [`Err`] with a 0-based
    /// index of the first incorrect character.
    pub fn new(rna: &'static str) -> Result<Self, usize> {
        match shared::check_rna(rna) {
            Ok(()) => Ok(Self::GivenNucleotides(rna)),
            Err(i) => Err(i),
        }
    }

    /// Create an [`Iterator`] over `self`'s RNA nucleotides (chars). For  
    /// [RNA-based variant](Rna::GivenNucleotides) this iterates over the given nucleotides. For  
    /// [DNA-based variant](Rna::DnaBased) this translates the DNA nucleotides to RNA ones on the
    /// fly (without storing them anywhere). Return the iterator as a boxed `dyn` trait object.
    fn iter(&self) -> Box<dyn Iterator<Item = char>> {
        match *self {
            Rna::GivenNucleotides(rna) => Box::new(rna.chars()),

            Rna::DnaBased(dna) => Box::new(dna.chars().map(shared::dna_to_rna)),
        }
    }
}

impl PartialEq for Rna {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}
/// Not necessary, but valid.
impl Eq for Rna {}

impl Debug for Rna {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "RNA {{")?;
        match self {
            Rna::GivenNucleotides(rna) => {
                write!(f, "GivenNucleotides {{{rna}}}")?;
            }
            Rna::DnaBased(dna) => {
                write!(f, "DnaBased {{{dna}}} which translates to ")?;
                self.iter().try_for_each(|c| write!(f, "{c}"))?;
            }
        }
        write!(f, "}}")
    }
}

#[cfg(test)]
pub mod test {
    //! Test(s) on top of Exercism's tests (which are in `../tests/`).

    // Unit tests of a `no_std` crate can't use `std` either. However, they can use heap.
    extern crate alloc;
    use alloc::format;

    #[test]
    /// Test both [`Dna::new`](super::Dna::new), and (primarily) [`core::fmt::Debug::fmt`] on
    /// [`Rna`](super::Rna). If [`Dna::new`](super::Dna::new) fails, it  
    /// returns [`Err`] containing `usize` index of the offending nucleotide (`char`), and this
    /// function then returns that [`Err`].
    fn test_rna_given_nucleotides_debug() -> Result<(), usize> {
        super::Dna::new("GCTA").map(|dna| {
            let rna = dna.into_rna();
            let rna_dbg = format!("{:?}", rna);
            assert_eq!(
                "RNA {DnaBased {GCTA} which translates to CGAU}",
                rna_dbg.as_str()
            );
        })
    }
}
