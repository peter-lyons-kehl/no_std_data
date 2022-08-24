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
        <dna::Dna<3>>::new("CGA").unwrap(),
        dna::Dna::new("CGA").unwrap()
    );
    assert_ne!(
        <dna::Dna<3>>::new("CGA").unwrap(),
        dna::Dna::new("AGC").unwrap()
    );
    assert_eq!(
        <dna::Rna<3>>::new("CGA").unwrap(),
        <dna::Rna<5>>::new("CGA").unwrap()
    );
    assert_ne!(
        <dna::Rna<3>>::new("CGA").unwrap(),
        dna::Rna::new("AGC").unwrap()
    );
}

#[test]
fn test_transcribes_cytosine_guanine() {
    assert_eq!(
        <dna::Rna<1>>::new("G").unwrap(),
        dna::Dna::new("C").unwrap().into_rna()
    );
}

#[test]
fn test_transcribes_guanine_cytosine() {
    assert_eq!(
        <dna::Rna<1>>::new("C").unwrap(),
        dna::Dna::new("G").unwrap().into_rna()
    );
}

#[test]
fn test_transcribes_adenine_uracil() {
    assert_eq!(
        <dna::Rna<1>>::new("U").unwrap(),
        dna::Dna::new("A").unwrap().into_rna()
    );
}

#[test]
fn test_transcribes_thymine_to_adenine() {
    assert_eq!(
        <dna::Rna<1>>::new("A").unwrap(),
        dna::Dna::new("T").unwrap().into_rna()
    );
}

#[test]
fn test_transcribes_all_dna_to_rna() {
    assert_eq!(
        <dna::Rna<12>>::new("UGCACCAGAAUU").unwrap(),
        dna::Dna::new("ACGTGGTCTTAA").unwrap().into_rna()
    )
}
