//! no_std heapless (bare metal/embedded-friendly)
#![no_std]
// Generic_associated_types are not required for the implementation itself, but only for
// `api_tests_mut_wipe_on_clone` unit tests.
// #![cfg_attr(test, feature = "generic_associated_types" )]
#![cfg_attr(test, feature(generic_associated_types))]

use core::fmt::{self, Debug, Formatter};
use core::{slice, str};
use utils::api_tests_mut::RnaTraitMutLeakStorage;
use utils::{checks, DnaTrait, OurResult, RnaTrait, RnaTraitMut};

#[cfg(test)]
mod api_tests_mut_wipe_on_mut;

const MAX_NUM_RNA_NUCLEOTIDES: usize = 40;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Dna<'a>(&'a str);

#[derive(Clone, Copy)]
pub struct Rna {
    rna: [u8; MAX_NUM_RNA_NUCLEOTIDES],
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
    /// Create a new [`Rna`] instance with given RNA nucleotides. If `rna` is valid, return  
    /// [`Some(Rna)`](Some<Rna>) containing the new instance. On error return [`Err`] with a 0-based
    /// index of the first incorrect character.
    fn new(rna: &'a str) -> OurResult<Self> {
        Self::new_from_iter(rna.chars())
    }
}

impl Rna {
    /// We purge any extra leftover data.
    fn set_from_iter_impl(&mut self, rna_iter: impl Iterator<Item = char>) -> OurResult<()> {
        let previous_len = self.len;

        self.len = utils::char_iter_to_bytes(&mut self.rna, rna_iter);
        for i in self.len..previous_len {
            self.rna[i] = u8::default();
        }

        // Here we must not use self.as_str() yet, The following call to str::from_utf8() verifies
        // that the bytes are a valid UTF-8 slice. Only then self.as_str() is safe.
        let slice =
            str::from_utf8(&self.rna[..self.len]).expect("UTF-8 encoded string of RNA nucleotides");
        checks::check_rna_str(slice)?;
        Ok(())
    }
    fn new_from_iter(rna_iter: impl Iterator<Item = char>) -> OurResult<Self> {
        let mut result = Rna::default();
        result.set_from_iter_impl(rna_iter)?;
        Ok(result)
    }

    fn as_str(&self) -> &str {
        unsafe {
            let u8_slice = slice::from_raw_parts(&self.rna as *const u8, self.len);
            str::from_utf8_unchecked(u8_slice)
        }
    }
}

impl<'a> RnaTraitMut<'a> for Rna {
    fn set_from_iter(&mut self, iter: &mut dyn Iterator<Item = char>) -> OurResult<()> {
        self.set_from_iter_impl(iter)
    }
}

impl<'a> RnaTraitMutLeakStorage<'a> for Rna {}

impl Default for Rna {
    fn default() -> Self {
        Self {
            rna: [0; MAX_NUM_RNA_NUCLEOTIDES],
            len: 0,
        }
    }
}

impl PartialEq for Rna {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for Rna {}

impl Debug for Rna {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Rna(\"{}\")", self.as_str())
    }
}
