//! no_std but with heap
#![no_std]
extern crate alloc;

use alloc::{borrow::ToOwned, string::String};
use core::fmt::Debug;

/// DNA (DNA nucleotide sequence).  
/// Implementing [`Eq`] is not necessary, but valid.
// See also "newtype" at https://doc.rust-lang.org/nightly/book/ch19-03-advanced-traits.html,
// https://doc.rust-lang.org/nightly/book/ch19-04-advanced-types.html and
// https://doc.rust-lang.org/nightly/rust-by-example/generics/new_types.html.
#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

/// RNA (RNA nucleotide sequence). If it was created based on DNA, all nucleotides have been
/// translated to RNA ones, and stored here. (That is different to all sister implementations.)
/// Implementing [`Eq`] is not necessary, but valid.
#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    /// Create a new instance with given DNA nucleotides. On error return [`Err`] with a 0-based
    /// index of the first incorrect character.
    pub fn new(dna: &str) -> Result<Self, usize> {
        match shared::check_dna(dna) {
            Ok(()) => Ok(Self(dna.to_owned())),
            Err(i) => Err(i),
        }
    }

    /// Create an [`Rna`] instance based on `self`. Transcript all nucleotides to RNA (and store
    /// them in the result [`Rna`] instance).
    pub fn into_rna(self) -> Rna {
        match self {
            Dna(dna) => {
                let rna_chars = dna.chars().map(shared::dna_to_rna).collect();
                Rna(rna_chars)
            }
        }
    }
}

impl Rna {
    /// Create a new instance with given RNA nucleotides. On error return [`Err`] with a 0-based
    /// index of the first incorrect character.
    pub fn new(rna: &str) -> Result<Self, usize> {
        match shared::check_rna(rna) {
            Ok(()) => Ok(Self(rna.to_owned())),
            Err(i) => Err(i),
        }
    }
}
