//! The game library

#![allow(clippy::type_complexity)]

pub mod prelude {
    pub use crate::camera::*;
    pub use crate::core::*;
    pub use crate::hex::*;
}

#[cfg(feature = "debug")]
mod debug;

pub(crate) mod light;
pub(crate) mod render;

pub mod camera;
pub mod core;
pub mod hex;
