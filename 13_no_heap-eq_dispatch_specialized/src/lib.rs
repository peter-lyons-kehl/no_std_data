//! no_std heapless (bare metal/embedded-friendly)
#![no_std]

use core::fmt::{self, Debug, Formatter};

/// DNA (DNA nucleotide sequence).
///
/// Implementing [`Eq`] is not necessary for our purpose, but valid.
#[derive(Debug, PartialEq, Eq)]
pub struct Dna<'a>(&'a str);

/// RNA (RNA nucleotide sequence).
pub enum Rna<'a> {
    /// Represented by given RNA nucleotides. Returned by [`Rna::new`].
    GivenNucleotides(&'a str),
    /// Represented by respective DNA nucleotides, but *not* transformed. Instead, methods of this
    /// type generate RNA nucleotides on the fly by iterating when the consumer calls
    /// [`PartialEq::eq`] or [`Debug::fmt`] on `&self`. See [`Rna::eq`] and [`Rna::iter`].
    DnaBased(&'a str),
}

impl<'a> Dna<'a> {
    /// Create a new [`Dna`] instance with given DNA nucleotides. If `dna` is valid, return  
    /// [`Some(Dna)`](Some<Dna>) containing the new instance. On error return [`Err`] with a 0-based
    /// index of the first incorrect character.
    pub fn new(dna: &'a str) -> utils::Result<Self> {
        utils::check_dna(dna)?;
        Ok(Self(dna))
    }

    /// Create a [DNA-based variant of `Rna`](Rna::GivenNucleotides) instance, based on `self`. No
    /// transformation/iteration is done yet - see [`Rna::DnaBased`].
    pub fn into_rna(self) -> Rna<'a> {
        match self {
            Dna(dna) => Rna::DnaBased(dna),
        }
    }
}

impl<'a> Rna<'a> {
    /// Create a new [`Rna`] instance with given RNA nucleotides -[`Rna::GivenNucleotides`] variant.
    /// If `rna` is valid, return  
    /// [`Some(Rna)`](Some<Rna>) containing the new instance. On error return [`Err`] with a 0-based
    /// index of the first incorrect character.
    pub fn new(rna: &'a str) -> utils::Result<Self> {
        match utils::check_rna_str(rna) {
            Ok(()) => Ok(Self::GivenNucleotides(rna)),
            Err(i) => Err(i),
        }
    }

    /// Get an [`Iterator`] over `self`'s RNA nucleotides (chars), and call `closure` with that
    /// (`self`'s) iterator and `other_rna_chars`. For  
    /// [RNA-based variant](Rna::GivenNucleotides) this iterates over the given nucleotides. For  
    /// [DNA-based variant](Rna::DnaBased) this translates the DNA nucleotides to RNA ones on the
    /// fly (without storing them anywhere).
    fn with_chars<R, C>(&self, other_rna_chars: &mut dyn Iterator<Item = char>, closure: C) -> R
    where
        C: Fn(&mut dyn Iterator<Item = char>, &mut dyn Iterator<Item = char>) -> R,
    {
        match self {
            Rna::GivenNucleotides(rna) => closure(&mut rna.chars(), other_rna_chars),
            Rna::DnaBased(dna) => closure(&mut dna.chars().map(utils::dna_to_rna), other_rna_chars),
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
            Self::DnaBased(dna) => other.with_chars(&mut dna.chars().map(utils::dna_to_rna), inner),
        }
    }
}
/// Not necessary, but valid.
impl<'a> Eq for Rna<'a> {}

impl<'a> Debug for Rna<'a> {
    /// Compared to [../../no_heap-slices-iterator]([../../no_heap-slices-iterator),
    /// [Self::DnaBased] variant here doesn't have `self.iter()`. So we map DNA to RNA chars here.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "RNA {{")?;
        match self {
            Rna::GivenNucleotides(rna) => {
                write!(f, "GivenNucleotides {{{rna}}}")?;
            }
            Rna::DnaBased(dna) => {
                write!(f, "DnaBased {{{dna}}} which translates to ")?;
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
    //! Test(s) on top of Exercism's tests (which are in `../tests/`).

    // Unit tests of a `no_std` crate can't use `std` either. However, they can use heap (even if
    // the crate being tested doesn't have access to heap).
    extern crate alloc;
    use alloc::format;

    #[test]
    /// Test both [`Dna::new`](super::Dna::new), and (primarily) [`core::fmt::Debug::fmt`] on
    /// [`Rna`](super::Rna). If [`Dna::new`](super::Dna::new) fails, it  
    /// returns [`Err`] containing `usize` index of the offending nucleotide (`char`), and this
    /// function then returns that [`Err`].
    fn test_rna_given_nucleotides_debug() -> utils::Result<()> {
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
