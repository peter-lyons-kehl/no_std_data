//! Used by implementations *_wipe_on_mut.

extern crate alloc;

use utils::api_tests_mut::RnaTraitMutLeakStorage;
use crate::api_tests_mut::{self, WithStorageLeaked};
use utils::DnaTrait;

pub trait Tests {
    type Dna<'a>: DnaTrait<'a, Self::Rna<'a>>;
    type Rna<'a>: RnaTraitMutLeakStorage<'a> + 'a;

    fn test_modify_string_based_rna_mutation_does_not_leak<'a>(
        with_storage_leaked: WithStorageLeaked<'a, Self::Rna<'a>, bool>,
    ) {
        let rna = api_tests_mut::cga_modified_to_u::<Self::Rna<'_>>();

        let leaks = api_tests_mut::leaks_g_or_a(&rna, with_storage_leaked);

        assert!(!leaks);
    }
}
