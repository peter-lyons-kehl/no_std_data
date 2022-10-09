///! Security unit test. This can't be an integration test (under ../tests/), because it needs
///! private access to [`dna::Rna::rna`].
use crate as dna;

use test_harness::api_tests_mut::wipe_on_mut::Tests;
use utils::api_tests_mut::{WithStorageLeaked, WithStorageLeakedCallBack};

struct T {}
impl Tests for T {
    type Dna<'a> = dna::Dna<'a>;
    type Rna<'a> = dna::Rna;
}

fn with_storage_leaked<RES>(
    rna: &dna::Rna,
    with_storage_leaked_call_back: WithStorageLeakedCallBack<bool>,
) -> bool {
    let bytes = &rna.rna[..];
    let mut bytes_iter = bytes.into_iter().cloned();
    with_storage_leaked_call_back(&mut bytes_iter)
}
type _TWithStorageLeaked<'a> = WithStorageLeaked<'a, dna::Rna, bool>;
const _CHECK_WITH_STORAGE_LEAKED_FUNCTION_SIGNATURE: _TWithStorageLeaked =
    &with_storage_leaked::<bool>;

#[test]
fn all_tests() {
    T::test_modify_string_based_rna_mutation_does_not_leak(&with_storage_leaked::<bool>);
}
