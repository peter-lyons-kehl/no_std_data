//! Used by implementations *_wipe_on_mut.

extern crate alloc;

use crate::api_tests_mut::{RnaTraitMut, RnaTraitMutLeakStorage, WithStorageLeaked};
use crate::{DnaTrait, OurResult, RnaTrait};
use alloc::vec::Vec;

/// A marker trait. See [`Tests`].
pub trait RnaTraitMutWipeOnMut<'a>: RnaTraitMutLeakStorage<'a> {}

pub trait Tests {
    type Dna<'a>: DnaTrait<'a, Self::Rna<'a>>;
    type Rna<'a>: RnaTraitMutWipeOnMut<'a> + 'a;

    fn test_modify_string_based_rna_mutation_does_not_leak<'a>(
        with_storage_leaked: WithStorageLeaked<'a, Self::Rna<'a>, bool>,
    ) {
        //@TODO
    }
}
