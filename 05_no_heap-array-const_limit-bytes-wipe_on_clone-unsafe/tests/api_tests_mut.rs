#![no_std]
#![feature(generic_associated_types)]

use no_heap_array_const_limit_bytes_wipe_on_clone_unsafe as dna;

use test_harness::api_tests_mut::Tests;
use utils::OurResult;

struct T {}
impl Tests for T {
    type Dna<'a> = dna::Dna<'a>;
    type Rna<'a> = dna::Rna;
}

#[test]
fn all_tests() -> OurResult<()> {
    T::all_tests()?;
    Ok(())
}
