use core::fmt::Debug;

pub mod api_tests_mut;
pub mod checks;

pub trait DnaTrait<'a, Rna>: Sized + PartialEq + Eq + Debug
where
    Rna: RnaTrait<'a> + 'a,
{
    fn new(dna: &'a str) -> OurResult<Self>;
    /// The common practice is that `into_*` methods consume `self` (rather than taking it by
    /// reference). However, it's OK to take by reference where possible - making the API more flexible.
    #[allow(clippy::wrong_self_convention)]
    fn into_rna(&self) -> Rna;
}

pub trait RnaTrait<'a>: Sized + PartialEq + Eq + Debug {
    fn new(rna: &'a str) -> OurResult<Self>;
}

pub trait RnaTraitMut<'a>: RnaTrait<'a> {
    /// Mutate `self`: Make it store all characters in the given `iter`. Fail if `iter` doesn't
    /// satisfy requirements particular of the given implementation.
    fn set_from_iter(&mut self, iter: &mut dyn Iterator<Item = char>) -> OurResult<()>;
}

/// Custom result type. It works with our Exercism exercise (the error variant uses `usize` to
/// indicate a 0-based character index that is not a valid DNA/RNA nucleotide). Type parameter `T`
/// is the success variant type, carrying a result as needed.
///
// New to Rust? Question mark operator shortcuts on error and returns it here.
pub type OurResult<T> = Result<T, usize>;

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

/// Iterate over `rna_iter` until its end. Transform its characters to UTF-8 and store them in `result`.
/// Return number (length) of copied UTF-8 bytes. Panic if `result` doesn't have enough space.
pub fn char_iter_to_bytes(result: &mut [u8], rna_iter: impl Iterator<Item = char>) -> usize {
    let mut char_to_utf8 = [0u8; 4];
    let mut result_idx = 0usize;
    for c in rna_iter {
        let utf8 = c.encode_utf8(&mut char_to_utf8[..]);
        // Prefer not the following two lines due to the function call overhead.
        // result[result_idx..result_idx + utf8.len()].copy_from_slice(&utf8.as_bytes()[..utf8.len()]);
        // result_idx += utf8.len();
        //
        // Ignoring clippy because of 1-4 items.
        #[allow(clippy::needless_range_loop)]
        for i in 0..utf8.len() {
            result[result_idx] = char_to_utf8[i];
            result_idx += 1;
        }
    }
    result_idx
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

    // @TODO test for char_iter_to_bytes()
}
