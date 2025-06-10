//! oxels
//!
//! Image Analysis in Rust.

pub mod io;
pub mod image;

pub use crate::io::meta_image::{load_meta_image, save_meta_image};
pub use crate::image::{Image, AnyImage};