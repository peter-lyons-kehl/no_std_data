#![no_std]
#![feature(generic_associated_types)]

use ok_heap_slices_box_dyn_trait_map as dna;

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
