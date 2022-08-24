//! no_std heapless (bare metal/embedded-friendly)
#![no_std]

use core::fmt::{self, Debug, Formatter};

/// DNA (DNA nucleotide sequence).  
/// Implementing [`Eq`] is not necessary, but valid.
#[derive(Debug, PartialEq, Eq)]
pub struct Dna<'a>(&'a str);

/// RNA (RNA nucleotide sequence).
pub enum Rna<'a> {
    /// Represented by given RNA nucleotides. Returned by [`Rna::new`].
    GivenNucleotides(&'a str),
    /// Represented by respective DNA nucleotides, but *not* transformed. Instead, methods of this
    /// type generate RNA nucleotides on the fly by iterating when the consumer calls
    /// [`PartialEq::eq`] or [`Debug::fmt`] on `&self`. See [`Rna::iter`].
    DnaBased(&'a str),
}

impl<'a> Dna<'a> {
    /// Create a new [`Dna`] instance with given DNA nucleotides. If `dna` is valid, return  
    /// [`Some(Dna)`](Some<Dna>) containing the new instance. On error return [`Err`] with a 0-based
    /// index of the first incorrect character.
    pub fn new(dna: &'a str) -> utils::Result<Self> {
        match utils::check_dna(dna) {
            Ok(()) => Ok(Self(dna)),
            Err(i) => Err(i),
        }
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

    /// Get the stored nucleotides (RNA for[Rna::GivenNucleotides], or DNA for [Rna::DnaBased]). Use
    /// together with [`Rna::is_dna_based`].
    fn stored_nucleotides(&self) -> &'a str {
        match *self {
            Self::GivenNucleotides(rna) => rna,
            Self::DnaBased(dna) => dna,
        }
    }

    fn is_dna_based(&self) -> bool {
        matches!(*self, Self::DnaBased(_))
    }

    /// Create an [`Iterator`] over `self`'s RNA nucleotides (chars). For  
    /// [RNA-based variant](Rna::GivenNucleotides) this iterates over the given nucleotides. For  
    /// [DNA-based variant](Rna::DnaBased) this translates the DNA nucleotides to RNA ones on the
    /// fly (without storing them anywhere).
    ///
    /// This return type can't be declared as `impl Iterator<Item = char> + 'a`, but it has to use
    /// `_` which indicates _lifetime elision_. Thanks to
    /// https://robinmoussu.gitlab.io/blog/post/2021-03-25_rust_iterators_tips_and_tricks.
    fn iter(&self) -> impl Iterator<Item = char> + '_ {
        self.stored_nucleotides().chars().map(|c| {
            if self.is_dna_based() {
                utils::dna_to_rna(c)
            } else {
                c
            }
        })
    }
}

impl<'a> PartialEq for Rna<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}
/// Not necessary for our purpose, but valid.
impl<'a> Eq for Rna<'a> {}

impl<'a> Debug for Rna<'a> {
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

    // Unit tests of a `no_std` crate can't use `std` either. However, they can use heap (even if
    // the crate being tested doesn't have access to heap).
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
