//! An implementation requiring `std` library
#[derive(Debug, PartialEq)]
pub struct Dna(String);

#[derive(Debug, PartialEq)]
pub struct Rna(String);

impl Dna {
    /** On error return Err with a 0-based index of the first incorrect character. */
    pub fn new(dna: &str) -> Result<Self, usize> {
        match shared::check_dna(dna) {
            Ok(()) => Ok(Self(dna.to_owned())),
            Err(i) => Err(i),
        }
    }

    pub fn into_rna(self) -> Rna {
        match self {
            Dna(dna) => {
                let rna_chars = dna.chars().map(shared::dna_to_rna).collect();
                Rna(rna_chars)
            }
        }
    }
}

impl<'a> Rna {
    /** On error return Err with a 0-based index of the first incorrect character. */
    pub fn new(rna: &str) -> Result<Self, usize> {
        match shared::check_rna(rna) {
            Ok(()) => Ok(Self(rna.to_owned())),
            Err(i) => Err(i),
        }
    }
}
