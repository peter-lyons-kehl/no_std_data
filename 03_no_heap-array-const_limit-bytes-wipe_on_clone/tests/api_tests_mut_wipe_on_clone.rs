#![no_std]
#![feature(generic_associated_types)]

use no_heap_array_const_limit_bytes_wipe_on_clone as dna;

use utils::api_tests_mut::wipe_on_leave::Leave;
use utils::api_tests_mut::wipe_on_leave::Tests;
use utils::api_tests_mut::{WithStorageLeaked, WithStorageLeakedCallBack};

struct T {}
impl Tests for T {
    type Dna<'a> = dna::Dna<'a>;
    type Rna<'a> = dna::Rna;
}

fn leave(rna: dna::Rna) -> dna::Rna {
    rna.clone()
}
type TLeave<'a> = Leave<'a, dna::Rna>;
const CHECK_LEAVE_FUNCTION_SIGNATURE: TLeave = &leave;
fn _checkLeaveFunctionSignature() {
    //@TODO remove
    let leave: TLeave = &leave;
}

fn with_storage_leaked<RES>(
    rna: &dna::Rna,
    with_storage_leaked_call_back: WithStorageLeakedCallBack<bool>,
) -> bool {
    todo!()
    /*
    let bytes = &rna.rna[..];
    let mut bytes_iter = bytes.into_iter().cloned();
    with_storage_leaked_call_back(&mut bytes_iter)
    */
}
type TWithStorageLeaked<'a> = WithStorageLeaked<'a, dna::Rna, bool>;
const CHECK_WITH_STORAGE_LEAKED: TWithStorageLeaked = &with_storage_leaked::<bool>;

#[test]
fn all_tests() {
    T::test_modify_string_based_rna_direct_without_leave_does_leak(&with_storage_leaked::<bool>);

    T::test_modify_string_based_rna_leave_does_not_leak(&leave, &with_storage_leaked::<bool>);
}
