//! Used by implementations *_wipe_on_mut.

// /use core::fmt::Debug;
use crate::{api_tests_mut::RnaTraitMutLeakStorage, DnaTrait};

/// A marker trait.
pub trait RnaTraitMutWipeOnMut<'a>: RnaTraitMutLeakStorage<'a> {}

pub trait Tests {
    type Dna<'a>: DnaTrait<'a, Self::Rna<'a>>;
    type Rna<'a>: RnaTraitMutWipeOnMut<'a> + 'a;

    fn test_valid_self_input() {}

    fn all_tests() {}
}
