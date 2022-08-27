//use core::fmt::Debug;
use crate::{DnaTrait, OurResult, RnaTrait};

pub trait RnaTraitMut<'a>: RnaTrait<'a> {
    /// Mutate `self`: Make it store all characters in the given `iter`. Fail if `iter` doesn't
    /// satisfy requirements particular of the given implementation.
    fn set_from_iter(&mut self, iter: &mut dyn Iterator<Item = char>) -> OurResult<()>;
}

pub trait RnaTraitMutLeakStorage<'a>: RnaTraitMut<'a> {
    /// Invoke the given call-back function `f` with an iterator over `self`'s whole storage
    /// (including any unused data; mapped to chars if needed).
    ///
    /// This uses an [`Iterator`] of `char`, even if the backing storage is in bytes. That allows
    /// us to re-use the tests. (Yes, that does increase a potential of a bug in the crate being
    /// tested or in the test harness.)
    ///
    /// An ideal signature for this function would be: `fn leak_whole_storage() -> impl
    /// Iterator<Item = char>` but that's not possible for trait functions. Hence working around
    /// with a double dynamic dispatch.
    ///
    /// Available & used only when testing.
    #[cfg(test)]
    fn with_whole_storage_leaked<R>(&self, f: &dyn Fn(&mut dyn Iterator<Item = char>) -> R) -> R;
}

pub trait Tests {
    type Dna<'a>: DnaTrait<'a, Self::Rna<'a>>;
    type Rna<'a>: RnaTraitMut<'a> + 'a;

    fn test_modify_string_based_rna() -> OurResult<()> {
        let mut rna_one = Self::Rna::new("CGAU")?;
        let nucleotides = "UAGC";
        let mut nucleotides_iter = nucleotides.chars();
        rna_one.set_from_iter(&mut nucleotides_iter)?;

        let rna_two = Self::Rna::new("UAGC")?;
        assert_eq!(rna_one, rna_two);

        Ok(())
    }

    fn all_tests() -> OurResult<()> {
        Self::test_modify_string_based_rna()?;
        Ok(())
    }
}
