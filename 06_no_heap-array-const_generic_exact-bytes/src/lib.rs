//! no_std heapless (bare metal/embedded-friendly)
#![no_std]

#[cfg(test)]
use test_harness;
use utils::{checks, DnaTrait, OurResult, RnaTrait};

/// Fixed length.
///
/// `const N` parameter does not affect storage of this type. It's used only to infer respective
/// ['Rna`] size when calling [`Dna::into_rna`].
///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Dna<'a, const N: usize>(&'a str);

/// RNA (RNA nucleotide sequence).
///
/// Usable only if the required `const N` parameter is known in compile time. Can't derive Default -
/// it's defined for arrays with only up to 32 items.
///
/// [`Rna`] in this implementation derives all its traits. It never has any leaking data - it always
/// uses all its array items. (As a consequence, if we added any mutation methods, those could only
/// replace data, but never "remove/shorten").
///
/// Only instances of types parameterized with same const generic `N` are comparable. Even if we
/// implemented PartialEq ourselves, it wouldn't make sense to implement it across various lengths.
///
/// On the contrary, see
/// [05_no_heap-array-const_generic_limit-bytes](../../05_no_heap-array-const_generic_limit-bytes/src/lib.rs)
/// which does compare instances of types parameterized even with different const generic `M`.
/// However, there `M` is not the actual length, but the maximum length.
///
/// This is not Unicode-friendly. For that we'd need to implement [`PartialEq`] ourselves, and
/// Unicode-proof [`Rna::new_from_iter`].
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Rna<const N: usize>([u8; N]);

impl<'a, const N: usize> DnaTrait<'a, Rna<N>> for Dna<'a, N> {
    /// Create a new [`Dna`] instance with given DNA nucleotides. If `dna` is valid, return  
    /// [`Some(Dna)`](Some<Dna>) containing the new instance. On error return [`Err`] with a 0-based
    /// index of the first incorrect character.
    fn new(dna: &'a str) -> OurResult<Self> {
        checks::check_dna(dna)?;
        Ok(Self(dna))
    }

    /// Create an [`Rna`] instance, based on `self`. The returned instance contains the translated
    /// nucleotides. (The result doesn't depend on the original [`Dna`] instance's lifetime). TODO
    /// add similar doc to `ok_heap_string`.
    fn into_rna(&self) -> Rna<N> {
        Rna::new_from_iter(self.0.chars().map(utils::dna_to_rna)).expect("RNA sequence")
    }
}

impl<'a, const N: usize> RnaTrait<'a> for Rna<N> {
    fn new(rna: &str) -> OurResult<Self> {
        Self::new_from_iter(rna.chars())
    }
}

impl<const N: usize> Rna<N> {
    fn new_from_iter(mut rna_iter: impl Iterator<Item = char>) -> OurResult<Self> {
        //let mut result = Self(core::array::from_fn(|_| Default::default()));
        // Can't `result.0.copy_from_slice(rna)` - because `result.0` is `&[char]`.
        let result = Self(core::array::from_fn(|_| {
            rna_iter.next().expect("nucleotide") as u8
        }));
        assert!(
            rna_iter.next().is_none(),
            "Not enough space, or too long RNA source."
        );
        checks::check_rna_char_iter(result.0.iter().map(|&b| b as char))?;
        Ok(result)
    }
}
