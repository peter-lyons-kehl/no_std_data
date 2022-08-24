//! no_std heapless (bare metal/embedded-friendly)
#![no_std]

/// DNA (DNA nucleotide sequence).
///
/// `const N` parameter does not affect storage of this type. It's used only to infer respective
/// ['Rna`] size when calling [`Dna::into_rna`].
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Dna<'a, const N: usize>(&'a str);

/// RNA (RNA nucleotide sequence).
///
/// Usable only if the required `const N` parameter is known in compile time. Can't derive Default -
/// it's defined for arrays only up to a certain size.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rna<const N: usize>([u8; N]);

impl<'a, const N: usize> Dna<'a, N> {
    /// Create a new [`Dna`] instance with given DNA nucleotides. If `dna` is valid, return  
    /// [`Some(Dna)`](Some<Dna>) containing the new instance. On error return [`Err`] with a 0-based
    /// index of the first incorrect character.
    pub fn new(dna: &'a str) -> utils::Result<Self> {
        utils::check_dna(dna)?;
        Ok(Self(dna))
    }

    /// Create an [`Rna`] instance, based on `self`. The returned instance contains the translated
    /// nucleotides. (The result doesn't depend on the original [`Dna`] instance's lifetime).
    /// TODO add similar doc to `ok_heap_string`.
    pub fn into_rna(self) -> Rna<N> {
        Rna::new_from_iter(self.0.chars().map(utils::dna_to_rna)).expect("RNA sequence")
    }
}

impl<const N: usize> Rna<N> {
    pub fn new(rna: &str) -> utils::Result<Self> {
        Self::new_from_iter(rna.chars())
    }
    pub fn new_from_iter(mut rna_iter: impl Iterator<Item = char>) -> utils::Result<Self> {
        //let mut result = Self(core::array::from_fn(|_| Default::default()));
        // Can't `result.0.copy_from_slice(rna)` - because `result.0` is `&[char]`.
        let result = Self(core::array::from_fn(|_| {
            rna_iter.next().expect("nucleotide") as u8
        }));
        assert!(
            rna_iter.next().is_none(),
            "Not enough space, or too long RNA source."
        );
        utils::check_rna_char_iter(result.0.iter().map(|&b| b as char))?;
        Ok(result)
    }
}
