// We don't need to have `no_std` here, but we can.
#![no_std]
// Needed for implementing api_tests::Tests
#![feature(generic_associated_types)]

use ret_tok_rnd_modify_any_store_own_vec_string as dna;
use test_harness::api_tests_read_only::Tests;

struct T {}
impl Tests for T {
    type Dna<'a> = dna::Dna;
    type Rna<'a> = dna::Rna;
}

#[test]
fn all_tests() {
    T::all_tests();
}
