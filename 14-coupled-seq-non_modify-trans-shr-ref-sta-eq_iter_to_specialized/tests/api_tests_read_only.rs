#![no_std]
#![feature(generic_associated_types)]

use coupled_seq_non_modify_trans_shr_ref_sta_eq_iter_to_specialized as dna;
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
