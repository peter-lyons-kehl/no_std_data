//! no_std heapless (bare metal/embedded-friendly) shared functionality
#![no_std]
// Needed for api_tests::Tests
#![feature(generic_associated_types)]

use core::fmt::Debug;

pub mod api_tests_read_only;
pub mod checks;

/// Custom result type. It works with our Exercism exercise (the error variant uses `usize` to
/// indicate a 0-based character index that is not a valid DNA/RNA nucleotide). Type parameter `T`
/// is the success variant type, carrying a result as needed.
///
// New to Rust? Question mark operator shortcuts on error and returns it here.
pub type OurResult<T> = Result<T, usize>;

pub trait DnaTrait<'a, Rna>: Sized + PartialEq + Eq + Debug
where
    Rna: RnaTrait<'a> + 'a,
{
    fn new(dna: &'a str) -> OurResult<Self>;
    fn into_rna(&self) -> Rna;
}

pub trait RnaTrait<'a>: Sized + PartialEq + Eq + Debug {
    fn new(rna: &'a str) -> OurResult<Self>;
}

/// Translate DNA nucleotide `dna_nucl` to a RNA nucleaotide. [`panic`] if `dna_nucl` is invalid.
pub fn dna_to_rna(dna_nucl: char) -> char {
    match dna_nucl {
        'G' => 'C',
        'C' => 'G',
        'T' => 'A',
        'A' => 'U',
        _ => panic!("Unrecognized nucleotide {dna_nucl}."),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_dna_to_rna() {
        assert_eq!(super::dna_to_rna('G'), 'C');
        assert_eq!(super::dna_to_rna('C'), 'G');
        assert_eq!(super::dna_to_rna('T'), 'A');
        assert_eq!(super::dna_to_rna('A'), 'U');
    }

    #[test]
    #[should_panic]
    fn test_dna_to_rna_panic_invalid() {
        super::dna_to_rna('U');
    }
}
