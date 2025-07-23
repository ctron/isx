use super::default_impl;
use alloc::{
    collections::{BTreeMap, BTreeSet, BinaryHeap},
    string::String,
    vec::Vec,
};

default_impl!(String);
default_impl!(Vec<T>);
default_impl!(BTreeSet<T>);
default_impl!(BTreeMap<K,V>);
default_impl!(BinaryHeap<T>);
