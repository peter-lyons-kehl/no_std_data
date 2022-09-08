use no_heap_bytes_ref_mix_slice_pass_in_storage_mut_initial_macro as dna;
use dna::into_rna;
// @TODO
use no_heap_bytes_ref_mix_slice_pass_in_storage_mut_initial_macro::{Dna, Rna};

#[test]
fn test_valid_dna_input() {
    assert!(dna::Dna::new("GCTA").is_ok());
}

#[test]
fn test_valid_rna_input() {
    assert!(dna::Rna::new("CGAU").is_ok());
}

#[test]
fn test_invalid_dna_input() {
    // Invalid character
    assert_eq!(dna::Dna::new("X").err(), Some(0));
    // Valid nucleotide, but invalid in context
    assert_eq!(dna::Dna::new("U").err(), Some(0));
    // Longer string with contained errors
    assert_eq!(dna::Dna::new("ACGTUXXCTTAA").err(), Some(4));
}

#[test]
fn test_invalid_rna_input() {
    // Invalid character
    assert_eq!(dna::Rna::new("X").unwrap_err(), 0);
    // Valid nucleotide, but invalid in context
    assert_eq!(dna::Rna::new("T").unwrap_err(), 0);
    // Longer string with contained errors
    assert_eq!(dna::Rna::new("ACGUTTXCUUAA").unwrap_err(), 4);
}

#[test]
fn test_acid_equals_acid() {
    assert_eq!(dna::Dna::new("CGA").unwrap(), dna::Dna::new("CGA").unwrap());
    assert_ne!(dna::Dna::new("CGA").unwrap(), dna::Dna::new("AGC").unwrap());
    assert_eq!(dna::Rna::new("CGA").unwrap(), dna::Rna::new("CGA").unwrap());
    assert_ne!(dna::Rna::new("CGA").unwrap(), dna::Rna::new("AGC").unwrap());
}

#[test]
fn test_transcribes_cytosine_guanine() {
    let mut storage = [0u8; 1];
    assert_eq!(
        dna::Rna::new("G").unwrap(),
        into_rna!(&dna::Dna::new("C").unwrap(), storage)
    );
}

#[test]
fn test_transcribes_cytosine_guanine_storage_can_be_shared() {
    let mut storage = [0u8; 1];
    // We can't pass &mut here. See test_transcribes_cytosine_guanine_storage_can_be_shared_expanded
    // for the reason.
    let rna = into_rna!(&dna::Dna::new("C").unwrap(), storage);
    assert_eq!(storage.len(), 1);
    assert_eq!(dna::Rna::new("G").unwrap(), rna);
}

#[test]
fn test_transcribes_cytosine_guanine_storage_can_be_shared_expanded() {
    let mut storage = [0u8; 1];

    let len = dna::Dna::new("C")
        .unwrap()
        .prepare_storage_from_dna(&mut storage);
    // Can't have the (unnecessary) `mut` in the following, because we couldn't borrow it as shared
    // later - even though `Rna::from_prepared_storage` uses the given slice as shared only.
    //
    // let rna = Rna::from_prepared_storage(&mut storage, len);
    let rna = Rna::from_prepared_storage(&storage, len);

    assert_eq!(storage.len(), 1);
    assert_eq!(dna::Rna::new("G").unwrap(), rna);
}

#[test]
fn test_transcribes_guanine_cytosine() {
    let mut storage = [0u8; 1];
    assert_eq!(
        dna::Rna::new("C").unwrap(),
        into_rna!(&dna::Dna::new("G").unwrap(), storage)
    );
}

#[test]
fn test_transcribes_adenine_uracil() {
    let mut storage = [0u8; 4];
    assert_eq!(
        dna::Rna::new("U").unwrap(),
        into_rna!(&dna::Dna::new("A").unwrap(), storage)
    );
}

#[test]
fn test_transcribes_thymine_to_adenine() {
    let mut storage = [0u8; 1];
    assert_eq!(
        dna::Rna::new("A").unwrap(),
        into_rna!(&dna::Dna::new("T").unwrap(), storage)
    );
}

#[test]
fn test_transcribes_all_dna_to_rna() {
    let mut storage = [0u8; 12];
    assert_eq!(
        dna::Rna::new("UGCACCAGAAUU").unwrap(),
        into_rna!(&dna::Dna::new("ACGTGGTCTTAA").unwrap(), storage)
    )
}
