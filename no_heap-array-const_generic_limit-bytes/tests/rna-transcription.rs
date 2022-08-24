use no_heap_array_const_generic_limit_bytes as dna;

#[test]
fn test_valid_dna_input() {
    // const generic parameter N doesn't matter here
    assert!(<dna::Dna<0>>::new("GCTA").is_ok());
}

#[test]
fn test_valid_rna_input() {
    assert!(<dna::Rna<4>>::new("CGAU").is_ok());
}

#[test]
fn test_invalid_dna_input() {
    // Invalid character
    assert_eq!(<dna::Dna<0>>::new("X").err(), Some(0));
    // Valid nucleotide, but invalid in context
    assert_eq!(<dna::Dna<0>>::new("U").err(), Some(0));
    // Longer string with contained errors
    assert_eq!(<dna::Dna<0>>::new("ACGTUXXCTTAA").err(), Some(4));
}

#[test]
fn test_invalid_rna_input() {
    // Invalid character
    assert_eq!(<dna::Rna<1>>::new("X").unwrap_err(), 0);
    // Valid nucleotide, but invalid in context
    assert_eq!(<dna::Rna<1>>::new("T").unwrap_err(), 0);
    // Longer string with contained errors
    assert_eq!(<dna::Rna<12>>::new("ACGUTTXCUUAA").unwrap_err(), 4);
}

#[test]
fn test_acid_equals_acid() {
    assert_eq!(
        <dna::Dna<50>>::new("CGA").unwrap(),
        <dna::Dna<80>>::new("CGA").unwrap()
    );
    assert_ne!(
        <dna::Dna<10>>::new("CGA").unwrap(),
        <dna::Dna<20>>::new("AGC").unwrap()
    );
    assert_eq!(
        <dna::Rna<3>>::new("CGA").unwrap(),
        <dna::Rna<50>>::new("CGA").unwrap()
    );
    assert_ne!(
        <dna::Rna<3>>::new("CGA").unwrap(),
        <dna::Rna<7>>::new("AGC").unwrap()
    );
}

#[test]
fn test_transcribes_cytosine_guanine() {
    assert_eq!(
        <dna::Rna<1>>::new("G").unwrap(),
        <dna::Dna<9>>::new("C").unwrap().into_rna()
    );
}

#[test]
fn test_transcribes_guanine_cytosine() {
    assert_eq!(
        <dna::Rna<20>>::new("C").unwrap(),
        <dna::Dna<10>>::new("G").unwrap().into_rna()
    );
}

#[test]
fn test_transcribes_adenine_uracil() {
    assert_eq!(
        <dna::Rna<10>>::new("U").unwrap(),
        <dna::Dna<20>>::new("A").unwrap().into_rna()
    );
}

#[test]
fn test_transcribes_thymine_to_adenine() {
    assert_eq!(
        <dna::Rna<5>>::new("A").unwrap(),
        <dna::Dna<8>>::new("T").unwrap().into_rna()
    );
}

#[test]
fn test_transcribes_all_dna_to_rna() {
    assert_eq!(
        <dna::Rna<12>>::new("UGCACCAGAAUU").unwrap(),
        <dna::Dna<20>>::new("ACGTGGTCTTAA").unwrap().into_rna()
    )
}
