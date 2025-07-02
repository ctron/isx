//! Check if a value is empty
//!
//! ```rust
//! use std::collections::HashMap;
//! use isx::IsEmpty;
//!
//! fn assert<V: IsEmpty>(v: V) {
//!   assert!(v.is_empty());
//! }
//!
//! fn test() {
//!   assert(String::default());
//!   assert([0u8;0]);
//!   assert(Vec::<String>::new());
//!   assert(HashMap::<String,String>::new())
//! }
//! ```
//!
//! You can also implement [`IsEmpty`] for your own type using a derive:
//!
//! ```rust
//! use isx_macros::IsEmpty;
//!
//! #[derive(IsEmpty)]
//! struct MyStruct {
//!     foo: String,
//!     bar: Vec<String>,
//! }
//! ```
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet};

pub use isx_macros::IsEmpty;

/// Check if a value is considered "empty".
///
/// A value is typically "empty" when it is a wrapper around something, but doesn't hold an actual
/// value.
///
/// ## Default implementations
///
/// There are a bunch of default implementations for common types. For all types which already have
/// an `is_empty` function, this function will be called. Notable exceptions are:
///
///   * `Option<T>`: If the option is `None`, it is considered empty.
///   * `()`: Is always considered empty.
///
/// ## Derive
///
/// This trait can be implemented using a derive. In this case `is_empty` will return `true` if:
///
/// * There are no fields in the struct or enum
/// * All fields implement `IsEmpty` and return `true` when calling `IsEmpty::is_empty`.
pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

macro_rules! default_impl {
    ($n:ident) => {
        impl IsEmpty for $n {
            fn is_empty(&self) -> bool {
                self.is_empty()
            }
        }
    };
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

impl<T> IsEmpty for [T] {
    fn is_empty(&self) -> bool {
        <[T]>::is_empty(self)
    }
}

impl<const N: usize, T> IsEmpty for [T; N] {
    fn is_empty(&self) -> bool {
        <[T]>::is_empty(self)
    }
}

impl IsEmpty for () {
    fn is_empty(&self) -> bool {
        true
    }
}

default_impl!(BTreeSet<T>);
default_impl!(BTreeMap<K,V>);
default_impl!(BinaryHeap<T>);
default_impl!(HashSet<T>);
default_impl!(HashMap<K,V>);
default_impl!(String);
default_impl!(Vec<T>);
