use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet};

/// Check if a value is considered "empty".
///
/// A value is typically "empty" when it is a wrapper around something, but doesn't hold an actual
/// value.
pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

macro_rules! default_impl {
    ($n:ident<$t:ident>) => {
        impl<$t> IsEmpty for $n<$t> {
            fn is_empty(&self) -> bool {
                self.is_empty()
            }
        }
    };
    ($n:ident<$($t:ident),+>) => {
        impl<$($t),+> IsEmpty for $n<$($t),+> {
            fn is_empty(&self) -> bool {
                self.is_empty()
            }
        }
    };
}

impl<T> IsEmpty for Option<T> {
    fn is_empty(&self) -> bool {
        self.is_none()
    }
}

default_impl!(Vec<T>);
default_impl!(HashSet<T>);
default_impl!(HashMap<K,V>);
default_impl!(BTreeSet<T>);
default_impl!(BTreeMap<K,V>);
default_impl!(BinaryHeap<T>);
