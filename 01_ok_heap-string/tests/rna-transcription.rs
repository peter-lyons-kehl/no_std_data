// We don't need to have `no_std` here, but we can.
#![no_std]
use ok_heap_string as dna;
use utils::api_tests::Tests;

struct T {}
impl Tests for T {
    type Dna = dna::Dna;
    type Rna = dna::Rna;
}

#[test]
fn all_tests() {
    T::all_tests();
}
