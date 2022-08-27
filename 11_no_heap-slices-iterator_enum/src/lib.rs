//! no_std heapless (bare metal/embedded-friendly)
#![no_std]

use core::fmt::{self, Debug, Formatter};
use core::str::Chars;
use utils::{checks, DnaTrait, OurResult, RnaTrait};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Dna<'a>(&'a str);

#[derive(Clone, Copy)]
pub enum Rna<'a> {
    /// Represented by given RNA nucleotides. Returned by [`Rna::new`].
    GivenNucleotides(&'a str),
    /// Represented by respective DNA nucleotides, but *not* transformed. Instead, methods of this
    /// type generate RNA nucleotides on the fly by iterating when the consumer calls
    /// [`PartialEq::eq`] or [`Debug::fmt`] on `&self`. See [`Rna::iter`].
    DnaBased(&'a str),
}

impl<'a> DnaTrait<'a, Rna<'a>> for Dna<'a> {
    /// Create a new [`Dna`] instance with given DNA nucleotides. If `dna` is valid, return  
    /// [`Some(Dna)`](Some<Dna>) containing the new instance. On error return [`Err`] with a 0-based
    /// index of the first incorrect character.
    fn new(dna: &'a str) -> OurResult<Self> {
        checks::check_dna(dna)?;
        Ok(Self(dna))
    }

    /// Create a [DNA-based variant of `Rna`](Rna::GivenNucleotides) instance, based on `self`. No
    /// transformation/iteration is done yet - see [`Rna::DnaBased`].
    fn into_rna(&self) -> Rna<'a> {
        match self {
            Dna(dna) => Rna::DnaBased(dna),
        }
    }
}

/// Iterator over RNA nucleotides. This iterates over either:
/// - given RNA ones (for [RnaIterator::GivenNucleotides]), or
/// - translated on the fly from DNA ones (for [RnaIterator::DnaBased]).
enum RnaIterator<'a> {
    GivenNucleotides(Chars<'a>),
    DnaBased(Chars<'a>),
}

impl<'a> RnaTrait<'a> for Rna<'a> {
    /// Create a new [`Rna`] instance with given RNA nucleotides -[`Rna::GivenNucleotides`] variant.
    /// If `rna` is valid, return  
    /// [`Some(Rna)`](Some<Rna>) containing the new instance. On error return [`Err`] with a 0-based
    /// index of the first incorrect character.
    fn new(rna: &'a str) -> OurResult<Self> {
        checks::check_rna_str(rna)?;
        Ok(Self::GivenNucleotides(rna))
    }
}

impl<'a> Rna<'a> {
    /// Create an [`RnaIterator`] over `self`'s RNA nucleotides (chars). For  
    /// [RNA-based variant](Rna::GivenNucleotides) this iterates over the given nucleotides. For  
    /// [DNA-based variant](Rna::DnaBased) this translates the DNA nucleotides to RNA ones on the
    /// fly (without storing them anywhere).
    ///
    /// We can't declare return type here as `impl Iterator<Item = char>` if we return a different
    /// expression for each `match *self` branch here. Why? Such alternative results would be two
    /// different implementations of [`Iterator`]. Hence we have our own type: [`RnaIterator`].
    fn iter(&self) -> RnaIterator<'a> {
        match *self {
            Rna::GivenNucleotides(rna) => RnaIterator::GivenNucleotides(rna.chars()),

            Rna::DnaBased(dna) => RnaIterator::DnaBased(dna.chars()),
        }
    }
}

impl<'a> Iterator for RnaIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            RnaIterator::DnaBased(chars) => {
                let dna = chars.next();
                match dna {
                    Some(nucl) => Some(utils::dna_to_rna(nucl)),
                    None => None,
                }
            }
            RnaIterator::GivenNucleotides(chars) => chars.next(),
        }
    }
}

impl<'a> PartialEq for Rna<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}
impl<'a> Eq for Rna<'a> {}

impl<'a> Debug for Rna<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "RNA {{")?;
        self.iter().try_for_each(|c| write!(f, "{c}"))?;
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
