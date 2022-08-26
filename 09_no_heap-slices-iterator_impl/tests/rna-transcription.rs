#![no_std]
#![feature(generic_associated_types)]

use no_heap_slices_iterator_impl as dna;

use utils::api_tests_read_only::Tests;

struct T {}
impl Tests for T {
    type Dna<'a> = dna::Dna<'a>;
    type Rna<'a> = dna::Rna<'a>;
}

#[test]
fn all_tests() {
    T::all_tests();
}
