#![no_std]

use core::fmt::{self, Debug, Formatter};
use utils::{checks, DnaTrait, OurResult, RnaTrait};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Dna<'a>(&'a str);

#[derive(Clone, Copy)]
pub enum Rna<'a> {
    GivenNucleotides(&'a str),
    DnaBased(&'a str),
}

impl<'a> DnaTrait<'a, Rna<'a>> for Dna<'a> {
    fn new(dna: &'a str) -> OurResult<Self> {
        checks::check_dna(dna)?;
        Ok(Self(dna))
    }

    fn into_rna(&self) -> Rna<'a> {
        match self {
            Dna(dna) => Rna::DnaBased(dna),
        }
    }
}

impl<'a> RnaTrait<'a> for Rna<'a> {
    fn new(rna: &'a str) -> OurResult<Self> {
        checks::check_rna_str(rna)?;
        Ok(Self::GivenNucleotides(rna))
    }
}

impl<'a> Rna<'a> {
    /// - Generic param P - type of the parameter to pass to the closure.
    /// - Generic param P - result type from the closure, to be returned from this
    /// `with_chars_universal`.
    /// - Generic param C - closure to call, with a (dynamic) iterator over chars from `self`, and
    ///   with the given `param`.
    /// - Return: Result of the call to `closure`.
    fn with_chars_reentrant<P, C, R>(&self, param: P, closure: C) -> R
    where
        C: Fn(&mut dyn Iterator<Item = char>, P) -> R,
    {
        match self {
            Rna::GivenNucleotides(rna) => closure(&mut rna.chars(), param),
            Rna::DnaBased(dna) => closure(&mut dna.chars().map(utils::dna_to_rna), param),
        }
    }
}

impl<'a> PartialEq for Rna<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.with_chars_reentrant(
            other,
            |self_chars/*: &mut dyn Iterator<Item = char>*/, other/*: &Self*/| {
                other.with_chars_reentrant(
                    self_chars,
                    |other_chars/*: &mut dyn Iterator<Item = char>*/,
                     self_chars/*: &mut dyn Iterator<Item = char>*/| {
                        other_chars.eq(self_chars)
                    },
                )
            },
        )
    }
}
impl<'a> Eq for Rna<'a> {}

impl<'a> Debug for Rna<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Rna(\"")?;
        match self {
            Rna::GivenNucleotides(rna) => {
                write!(f, "{rna}")?;
            }
            Rna::DnaBased(dna) => {
                // Compared to ../../no_std-no_heap-slices-iterator (TODO <-- fix name) here we
                // don't have self.iter(). So we map dna to rna here:
                dna.chars()
                    .map(utils::dna_to_rna)
                    .try_for_each(|c| write!(f, "{c}"))?;
            }
        }
        write!(f, "\")")
    }
}
