//! Used by implementations *_wipe_on_mut.

extern crate alloc;

use crate::api_tests_mut::RnaTraitMutLeakStorage;

/// A marker trait. See [`Tests`].
pub trait RnaTraitMutWipeOnMut<'a>: RnaTraitMutLeakStorage<'a> {}
