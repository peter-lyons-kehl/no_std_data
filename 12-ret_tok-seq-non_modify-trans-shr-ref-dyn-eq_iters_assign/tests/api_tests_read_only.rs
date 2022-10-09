#![no_std]
#![feature(generic_associated_types)]

use ret_tok_seq_non_modify_trans_shr_ref_dyn_eq_iters_assign as dna;

use test_harness::api_tests_read_only::Tests;

struct T {}
impl Tests for T {
    type Dna<'a> = dna::Dna<'a>;
    type Rna<'a> = dna::Rna<'a>;
}

#[test]
fn all_tests() {
    T::all_tests();
}
