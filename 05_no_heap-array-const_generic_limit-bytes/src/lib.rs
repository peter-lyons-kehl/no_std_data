//! no_std heapless (bare metal/embedded-friendly)
#![no_std]
// Needed as of August 2022, only for RnaImpl::clone_max_size.
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use core::fmt::{self, Debug, Formatter};
use core::str;
use utils::OurResult;

const DEFAULT_MAX_NUCLEOTIDES: usize = 12;

/// DNA (DNA nucleotide sequence).
///
/// `const N` parameter does not affect storage of this type. It's used only to infer respective
/// ['Rna`] size when calling [`Dna::into_rna`].
///
/// We don't derive [`PartialEq`]. Why? Because we want to compare [`Dna`] types regardless of `M`.
/// (And for security & correctness, if we added mutation methods.)
#[derive(Debug, Clone, Copy)]
pub struct DnaImpl<'a, const M: usize = DEFAULT_MAX_NUCLEOTIDES>(&'a str);

pub type Dna<'a> = DnaImpl<'a, DEFAULT_MAX_NUCLEOTIDES>;

/// RNA (RNA nucleotide sequence).
///
/// Usable only if the required `const N` parameter is known in compile time. Can't derive Default -
/// it's defined for arrays only up to a certain size.
///
/// We don't derive [`PartialEq`] or [`Debug`] or [`Clone`] or [`Copy`] (neither Serde's
/// `Serialize`, if we used it). See
/// [02_no_heap-array-const_limit-chars](../../02_no_heap-array-const_limit-chars/src/lib.rs) for
/// notes on security.
///
/// We don't derive [`PartialEq`] for the same reason as in [`DnaImpl`].
pub struct RnaImpl<const M: usize = DEFAULT_MAX_NUCLEOTIDES> {
    rna: [u8; M],
    len: usize,
}

pub type Rna = RnaImpl<DEFAULT_MAX_NUCLEOTIDES>;

impl<'a, const M: usize> DnaImpl<'a, M> {
    /// Create a new [`Dna`] instance with given DNA nucleotides. If `dna` is valid, return  
    /// [`Some(Dna)`](Some<Dna>) containing the new instance. On error return [`Err`] with a 0-based
    /// index of the first incorrect character.
    pub fn new(dna: &'a str) -> OurResult<Self> {
        utils::check_dna(dna)?;
        Ok(Self(dna))
    }

    /// Create an [`Rna`] instance, based on `self`. The returned instance contains the translated
    /// nucleotides. (The result doesn't depend on the original [`Dna`] instance's lifetime). TODO
    /// add similar doc to `ok_heap_string`.
    pub fn into_rna(&self) -> RnaImpl<M> {
        RnaImpl::new_from_iter(self.0.chars().map(utils::dna_to_rna)).expect("RNA sequence")
    }
}

impl<const M: usize> RnaImpl<M> {
    pub fn new(rna: &str) -> OurResult<Self> {
        Self::new_from_iter(rna.chars())
    }
    pub fn new_from_iter(mut rna_iter: impl Iterator<Item = char>) -> OurResult<Self> {
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
        let result = Self { rna, len };
        // Only check the valid items: `0..len`. Hence `Iterator::take`.
        utils::check_rna_str(result.as_str())?;
        Ok(result)
    }

    fn as_str(&self) -> &str {
        str::from_utf8(&self.rna[..self.len]).expect("UTF-8 encoded string of RNA nucleotides")
    }

    pub fn clone_max_size<const N: usize>(&self) -> RnaImpl<N> {
        assert!(self.len <= N, "Calling clone_max_size on an instance with len={}, but the target maximum size is insufficient: {}.", self.len, N);
        let mut rna = [u8::default(); N];
        for i in 0..self.len {
            rna[i] = self.rna[i];
        }
        RnaImpl { rna, len: self.len }
    }
}

impl<const M: usize> Clone for RnaImpl<M> {
    fn clone(&self) -> Self {
        self.clone_max_size::<M>()
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
        self.as_str() == other.as_str()
    }
}
/// Not necessary, but valid.
impl Eq for RnaImpl {}

impl<const M: usize> Debug for RnaImpl<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "RNA {{{}}}", self.as_str())
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

    #[test]
    fn test_rna_given_nucleotides_debug() -> OurResult<()> {
        let rna = super::Rna::new("CGAU")?;
        let rna_dbg = format!("{:?}", rna);
        assert_eq!("RNA {CGAU}", rna_dbg.as_str());
        Ok(())
    }

    #[test]
    fn test_rna_from_dna_debug() -> OurResult<()> {
        let dna = <super::DnaImpl<20>>::new("GCTA")?;
        let rna = dna.into_rna();
        let rna_dbg = format!("{:?}", rna);
        assert_eq!("RNA {CGAU}", rna_dbg.as_str());
        Ok(())
    }
}
