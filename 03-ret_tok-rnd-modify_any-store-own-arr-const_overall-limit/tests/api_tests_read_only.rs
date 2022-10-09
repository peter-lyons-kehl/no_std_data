#![no_std]
#![feature(generic_associated_types)]

use ret_tok_rnd_modify_any_store_own_arr_const_overall_limit as dna;

use test_harness::api_tests_read_only::Tests;

struct T {}
impl Tests for T {
    type Dna<'a> = dna::Dna<'a>;
    type Rna<'a> = dna::Rna;
}

#[test]
fn all_tests() {
    T::all_tests();
}
