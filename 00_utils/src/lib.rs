//! no_std heapless (bare metal/embedded-friendly) shared functionality
#![no_std]

const DNA_NUCLEOTIDES: &str = "GCTA";
const RNA_NUCLEOTIDES: &str = "CGAU";

/// Custom result type. It works with our Exercism exercise (the error variant uses `usize` to
/// indicate a 0-based character index that is not a valid DNA/RNA nucleotide). Type parameter `T`
/// is the success variant type, carrying a result as needed.
///
// New to Rust? Question mark operator shortcuts on error and returns it here.
pub type OurResult<T> = Result<T, usize>;

/// Result of [`check`] and related functions.
///
/// It doesn't need to be public (even if used in signatures of public functions here - because
/// those get exposed with the type resolved, without the alias).
type CheckResult = OurResult<()>;

/// Check that any characters from `chars_to_be_checked` are in `allowed_chars`. See [`check_dna`]
/// or [`check_rna_str`] for result type description.
fn check(to_be_checked: impl Iterator<Item = char>, allowed: &str) -> CheckResult {
    for (i, c) in to_be_checked.enumerate() {
        if !allowed.contains(c) {
            return Err(i);
        }
    }
    Ok(())
}

/// Check that `dna` contains DNA nucleotides only. On success return [`Ok(())`](Ok). On error
/// return [`Err`] with a 0-based index of the first incorrect character.
pub fn check_dna(dna: &str) -> CheckResult {
    check(dna.chars(), DNA_NUCLEOTIDES)
}

/// Check that `rna_iter` yields RNA nucleotides only. On success return [`Ok(())`](Ok). On error
/// return [`Err`] with a 0-based index of the first incorrect character.
pub fn check_rna_char_iter(rna_iter: impl Iterator<Item = char>) -> CheckResult {
    check(rna_iter, RNA_NUCLEOTIDES)
}

/// Check that `rna` contains RNA nucleotides only. On success return [`Ok(())`](Ok). On error
/// return [`Err`] with a 0-based index of the first incorrect character.
pub fn check_rna_str(rna: &str) -> CheckResult {
    check_rna_char_iter(rna.chars())
}

/// Check that `rna` contains RNA nucleotides only. On success return [`Ok(())`](Ok). On error
/// return [`Err`] with a 0-based index of the first incorrect character.
pub fn check_rna_chars(rna: &[char]) -> CheckResult {
    check_rna_char_iter(rna.iter().cloned())
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
    fn test_check_dna_rna_valid() {
        assert!(super::check_dna("GCTA").is_ok());
        assert!(super::check_rna_str("CGAU").is_ok());
    }

    #[test]
    fn test_check_dna_rna_invalid() {
        assert_eq!(super::check_dna("CU"), Err(1));
        assert_eq!(super::check_rna_str("CT"), Err(1));
    }

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

    #[test]
    fn test_check_dna() {
        assert!(super::check_dna("GCTA").is_ok());
        assert_eq!(super::check_dna("U"), Err(0));
        assert_eq!(super::check_dna("GX"), Err(1));
    }

    #[test]
    fn test_check_rna_iter() {
        assert!(super::check_rna_char_iter("CGAU".chars()).is_ok());
        assert_eq!(super::check_rna_char_iter("T".chars()), Err(0));
        assert_eq!(super::check_rna_char_iter("GX".chars()), Err(1));
    }
    #[test]
    fn test_check_rna_str() {
        assert!(super::check_rna_str("CGAU").is_ok());
        assert_eq!(super::check_rna_str("T"), Err(0));
        assert_eq!(super::check_rna_str("GX"), Err(1));
    }
    #[test]
    fn test_check_rna_chars() {
        assert!(super::check_rna_chars(&['C', 'G', 'A', 'U']).is_ok());
        assert_eq!(super::check_rna_chars(&['T']), Err(0));
        assert_eq!(super::check_rna_chars(&['G', 'X']), Err(1));
    }
}
