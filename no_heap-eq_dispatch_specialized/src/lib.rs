//! no_std heapless (bare metal/embedded-friendly)
#![no_std]
use core::fmt::{self, Debug, Formatter};

/// DNA (DNA nucleotide sequence)
#[derive(Debug, PartialEq, Eq)]
pub struct Dna<'a>(&'a str);

/// RNA (RNA nucleotide sequence).
pub enum Rna<'a> {
    /// Represented by given RNA nucleotides.
    GivenNucleotides(&'a str),
    /// Represented by respective DNA nucleotides, but *not* transformed. Instead, methods of this
    /// type generate RNA nucleotides on the fly by iterating when the consumer calls
    /// [`PartialEq::eq`] or [`Debug::fmt`] on `&self`. See [`Rna::new()`].
    DnaBased(&'a str),
}

impl<'a> Dna<'a> {
    /// On error: return [`Err`] with a 0-based index of the first incorrect character.
    pub fn new(dna: &'a str) -> Result<Self, usize> {
        shared::check_dna(dna)?;
        Ok(Self(dna))
    }
    /// Transform to (the DNA-based variant of) [`Rna`]. No transformation/iteration is done yet -
    /// see [`Rna::DnaBased`] instead.
    pub fn into_rna(self) -> Rna<'a> {
        match self {
            Dna(dna) => Rna::DnaBased(dna),
        }
    }
}

impl<'a> Rna<'a> {
    /// On error return [`Err`] with a 0-based index of the first incorrect character.
    pub fn new(rna: &'a str) -> Result<Self, usize> {
        match shared::check_rna(rna) {
            Ok(()) => Ok(Self::GivenNucleotides(rna)),
            Err(i) => Err(i),
        }
    }

    /// Get an iterator over `self`'s RNA nucleotides (chars), and call `closure` with that
    /// (`self`'s) iterator and `other_rna_chars`.
    fn with_chars<R, C>(&self, other_rna_chars: &mut dyn Iterator<Item = char>, closure: C) -> R
    where
        C: Fn(&mut dyn Iterator<Item = char>, &mut dyn Iterator<Item = char>) -> R,
    {
        match self {
            Rna::GivenNucleotides(rna) => closure(&mut rna.chars(), other_rna_chars),
            Rna::DnaBased(dna) => {
                closure(&mut dna.chars().map(shared::dna_to_rna), other_rna_chars)
            }
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

        match self {
            Self::GivenNucleotides(rna) => other.with_chars(&mut rna.chars(), inner),
            Self::DnaBased(dna) => {
                other.with_chars(&mut dna.chars().map(shared::dna_to_rna), inner)
            }
        }
    }
}
/// Not necessary, but valid.
impl<'a> Eq for Rna<'a> {}

impl<'a> Debug for Rna<'a> {
    /// Compared to [../../no_std-no_heap-slices-iterator]([../../no_std-no_heap-slices-iterator),
    /// [Self::DnaBased] variant here doesn't have `self.iter()`. So we map DNA to RNA for it.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "RNA {{")?;
        match self {
            Rna::GivenNucleotides(rna) => {
                write!(f, "GivenNucleotides {{{rna}}}")?;
            }
            Rna::DnaBased(dna) => {
                write!(f, "DnaBased {{{dna}}} which translates to ")?;
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
    //! Test(s) on top of Exercism's tests (which are in `../tests/`).

    // Even though the library itself is `no_std`, the unit tests are in a separate crate, hence
    // they can use `alloc`, even full `std`.
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
