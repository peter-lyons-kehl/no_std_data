extern crate alloc;

use crate::RnaTraitMut;
use alloc::vec::Vec;

/// A marker trait. See [`Tests`] and [`Leave`].
pub trait RnaTraitMutLeakStorage<'a>: RnaTraitMut<'a> {}

/// Type (signature) of a call back function that [`Tests`] trait passes to the user-provided
/// function that has signature [`WithStorageLeaked`]. [`Tests`] does that in  its `test_`
/// functions.
#[allow(type_alias_bounds)]
pub type WithStorageLeakedCallBack<'a, RES: 'a> = &'a dyn Fn(&mut dyn Iterator<Item = u8>) -> RES;

/// Type (signature) of user's implementation's call back function that is passed by the user to
/// `test_` functions from [`Tests`] trait, so that `test_` functions here can detect leakage.
///
/// This exposes `self`'s whole storage (including any unused data; mapped to bytes if needed).
#[allow(type_alias_bounds)]
pub type WithStorageLeaked<'a, RNA: RnaTraitMutLeakStorage<'a>, RES> =
    &'a dyn Fn(&RNA, WithStorageLeakedCallBack<'a, RES>) -> RES;

/// A helper.
fn cga_modified_to_u<'a, R: RnaTraitMut<'a>>() -> R {
    let mut rna = R::new("CGA").expect("RNA");
    rna.set_from_iter(&mut "U".chars()).expect("success");
    rna
}

/// A helper.
fn leaks_g_or_a<'a, R: RnaTraitMutLeakStorage<'a>>(
    rna: &R,
    with_storage_leaked: WithStorageLeaked<'a, R, bool>,
) -> bool {
    #[allow(clippy::char_lit_as_u8)]
    with_storage_leaked(rna, &|bytes_iter| {
        let bytes = bytes_iter.collect::<Vec<_>>();
        bytes[1] == 'G' as u8 || bytes[2] == 'A' as u8
    })
}
