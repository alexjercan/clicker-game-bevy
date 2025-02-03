//! The game library

#![allow(clippy::type_complexity)]

pub mod prelude {
    pub use crate::camera::*;
    pub use crate::core::*;
    pub use crate::hex::*;
    pub use crate::levelxp::*;
    pub use crate::skilltree::*;
}

#[cfg(feature = "debug")]
mod debug;

pub(crate) mod meth;

pub mod camera;
pub mod core;
pub mod hex;
pub mod levelxp;
pub mod skilltree;
