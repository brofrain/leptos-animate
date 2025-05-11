use leptos::prelude::{
    guards::{Plain, ReadGuard},
    LocalStorage,
    Read,
    Write,
};
use reactive_stores::{Store, StoreField};

pub trait Utils<T>
where
    T: 'static,
    Self: StoreField<Value = T>
        + Write<Value = T>
        + Copy
        + Read<Value = ReadGuard<T, Plain<T>>>,
{
}

impl<T> Utils<T> for Store<T> where T: Send + Sync + 'static {}
impl<T> Utils<T> for Store<T, LocalStorage> where T: 'static {}
