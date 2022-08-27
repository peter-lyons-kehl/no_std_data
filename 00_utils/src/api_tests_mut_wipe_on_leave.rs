//! Used by implementations *_wipe_on_clone and *_wipe_on_drop.

// /use core::fmt::Debug;
use crate::{api_tests_mut::RnaTraitMutLeakStorage, DnaTrait};

///
pub trait RnaTraitMutWipeOnLeave<'a>: RnaTraitMutLeakStorage<'a> {
    /// This calls either `.clone()` or `.drop()`, depending on the implementation. If cloned, it
    /// returns the new instance. If dropped, it returns `self`.
    ///
    /// Of course,[`Clone::clone`] doesn't consume `self` (but takes it by a shared reference
    /// `&self`). However, that difference doesn't matter for our testing. Treating `.clone()` and
    /// `.drop()` similarly allows us to re-use this trait for testing both [`Clone`] and [`Drop`].
    ///
    /// This use of the instance after `.drop()` doesn't follow [`Drop`] contract, but that's OK for
    /// our testing.
    ///
    /// Available & used only when testing.
    #[cfg(test)]
    fn leave(self) -> Self;
}

pub trait Tests {
    type Dna<'a>: DnaTrait<'a, Self::Rna<'a>>;
    type Rna<'a>: RnaTraitMutWipeOnLeave<'a> + 'a;

    fn test_valid_self_input() {}

    fn all_tests() {}
}
