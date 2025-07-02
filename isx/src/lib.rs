pub mod default;
pub mod empty;

pub mod prelude {
    pub use crate::default::*;
    pub use crate::empty::*;
}

pub use crate::default::IsDefault;
pub use crate::empty::IsEmpty;
