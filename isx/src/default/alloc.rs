use super::empty_impl;
use crate::IsDefault;
use alloc::{
    collections::{BTreeMap, BTreeSet, BinaryHeap},
    string::String,
    vec::Vec,
};

empty_impl!(Vec<T>);
empty_impl!(BTreeMap<K,V>);
empty_impl!(BTreeSet<T>);

impl IsDefault for String {
    fn is_default(&self) -> bool {
        self.is_empty()
    }
}

impl<T: Ord> IsDefault for BinaryHeap<T> {
    fn is_default(&self) -> bool {
        self.is_empty()
    }
}
