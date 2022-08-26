//! no_std heapless (bare metal/embedded-friendly)
#![no_std]

use core::fmt::{self, Debug, Formatter};
use utils::{checks, DnaTrait, OurResult, RnaTrait};

const MAX_NUM_RNA_NUCLEOTIDES: usize = 12;

// @TODO Others: Derive/impl Clone.

/// DNA (DNA nucleotide sequence). `Dna` itself is `&str` slice-based. (Sufficient for our purpose.)
/// Only `Rna` is array-based.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Dna<'a>(&'a str);

/// RNA (RNA nucleotide sequence). Storing RNA nucleotides.
///
/// We don't derive [`PartialEq`] or [`Debug`] or [`Clone`] or [`Copy`]. If we were using
/// [Serde](https://docs.rs/serde/latest/serde/), we wouln't derive its `Serialize` either. Why?
/// Because an [`Rna`] instance may contain leftover nucleotides.
///
/// Let's say we have derived [`PartialEq`] or [`Debug`] or [`Clone`] or [`Copy`]. Then (possibly
/// later) we add modification methods, but we'd forget to wipe out any unused characters after any
/// modification that shortens `len`. If we left in the derived [`PartialEq`] and [`Clone`] or
/// `Serialize`:
/// - Two instances with the same `len` and `rna[..len]` would be treated as unequal if their unused
///   characters (left from before the modification) would differ. That's a incorrect behavior, and
///   insecure, too (because it reveals some information about the past content). And,
/// - Formatting or serializing an instance after a modification could make (potentially
///   confidential) previous characters leak out!
///
/// Security and mutation: Properly implementing similar types is difficult. Otherwise they may leak
/// older data. (Mutation methods and related wiping out such data is not in our scope.)
///
/// Alternatively, we could derive all the above mentioned traits, if we wipe out any unused `rna`
/// slots after any modification.
///
/// Deriving [`Default`] makes the new instance valid, because it sets `len` to 0. However, this
/// works for [`MAX_NUM_RNA_NUCLEOTIDES`] being not more than 32. Otherwise we'd need to initialize
/// the array ourselves with [`core::array::from_fn`].
#[derive(Default)]
pub struct Rna {
    rna: [char; MAX_NUM_RNA_NUCLEOTIDES],
    len: usize,
}

impl<'a> Dna<'a> {
    pub fn new(dna: &'a str) -> OurResult<Self> {
        checks::check_dna(dna)?;
        Ok(Self(dna))
    }

    pub fn into_rna(&self) -> Rna {
        Rna::new_from_iter(self.0.chars().map(utils::dna_to_rna)).expect("RNA")
    }
}

impl Rna {
    /// Create a new [`Rna`] instance with given RNA nucleotides -[`Rna::GivenNucleotides`] variant.
    /// If `rna` is valid, return  
    /// [`Some(Rna)`](Some<Rna>) containing the new instance. On error return [`Err`] with a 0-based
    /// index of the first incorrect character.
    pub fn new<'a>(rna: &'a str) -> OurResult<Self> {
        Self::new_from_iter(rna.chars())
    }

    fn new_from_iter(rna_iter: impl Iterator<Item = char>) -> OurResult<Self> {
        let mut result = Rna::default();
        for c in rna_iter {
            result.rna[result.len] = c;
            result.len += 1;
        }
        checks::check_rna_chars(result.chars())?;
        Ok(result)
    }

    fn chars(&self) -> &[char] {
        &self.rna[..self.len]
    }
}

impl PartialEq for Rna {
    fn eq(&self, other: &Self) -> bool {
        self.chars() == other.chars()
    }
}
/// Not necessary, but valid.
impl Eq for Rna {}

impl Debug for Rna {
    /// Compared to [../../no_heap-slices-iterator]([../../no_heap-slices-iterator),
    /// [Self::DnaBased] variant here doesn't have `self.iter()`. So we map DNA to RNA chars here.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        // In `no_std` with heap we could have:
        #[cfg(feature = "with_heap")]
        {
            extern crate alloc;
            use alloc::string::String;
            write!(f, "RNA {{{:?}}}", self.chars().iter().collect::<String>())
        }
        // But to make this heapless-compatible, we iterate over characters instead:
        #[cfg(not(feature = "with_heap"))]
        {
            write!(f, "RNA {{")?;
            self.chars().iter().try_for_each(|&c| write!(f, "{}", c))?;
            write!(f, "}}")
        }
    }
}

impl Clone for Rna {
    fn clone(&self) -> Self {
        let mut rna = [char::default(); MAX_NUM_RNA_NUCLEOTIDES];
        for i in 0..self.len {
            rna[i] = self.rna[i];
        }
        Self { rna, len: self.len }
    }
}

#[cfg(test)]
pub mod test {
    //! Test(s) on top of Exercism's tests (which are in `../tests/`).

    // Unit tests of a `no_std` crate can't use `std` either. However, they can use heap (even if
    // the crate being tested doesn't have access to heap).
    extern crate alloc;
    use super::OurResult;
    use alloc::format;

    /// New to Rust? An empty pair of parenthesis `()` here is an empty/no value type, called a
    /// "unit type". Here it indicates content of [`Result::Ok`] returned on success.
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
