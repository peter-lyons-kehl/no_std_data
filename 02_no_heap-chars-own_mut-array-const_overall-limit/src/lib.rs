//! no_std and heapless (bare metal/embedded-friendly)
#![no_std]

use core::array;
use core::fmt::{self, Debug, Formatter};
use utils::{checks, DnaTrait, OurResult, RnaTrait, RnaTraitMut};

/// This is higher than `32`, so that we make sure to implement [`Default`] ourselves. ([`Default`]
/// can be derived for arrays only up to size `32`.)
const MAX_NUM_RNA_NUCLEOTIDES: usize = 40;

// @TODO Others: Derive/impl Clone.

/// DNA (DNA nucleotide sequence). `Dna` itself is `&str` slice-based. (Sufficient for our purpose.)
/// Only `Rna` is array-based.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Dna<'a>(&'a str);

/// RNA (RNA nucleotide sequence). Storing RNA nucleotides.
#[derive(PartialEq, Clone, Copy)]
pub struct Rna {
    rna: [char; MAX_NUM_RNA_NUCLEOTIDES],
    len: usize,
}

impl<'a> DnaTrait<'a, Rna> for Dna<'a> {
    fn new(dna: &'a str) -> OurResult<Self> {
        checks::check_dna(dna)?;
        Ok(Self(dna))
    }

    fn into_rna(&self) -> Rna {
        Rna::new_from_iter(self.0.chars().map(utils::dna_to_rna)).expect("RNA")
    }
}

impl<'a> RnaTrait<'a> for Rna {
    /// Create a new [`Rna`] instance with given RNA nucleotides -[`Rna::GivenNucleotides`] variant.
    /// If `rna` is valid, return  
    /// [`Some(Rna)`](Some<Rna>) containing the new instance. On error return [`Err`] with a 0-based
    /// index of the first incorrect character.
    fn new(rna: &'a str) -> OurResult<Self> {
        Self::new_from_iter(rna.chars())
    }
}
impl Rna {
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

impl Default for Rna {
    fn default() -> Self {
        Self {
            rna: [char::default(); MAX_NUM_RNA_NUCLEOTIDES],
            len: 0,
        }
    }
}

// @TODO RnaTraitMut

/// Not necessary, but valid.
impl Eq for Rna {}

impl Debug for Rna {
    /// Compared to [../../no_heap-slices-iterator]([../../no_heap-slices-iterator),
    /// [Self::DnaBased] variant here doesn't have `self.iter()`. So we map DNA to RNA chars here.
    /// Honoring default derived format of a newtype-based implementation, so we can re-use same tests.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        // In `no_std` with heap we could have:
        #[cfg(feature = "with_heap")]
        {
            extern crate alloc;
            use alloc::string::String;
            write!(f, "RNA({})", self.chars().iter().collect::<String>())
        }
        // But to make this heapless-compatible, we iterate over characters instead:
        #[cfg(not(feature = "with_heap"))]
        {
            write!(f, "Rna(\"")?;
            self.chars().iter().try_for_each(|&c| write!(f, "{}", c))?;
            write!(f, "\")")
        }
    }
}
