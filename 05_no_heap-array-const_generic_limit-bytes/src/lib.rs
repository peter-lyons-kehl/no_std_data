//! no_std heapless (bare metal/embedded-friendly)
#![no_std]
#![feature(array_try_from_fn)]

use core::fmt::{self, Debug, Formatter};
use core::str;

const DEFAULT_MAX_NUCLEOTIDES: usize = 12;

/// DNA (DNA nucleotide sequence).
///
/// `const N` parameter does not affect storage of this type. It's used only to infer respective
/// ['Rna`] size when calling [`Dna::into_rna`].
///
/// We can't derive [`PartialEq`]. Why? Because we want to compare [`Rna`] types regardless of `M`.
#[derive(Debug, Clone)]
pub struct DnaImpl<'a, const M: usize = DEFAULT_MAX_NUCLEOTIDES>(&'a str);

pub type Dna<'a> = DnaImpl<'a, DEFAULT_MAX_NUCLEOTIDES>;

/// RNA (RNA nucleotide sequence).
///
/// Usable only if the required `const N` parameter is known in compile time. Can't derive Default -
/// it's defined for arrays only up to a certain size.
///
/// We can't derive [`PartialEq`] or [`Debug`]. Why? Because an `Rna` instance may contain leftover
/// nucleotides - insecure! Also, we want to compare [`Rna`] types regardless of `M`.
#[derive(Clone)]
pub struct RnaImpl<const M: usize = DEFAULT_MAX_NUCLEOTIDES> {
    rna: [u8; M],
    len: usize,
}

pub type Rna = RnaImpl<DEFAULT_MAX_NUCLEOTIDES>;

impl<'a, const M: usize> DnaImpl<'a, M> {
    /// Create a new [`Dna`] instance with given DNA nucleotides. If `dna` is valid, return  
    /// [`Some(Dna)`](Some<Dna>) containing the new instance. On error return [`Err`] with a 0-based
    /// index of the first incorrect character.
    pub fn new(dna: &'a str) -> utils::Result<Self> {
        utils::check_dna(dna)?;
        Ok(Self(dna))
    }

    /// Create an [`Rna`] instance, based on `self`. The returned instance contains the translated
    /// nucleotides. (The result doesn't depend on the original [`Dna`] instance's lifetime). TODO
    /// add similar doc to `ok_heap_string`.
    pub fn into_rna(self) -> RnaImpl<M> {
        RnaImpl::new_from_iter(self.0.chars().map(utils::dna_to_rna)).expect("RNA sequence")
    }
}

impl<const M: usize> RnaImpl<M> {
    pub fn new(rna: &str) -> utils::Result<Self> {
        Self::new_from_iter(rna.chars())
    }
    pub fn new_from_iter(mut rna_iter: impl Iterator<Item = char>) -> utils::Result<Self> {
        let mut len = 0usize;
        let rna = core::array::from_fn(|_| {
            if let Some(c) = rna_iter.next() {
                len += 1;
                c as u8
            } else {
                0 // extra slots - not used by current data
            }
        });
        if rna_iter.next().is_some() {
            // Extra characters left.
            return Err(len);
        }
        // Only check the valid items: `0..len`. Hence `Iterator::take`.
        utils::check_rna_char_iter(rna.iter().take(len).map(|&b| b as char))?;
        Ok(Self { rna, len })
    }

    fn bytes(&self) -> &[u8] {
        &self.rna[..self.len]
    }
}

impl<'a, const L: usize, const R: usize> PartialEq<DnaImpl<'_, R>> for DnaImpl<'a, L> {
    fn eq(&self, other: &DnaImpl<'_, R>) -> bool {
        self.0 == other.0
    }
}
impl<'a, const M: usize> Eq for DnaImpl<'a, M> {}

impl<const L: usize, const R: usize> PartialEq<RnaImpl<R>> for RnaImpl<L> {
    fn eq(&self, other: &RnaImpl<R>) -> bool {
        self.bytes() == other.bytes()
    }
}
/// Not necessary, but valid.
impl Eq for RnaImpl {}

impl<const N: usize> Debug for RnaImpl<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "RNA {{{}}}",
            str::from_utf8(self.bytes()).expect("UTF-8 encoded string")
        )
    }
}

#[cfg(test)]
pub mod test {
    //! Test(s) on top of Exercism's tests (which are in `../tests/`).

    // Unit tests of a `no_std` crate can't use `std` either. However, they can use heap (even if
    // the crate being tested doesn't have access to heap).
    extern crate alloc;
    use alloc::format;

    // @TODO: add & rename - in othre crates
    #[test]
    fn test_rna_given_nucleotides_debug() {
        let rna = super::Rna::new("CGAU").expect("Rna");
        let rna_dbg = format!("{:?}", rna);
        assert_eq!("RNA {CGAU}", rna_dbg.as_str());
    }

    #[test]
    fn test_dna_into_rna_debug() {
        let dna = <super::DnaImpl<20>>::new("GCTA").expect("Dna");
        let rna = dna.into_rna();
        let rna_dbg = format!("{:?}", rna);
        assert_eq!("RNA {CGAU}", rna_dbg.as_str());
    }
}
