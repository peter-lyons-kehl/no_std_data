//! no_std but with heap
#![no_std]
extern crate alloc;

use alloc::{borrow::ToOwned, string::String};
use core::fmt::Debug;
use utils::{checks, DnaTrait, OurResult, RnaTrait};

/// DNA (DNA nucleotide sequence).  
///
/// Implementing [`Eq`] or [`Clone`] is not necessary, but valid/helpful. However, derive [`Copy`]
/// trait only if the type is unlikely to have non-Copy fields added later. Or if all its consumers
/// are under your control.
///
/// We could implement [`core::hash::Hash`], too. But that's much less useful, since `no_std`
/// doesn't have standard `HashSet/HashMap`. TODO reconsider with no_std Hash crate(s).
///
// See also "newtype" at https://doc.rust-lang.org/nightly/book/ch19-03-advanced-traits.html,
// https://doc.rust-lang.org/nightly/book/ch19-04-advanced-types.html and
// https://doc.rust-lang.org/nightly/rust-by-example/generics/new_types.html.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Dna(String);

/// RNA (RNA nucleotide sequence).
///
/// If it was created based on DNA, all nucleotides have been translated to RNA ones, and stored
/// here. (That is different to all other implementations in neighbor crates.)
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rna(String);

impl DnaTrait<Rna> for Dna {
    /// Create a new [`Dna`] instance with given DNA nucleotides. If `dna` is valid, return  
    /// [`Some(Dna)`](Some<Dna>) containing the new instance. On error return [`Err`] with a 0-based
    /// index of the first incorrect character.
    fn new(dna: &str) -> OurResult<Self> {
        checks::check_dna(dna)?;
        Ok(Self(dna.to_owned()))
    }

    /// Create an [`Rna`] instance based on `self`. Transcript all nucleotides to RNA (and store
    /// them in the result [`Rna`] instance).
    fn into_rna(&self) -> Rna {
        match self {
            Dna(dna) => {
                let rna_chars = dna.chars().map(utils::dna_to_rna).collect();
                Rna(rna_chars)
            }
        }
    }
}

impl RnaTrait for Rna {
    /// Create a new [`Rna`] instance with given RNA nucleotides. If `rna` is valid, return  
    /// [`Some(Rna)`](Some<Rna>) containing the new instance. On error return [`Err`] with a 0-based
    /// index of the first incorrect character.
    fn new(rna: &str) -> OurResult<Self> {
        checks::check_rna_str(rna)?;
        Ok(Self(rna.to_owned()))
    }
}
