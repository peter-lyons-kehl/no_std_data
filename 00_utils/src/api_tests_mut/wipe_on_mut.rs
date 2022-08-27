//! Used by implementations *_wipe_on_mut.

extern crate alloc;

use crate::api_tests_mut::{RnaTraitMut, RnaTraitMutLeakStorage};
use crate::{DnaTrait, OurResult, RnaTrait};
use alloc::vec::Vec;

/// A marker trait.
pub trait RnaTraitMutWipeOnMut<'a>: RnaTraitMutLeakStorage<'a> {}

pub trait Tests {
    type Dna<'a>: DnaTrait<'a, Self::Rna<'a>>;
    type Rna<'a>: RnaTraitMutWipeOnMut<'a> + 'a;

    fn test_modify_string_based_rna_does_not_leak() {}
}
