//! Provide some traits (and implementations) for types in the spirit of `is_*` (like `is_empty`)
//!
//! The idea is to provide standard traits for types to e.g. check "if something is empty"? In
//! some cases, one might just be interested if something is empty. However, Rust currently doesn't
//! offer a trait like `IsEmpty`, although a lot of types provide a `is_empty(&self) -> bool`
//! function. This crate tries to fix that.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod default;
pub mod empty;

/// The prelude, including everything necessary
pub mod prelude {
    pub use crate::default::*;
    pub use crate::empty::*;
}

pub use crate::default::IsDefault;
pub use crate::empty::IsEmpty;
