//! no_std heapless (bare metal/embedded-friendly) shared functionality
#![no_std]

const DNA_NUCLEOTIDES: &str = "GCTA";
const RNA_NUCLEOTIDES: &str = "CGAU";

/// Result of [`check`] and related functions.
type CheckResult = Result<(), usize>;

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

/// Check that `rna` contains RNA nucleotides only. On success return [`Ok(())`](Ok). On error
/// return [`Err`] with a 0-based index of the first incorrect character.
pub fn check_rna_str(rna: &str) -> CheckResult {
    check(rna.chars(), RNA_NUCLEOTIDES)
}

/// Check that `rna` contains RNA nucleotides only. On success return [`Ok(())`](Ok). On error
/// return [`Err`] with a 0-based index of the first incorrect character.
pub fn check_rna_chars(rna: &[char]) -> CheckResult {
    check(rna.iter().cloned(), RNA_NUCLEOTIDES)
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
    fn check_dna_rna_valid() {
        assert!(super::check_dna("GCTA").is_ok());
        assert!(super::check_rna_str("CGAU").is_ok());
    }

    #[test]
    fn check_dna_rna_invalid() {
        assert_eq!(super::check_dna("CU"), Err(1));
        assert_eq!(super::check_rna_str("CT"), Err(1));
    }

    #[test]
    fn dna_to_rna() {
        assert_eq!(super::dna_to_rna('G'), 'C');
        assert_eq!(super::dna_to_rna('C'), 'G');
        assert_eq!(super::dna_to_rna('T'), 'A');
        assert_eq!(super::dna_to_rna('A'), 'U');
    }

    #[test]
    #[should_panic]
    fn dna_to_rna_panic_invalid() {
        super::dna_to_rna('U');
    }
}
