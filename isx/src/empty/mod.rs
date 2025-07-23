//! Check if a value is empty
//!
//! ```rust
//! use isx::IsEmpty;
//!
//! fn assert<V: IsEmpty>(v: V) {
//!   assert!(v.is_empty());
//! }
//!
//! fn test() {
//!   assert("");
//!   assert([0u8;0]);
//! }
//! ```

#![cfg_attr(
    feature = "std",
    doc = r#"
```rust
use std::collections::HashMap;
use isx::IsEmpty;

fn assert<V: IsEmpty>(v: V) {
  assert!(v.is_empty());
}

fn test() {
  assert(String::default());
  assert(Vec::<String>::new());
  assert(HashMap::<String,String>::new())
}
```

You can also implement [`IsEmpty`] for your own type using a derive:

```rust
use isx_macros::IsEmpty;

#[derive(IsEmpty)]
struct MyStruct {
    foo: String,
    bar: Vec<String>,
}
```
"#
)]

#[cfg(feature = "alloc")]
mod alloc;
#[cfg(feature = "bytes")]
mod bytes;
#[cfg(feature = "std")]
mod std;

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
/// This trait can be implemented using a derive. In this case, `is_empty` will return `true` if:
///
/// * There are no fields in the struct or enum
/// * All fields implement `IsEmpty` and return `true` when calling `IsEmpty::is_empty`.
pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

#[allow(unused_macros)]
macro_rules! default_impl {
    ($n:ident) => {
        impl $crate::IsEmpty for $n {
            fn is_empty(&self) -> bool {
                <$n>::is_empty(self)
            }
        }
    };
    ($n:ident<$t:ident>) => {
        impl<$t> $crate::IsEmpty for $n<$t> {
            fn is_empty(&self) -> bool {
                <$n<$t>>::is_empty(self)
            }
        }
    };
    ($n:ident<$($t:ident),+>) => {
        impl<$($t),+> $crate::IsEmpty for $n<$($t),+> {
            fn is_empty(&self) -> bool {
                <$n<$($t),+>>::is_empty(self)
            }
        }
    };
}

#[allow(unused_imports)]
pub(crate) use default_impl;

impl IsEmpty for &str {
    fn is_empty(&self) -> bool {
        str::is_empty(self)
    }
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
