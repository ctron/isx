//! Check if a value is the default.
//!
//! ```rust
//! use isx::prelude::*;
//!
//! fn test () {
//!   assert!(false.is_default());
//!   assert!(true.is_not_default());
//! }
//! ```

use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet};

/// Check if the value represents the default value.
pub trait IsDefault: Default {
    /// Return `true` if the current value is considered the default.
    fn is_default(&self) -> bool;

    /// Return `false` if the current value is considered the default.
    fn is_not_default(&self) -> bool {
        !self.is_default()
    }
}

macro_rules! default_impl {
    ($n:ty) => {
        impl IsDefault for $n {
            fn is_default(&self) -> bool {
                self == &<$n>::default()
            }
        }
    };
    ($n:ty, $v:expr) => {
        impl IsDefault for $n {
            fn is_default(&self) -> bool {
                self == &$v
            }
        }
    };
}

macro_rules! empty_impl {
    ($n:ident<$($t:ident),+>) => {
        impl <$($t),+> IsDefault for $n<$($t),+> {
            fn is_default(&self) -> bool {
                self.is_empty()
            }
        }
    };
}

default_impl!(bool, false);
default_impl!(usize, 0usize);
default_impl!(isize, 0isize);
default_impl!(u8, 0u8);
default_impl!(u16, 0u16);
default_impl!(u32, 0u32);
default_impl!(u64, 0u64);
default_impl!(u128, 0u128);
default_impl!(i8, 0i8);
default_impl!(i16, 0i16);
default_impl!(i32, 0i32);
default_impl!(i64, 0i64);
default_impl!(i128, 0i128);

default_impl!(std::path::PathBuf);

impl IsDefault for String {
    fn is_default(&self) -> bool {
        self.is_empty()
    }
}

impl IsDefault for &str {
    fn is_default(&self) -> bool {
        self.is_empty()
    }
}

impl IsDefault for () {
    fn is_default(&self) -> bool {
        true
    }
}

empty_impl!(Vec<T>);
empty_impl!(BTreeMap<K,V>);
empty_impl!(BTreeSet<T>);
empty_impl!(HashMap<K,V>);
empty_impl!(HashSet<T>);

impl<T: Ord> IsDefault for BinaryHeap<T> {
    fn is_default(&self) -> bool {
        self.is_empty()
    }
}
