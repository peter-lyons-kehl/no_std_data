//! Used by implementations *_wipe_on_clone and *_wipe_on_drop.

use crate::api_tests_mut;
use crate::DnaTrait;
use api_tests_mut::{RnaTraitMutLeakStorage, WithStorageLeaked};

/// A marker trait. See [`Tests`] and [`Leave`].
pub trait RnaTraitMutWipeOnLeave<'a>: RnaTraitMutLeakStorage<'a> {}

/// Type (signature) of user's implementation's call back function that is passed by the user to
/// `test_` functions from [`Tests`] trait, so that `test_` functions here can detect leakage.
///
/// This simulates/invokes operation(s) that would result in an instance leaking data.
///
/// For mainstream implementations this should call either [`Clone::clone`] or [`Drop::drop`],
/// depending on the implementation. If cloned, it returns the new instance. If dropped, it returns
/// `self`.
///
/// Of course,[`Clone::clone`] doesn't consume `self` (but takes it by a shared reference `&self`).
/// However, that difference doesn't matter for our testing. Treating `.clone()` and `.drop()`
/// similarly allows us to re-use this trait for testing both [`Clone`] and [`Drop`].
///
/// Such a use of the instance after [`Drop::drop`] doesn't follow [`Drop`] contract, but that's OK
/// for our testing.
#[allow(type_alias_bounds)]
pub type Leave<'a, RNA: RnaTraitMutWipeOnLeave<'a>> = &'a dyn Fn(RNA) -> RNA;

pub trait Tests {
    type Dna<'a>: DnaTrait<'a, Self::Rna<'a>>;
    type Rna<'a>: RnaTraitMutWipeOnLeave<'a> + 'a;

    /// An optional test - a [`DnaTrait`] implementation may not leak at all before "leave". But if
    /// it does (that is, if this test does pass), then it has to satisfy
    /// [`test_modify_string_based_rna_leave_does_not_leak`].
    ///
    /// Not to be re-implemented, but to be run as-is from user's tests.
    fn test_modify_string_based_rna_direct_without_leave_does_leak<'a>(
        with_storage_leaked: WithStorageLeaked<'a, Self::Rna<'a>, bool>,
    ) {
        let rna = api_tests_mut::cga_modified_to_u::<Self::Rna<'_>>();

        let leaks = api_tests_mut::leaks_g_or_a(&rna, with_storage_leaked);

        assert!(leaks);
    }

    /// A required test.
    ///
    /// Not to be re-implemented, but to be run as-is from user's tests.
    fn test_modify_string_based_rna_leave_does_not_leak<'a>(
        leave: Leave<'a, Self::Rna<'a>>,
        with_storage_leaked: WithStorageLeaked<'a, Self::Rna<'a>, bool>,
    ) {
        let rna = api_tests_mut::cga_modified_to_u::<Self::Rna<'_>>();

        let rna = leave(rna);
        let leaks = api_tests_mut::leaks_g_or_a(&rna, with_storage_leaked);
        assert!(!leaks);
    }
}
