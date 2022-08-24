//! no_std heapless (bare metal/embedded-friendly)
#![no_std]

// @TODO Others: remove import of Debug - where it's derived only

use core::fmt::{self, Debug, Formatter};

const MAX_NUM_RNA_NUCLEOTIDES: usize = 14;

// @TODO Others: Derive/impl Clone.

/// DNA (DNA nucleotide sequence). `Dna` itself is `&str` slice-based. (Sufficient for our purpose.) Only `Rna` is array-based.
///
/// Implementing [`Eq`] is not necessary, but valid.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Dna<'a>(&'a str);

/// RNA (RNA nucleotide sequence). Storing RNA nucleotides.
///
/// We can't derive [`PartialEq`] or [`Debug`]. Why? Because an `Rna` instance may contain leftover
/// nucleotides.
///
/// Security: Properly implementing similar types is difficult. Otherwise they may leak older data.
/// (Wiping out such data is not in our scope.)
/// 
/// Deriving [`Default`] makes the new instance valid, because it sets `len` to 0.
#[derive(Default, Clone)]
pub struct Rna {
    rna: [char; MAX_NUM_RNA_NUCLEOTIDES],
    len: usize,
}

impl<'a> Dna<'a> {
    pub fn new(dna: &'a str) -> Result<Self, usize> {
        // @TODO in other projects: use ? op, and add a link
        shared::check_dna(dna)?;
        Ok(Self(dna))
    }

    pub fn into_rna(self) -> Rna {
        Rna::new_from_iter(self.0.chars().map(shared::dna_to_rna)).expect("RNA")
    }
}

impl Rna {
    /// Create a new [`Rna`] instance with given RNA nucleotides -[`Rna::GivenNucleotides`] variant.
    /// If `rna` is valid, return  
    /// [`Some(Rna)`](Some<Rna>) containing the new instance. On error return [`Err`] with a 0-based
    /// index of the first incorrect character.
    pub fn new<'a>(rna: &'a str) -> Result<Self, usize> {
        Self::new_from_iter(rna.chars())
    }

    fn new_from_iter(rna_iter: impl Iterator<Item = char>) -> Result<Self, usize> {
        let mut result = Rna::default();
        for c in rna_iter {
            result.rna[result.len] = c;
            result.len += 1;
        }
        shared::check_rna_chars(result.chars())?;
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
        write!(f, "RNA {{{:?}}}", self.chars())
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
            assert_eq!("RNA {['C', 'G', 'A', 'U']}", rna_dbg.as_str());
        });
    }
}
